#![feature(plugin)]
#![plugin(peg_syntax_ext)]
pub use param::*;

#[derive(Debug)]
pub struct Equation{
    left: Operand,
    right: Operand
}

#[derive(Debug)]
pub struct Function{
    pub function:String,
    pub params:Vec<Operand>,
}

#[derive(Debug)]
pub enum Operand{
    Column(String),
    Function(Function),
    Value(String),
    Vec(Vec<Operand>),
}

#[derive(Debug)]
pub enum Connector{
    AND,
    OR,
}
#[derive(Debug)]
pub enum Direction{
    ASC,
    DESC,
}

#[derive(Debug)]
pub struct Order{
    column: String,
    direction: Direction,
}

#[derive(Debug)]
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
pub struct Condition{
    pub left:Operand,
    pub equality:Equality,
    pub right:Operand,
}

#[derive(Debug)]
struct Filter{
    connector: Option<Connector>,
    condition: Condition,
    subfilter: Vec<Filter>,
}

#[derive(Debug)]
pub struct Params{
    equations: Vec<Equation>,
    orders: Vec<Order>,
    filters: Vec<Filter>,
    conditions: Vec<Condition>,
}


peg! param(r#"
use super::*;


#[pub]
name -> String
  = [a-zA-Z0-9_]+ { match_str.to_string() }
#[pub]
equation -> Equation
    = l:operand "=" r:operand { Equation{left:l, right:r} }
#[pub]
operand -> Operand
	= c: name { Operand::Column(c) }

#[pub]
function -> Function
	= f:name "(" p: operand ")" { Function {function: f, params: vec![p]}}
	
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
	= l: operand "=" eq: equality "." r: operand {
		Condition{left: l, equality: eq, right: r}
	}
"#);

pub fn parse_param_condition(str: &str)->Result<Condition, param::ParseError>{
    condition(str)
}

fn main() {
	println!("{:?}",equation("lee=cesar"));
	println!("{:?}",operand("description"));
	println!("{:?}",function("sum(total)"));
	println!("eq: {:?}",equality("eq"));
	println!("neq: {:?}",equality("neq"));
	println!("lt: {:?}",equality("lt"));
	println!("lte: {:?}",equality("lte"));	
	println!("lteee: {:?}",equality("lteee"));	
	println!("gt: {:?}",equality("gt"));	
	println!("{:?}",equality("gte"));	
	println!("in: {:?}",equality("in"));	
	println!("not_in: {:?}",equality("not_in"));	
	println!("is_not: {:?}",equality("is_not"));	
	println!("like:{:?}",equality("like"));	
	println!("condition:{:?}",condition("age=lt.13"));	
}