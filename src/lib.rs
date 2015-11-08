#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub use self::param::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Equation{
    pub left: Operand,
    pub right: Operand
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Function{
    pub function:String,
    pub params:Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Operand{
    Column(String),
    Function(Function),
    Value(String),
    Vec(Vec<Operand>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Connector{
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction{
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Order{
    pub column: String,
    pub direction: Direction,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Equality{
    EQ, // = ,
    NEQ, // != ,
    LT, // <,
    LTE, // <=,
    GT, // >,
    GTE, // >=,
    IN, // IN
    NOT_IN,//NOT IN,
    IS,// IS
    IS_NOT,// IS NOT 
    LIKE, // LIKE
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Condition{
    pub left:Operand,
    pub equality:Equality,
    pub right:Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filter{
    pub connector: Option<Connector>,
    pub condition: Condition,
    pub subfilter: Vec<Filter>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Params{
    pub filters: Vec<Filter>,
    pub group_by: Vec<Operand>,
    pub having: Vec<Operand>,
    pub order_by: Vec<Order>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub equations: Vec<Equation>,
}


peg! param(r#"
use super::*;


#[pub]
name -> String
  	= [a-zA-Z0-9_]+ { match_str.to_string() }

#[pub]
column_name -> String
	= t:name "." c:name { format!("{}.{}", t,c) } 
	/ c:name { format!("{}", c) }

#[pub]
equation -> Equation
    = l:operand "=" r:operand { Equation{left:l, right:r} }

#[pub]
operand -> Operand
	= f:function { Operand::Function(f) }
	/ c:column_name { Operand::Column(c) }

#[pub]
function -> Function
	= f:name "(" p:operand ")" { Function {function: f, params: vec![p]}}
	
#[pub]
equality -> Equality
	= "eq"     { Equality::EQ }
	/ "neq"    { Equality::NEQ }
	/ "lt" e:"e"?     { 
			match e { 
				None => Equality::LT,
				Some(e) => Equality::LTE, 
			} 
	}
	/ "gt" e:"e"?     { 
			match e { 
				None => Equality::GT,
				Some(e) => Equality::GTE, 
			} 
	}
    / "in"     { Equality::IN }
    / "not_in" { Equality::NOT_IN }
    / "is" _not:"_not"?     { 
			match _not { 
				None => Equality::IS,
				Some(e) => Equality::IS_NOT, 
			} 
	}
    / "like"   { Equality::LIKE }

#[pub]
condition -> Condition
	= l:operand "=" eq:equality "." r:operand {
		Condition{left: l, equality: eq, right: r}
	}

#[pub]
direction -> Direction
	= "asc" { Direction::ASC }
	/ "desc" { Direction::DESC }

#[pub]
order -> Order
	= c:name "." d:direction { Order{ column: c, direction: d} }

#[pub]
order_by -> Vec<Order>
	= "order_by" "=" o:order++ "," {o}

#[pub]
group_by -> Vec<Operand>
	= "group_by" "=" fields:operand++ "," {
		fields
	}

#[pub]
connector -> Connector
	= "&" { Connector::AND }
	/ "|" { Connector::OR }

#[pub]
filter -> Filter
    = c: condition conn: connector f: filter {
    	let rf = Filter{
    		connector:Some(conn),
    		condition: f.condition,
    		subfilter: f.subfilter
    	};
		Filter{
    		connector: None,
    		condition: c,
    		subfilter: vec![rf]
    	}
    }
    / c: condition{
    	Filter{
    		connector: None,
    		condition: c,
    		subfilter: vec![]
    	}
    }
    / "(" f:filter ")" { 
			f
	}

#[pub]
connector_condition -> (Connector, Condition)
	= con:connector rc:condition { (con, rc) }	

#[pub]
connector_filter -> (Connector, Filter)
	= con:connector rf:filter { (con, rf) }	

#[pub]
and_order_by -> Vec<Order>
	=  "&"? o:order_by { o }

#[pub]
and_group_by -> Vec<Operand>
	=  "&"? g:group_by { g }
#[pub]
and_equations -> Vec<Equation>
	=  "&"? e:equation ** "&" { e }

#[pub]
and_filters -> Vec<Filter>
	=  "&"? f:filter { vec![f] }

#[pub]
params -> Params
 = f:and_filters? g:and_group_by? o:and_order_by? e:and_equations? {
 	Params{ 
     		filters: match f{
     						Some(f)=> f,
     						None => vec![]
 						}, 
     		group_by: match g{
     						Some(g)=> g,
     						None => vec![]
 						}, 
 			having: vec![],
     		order_by: match o{
     						Some(o)=> o,
     						None => vec![]
 						},
 			page: None,
 			page_size: None,
     		equations: match e{
     						Some(e)=> e,
     						None => vec![]
 						}, 
     	} 
 }
"#);

