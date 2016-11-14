#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub use self::param::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Equation {
    pub left: Operand,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Operand {
    Column(String),
    Function(Function),
    Number(f64),
    Boolean(bool),
    Value(String),
}

impl Operand{
    fn value_append(self, s:&str)->Self{
        match self{
            Operand::Value(value) => {
                Operand::Value(format!("{}{}",value,s))
            }
            Operand::Column(value) => {
                Operand::Value(format!("{}{}",value,s))
            }
            _ => self
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Connector {
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Equality {
    EQ, // = ,  eq
    NEQ, // != , neq
    LT, // <,  lt
    LTE, // <=, lte
    GT, // >, gt
    GTE, // >=, gte
    IN, // IN, in
    NOT_IN, // NOT IN, not_in
    IS, // IS, is
    IS_NOT, // IS NOT, is_not
    LIKE, // LIKE, like
    ILIKE, // ILIKE case insensitive like, postgresql specific
    ST // Starts with, which will become ILIKE 'value%'
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filter {
    pub connector: Option<Connector>,
    pub condition: Condition,
    pub sub_filters: Vec<Filter>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Params {
    pub filters: Vec<Filter>,
    pub equations: Vec<Equation>,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Query {
    pub from: Vec<Operand>,
    pub join: Vec<Join>,
    pub filters: Vec<Filter>,
    pub group_by: Vec<Operand>,
    pub having: Vec<Filter>,
    pub order_by: Vec<Order>,
    pub range: Option<Range>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Limit {
    pub limit: i64,
    pub offset: Option<i64>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table: Operand,
    pub column1: Vec<String>,
    pub column2: Vec<String>,
}

peg! param(r#"
use super::*;


#[pub]
name -> String
  	= [a-zA-Z0-9_\-]+ { match_str.to_string() }

#[pub]
number -> f64
    = "-"? int frac? exp? {match_str.parse().unwrap()}

int -> i64
	= [0-9]{1,} {match_str.parse().unwrap()}

exp -> String
    = ("e" / "E") ("-" / "+")? [0-9]{1,} { match_str.to_owned() }

frac -> f64
    = "." [0-9]{1,} { match_str.parse().unwrap() }

#[pub]
boolean -> bool
	= "true" { true }
	/ "false" { false }

#[pub]
column_name -> String
	= t:name "." d:!direction c:name { format!("{}.{}", t, c) }
	/ c:name  { format!("{}", c) }

#[pub]
equation -> Equation
    = l:operand "=" r:operand { Equation{left:l, right:r} }

#[pub]
operand -> Operand
	= b:boolean { Operand::Boolean(b) }
	/ n:number { Operand::Number(n as f64) }
	/ f:function { Operand::Function(f) }
	/ c:column_name { Operand::Column(c) }
    / "\[" v:name "\]" {Operand::Value(v) }

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
    / "ilike"   { Equality::ILIKE }
    / "st"  {Equality::ST}

#[pub]
condition -> Condition
	= l:operand "=" eq:equality "." r:operand {
        if eq == Equality::ST{
		    Condition{left: l, equality: Equality::ILIKE, right: r.value_append("%")}
        }else{
		    Condition{left: l, equality: eq, right: r}
        }
	}

#[pub]
direction -> Direction
	= "asc" { Direction::ASC }
	/ "desc" { Direction::DESC }

#[pub]
nulls_where -> NullsWhere
	= "nullsfirst" { NullsWhere::FIRST }
	/ "nullslast" { NullsWhere::LAST }

#[pub]
order -> Order
	= o:operand d:("." direction)? n:("." nulls_where)?  { Order{ operand: o, direction: d, nulls_where: n} }

#[pub]
order_by -> Vec<Order>
	= "order_by" "=" o:order++ "," {o}

#[pub]
group_by -> Vec<Operand>
	= "group_by" "=" fields:operand++ "," { fields }

#[pub]
from -> Vec<Operand>
	= "from" "=" fr:operand++ "," { fr }

modifier -> Modifier
	= "left"  { Modifier::LEFT }
	/ "right" { Modifier::RIGHT }
	/ "full"  { Modifier::FULL }

join_type -> JoinType
    = "inner" { JoinType::INNER }
    / "outer" { JoinType::OUTER }
    / "cross" { JoinType::CROSS }
    / "natural" { JoinType::NATURAL }

modifier_join_type -> (Option<Modifier>, Option<JoinType>)
 = m:modifier "_" jt:join_type { ( Some(m), Some(jt) ) }
 / m:modifier  { ( Some(m), None ) }
 / jt:join_type  { ( None, Some(jt) ) }

#[pub]
join -> Join
 =  m_jt:modifier_join_type? "_"? "join" "=" t:operand on:and_on+ {
 	let mut columns1 = vec![];
 	let mut columns2 = vec![];
 	for (c1,c2) in on{
 		columns1.push(c1);
 		columns2.push(c2);
 	}
 	let (m, jt) = match m_jt{
 		Some( m_jt ) => m_jt,
 		None => (None, None)
 	};
 	Join{
 		modifier: m,
 		join_type: jt,
 		table: t,
 		column1: columns1,
 		column2: columns2
 	}
 }

and_join -> Vec<Join>
 = "&" j:join++"&" { j }

and_on -> (String, String)
 = "&on=" c1:column_name "=" c2:column_name { (c1, c2) }

#[pub]
having -> Vec<Filter>
	= "having" "=" f:filter { vec![f] }

#[pub]
page -> i64
	= "page" "=" p:int { p }

#[pub]
page_size -> i64
	= "page_size" "=" ps:int { ps }

#[pub]
limit -> i64
	= "limit" "=" l:int { l }
#[pub]
offset -> i64
	= "offset" "=" o:int { o }

#[pub]
range -> Range
	= p:(page) ps:("&" page_size) {
		Range::Page(Page{ page: p, page_size: ps})
	}
	/ l:(limit) o:("&" offset)? {
		Range::Limit(Limit{ limit: l, offset: o})
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
    		sub_filters: f.sub_filters
    	};
		Filter{
    		connector: None,
    		condition: c,
    		sub_filters: vec![rf]
    	}
    }
    / "(" f:filter ")" {
			f
	}
    / c: condition{
    	Filter{
    		connector: None,
    		condition: c,
    		sub_filters: vec![]
    	}
    }


and_equations -> Vec<Equation>
	=  "&"? e:equation ** "&" { e }

and_filters -> Vec<Filter>
	=  "&"? f:filter { vec![f] }

#[pub]
params -> Params
 = f:and_filters? e:and_equations? {
 	Params{
     		filters: match f{
     						Some(f)=> f,
     						None => vec![]
 						},
     		equations: match e{
     						Some(e)=> e,
     						None => vec![]
 						},
     	}
 }

#[pub]
query -> Query
 = fr:from? j:and_join? f:and_filters? g:("&"? group_by)? h:("&"? having)? o:("&"? order_by)? r:("&"?range)? e:and_equations? {
 	Query{
 			from: match fr{
 					Some(fr) => fr,
 					None => vec![]
 				},
 			join: match j{
     				Some(j) => j,
     				None => vec![]
     			},
     		filters: match f{
     						Some(f)=> f,
     						None => vec![]
 						},
     		group_by: match g{
     						Some(g)=> g,
     						None => vec![]
 						},
 			having: match h{
 					Some(h) => h,
 					None => vec![]
 			},
     		order_by: match o{
     						Some(o)=> o,
     						None => vec![]
 						},
 			range: r,
     		equations: match e{
     						Some(e)=> e,
     						None => vec![]
 						},
     	}
 }
"#);

#[test]
fn test_boolean_true() {
    assert_eq!(Ok(true), boolean("true"));
}
#[test]
fn test_boolean_false() {
    assert_eq!(Ok(false), boolean("false"));
}

#[test]
fn test_number() {
    assert_eq!(Ok(123f64), number("123"));
}

#[test]
fn test_name() {
    assert_eq!(Ok("age".to_owned()), name("age"));
}


#[test]
fn test_column() {
    assert_eq!(Ok(Operand::Column("age".to_owned())), operand("age"));
}




#[test]
fn test_table_column() {
    assert_eq!(Ok(Operand::Column("person.age".to_owned())),
               operand("person.age"));
}


#[test]
fn test_from() {
    assert_eq!(Ok(vec![Operand::Column("person".to_owned())]),
               from("from=person"));
}

#[test]
fn test_left_join() {
    assert_eq!(Ok(Join {
                   modifier: Some(Modifier::LEFT),
                   join_type: None,
                   table: Operand::Column("person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("left_join=person&on=person.student_id=student.id"));
}


#[test]
fn test_join() {
    assert_eq!(Ok(Join {
                   modifier: None,
                   join_type: None,
                   table: Operand::Column("bazaar.person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("join=bazaar.person&on=person.student_id=student.id"));
}


#[test]
#[should_panic]
fn test_join_without_on() {
    assert_eq!(Ok(Join {
                   modifier: None,
                   join_type: None,
                   table: Operand::Column("bazaar.person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("join=bazaar.person"));
}


#[test]
fn test_function() {
    assert_eq!(Ok(Function {
                   function: "min".to_owned(),
                   params: vec![Operand::Column("age".to_owned())],
               }),
               function("min(age)"));
}

#[test]
fn test_order() {
    assert_eq!(Ok(Order {
                   operand: Operand::Column("age".to_owned()),
                   direction: Some(Direction::DESC),
                   nulls_where: None,
               }),
               order("age.desc"));
}



#[test]
fn test_euqation() {
    assert_eq!(Ok(Equation {
                   left: Operand::Column("x".to_owned()),
                   right: Operand::Number(123f64),
               }),
               equation("x=123"));
}



#[test]
fn test_condition() {
    assert_eq!(Ok(Condition {
                   left: Operand::Column("age".to_owned()),
                   equality: Equality::EQ,
                   right: Operand::Number(13f64),
               }),
               condition("age=eq.13"));
}

#[test]
fn test_starts_with() {
    assert_eq!(Ok(Condition {
                   left: Operand::Column("name".to_owned()),
                   equality: Equality::ILIKE,
                   right: Operand::Value("le%".to_string()),
               }),
               condition("name=st.le"));
}

#[test]
fn test_filter() {
    assert_eq!(Ok(Filter {
                   connector: None,
                   condition: Condition {
                       left: Operand::Column("student".to_owned()),
                       equality: Equality::EQ,
                       right: Operand::Boolean(true),
                   },
                   sub_filters: vec![],
               }),
               filter("student=eq.true"))
}





#[test]
fn test_params() {
    assert_eq!(Ok(Params {
                   filters: vec![Filter {
                                     connector: None,
                                     condition: Condition {
                                         left: Operand::Column("age".to_owned()),
                                         equality: Equality::LT,
                                         right: Operand::Number(13f64),
                                     },
                                     sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ],
                                 }],
                   equations: vec![Equation {
                                       left: Operand::Column("x".to_owned()),
                                       right: Operand::Number(123f64),
                                   }],
               }),
               params("age=lt.13&student=eq.true|gender=eq.M&x=123"));
}



#[test]
fn test_query() {
    assert_eq!(Ok(Query {
                   filters: vec![Filter {
                                     connector: None,
                                     condition: Condition {
                                         left: Operand::Column("age".to_owned()),
                                         equality: Equality::LT,
                                         right: Operand::Number(13f64),
                                     },
                                     sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ],
                                 }],
                   order_by: vec![Order {
                                      operand: Operand::Column("age".to_owned()),
                                      direction: Some(Direction::DESC),
                                      nulls_where: None,
                                  },
                                  Order {
                                      operand: Operand::Column("height".to_owned()),
                                      direction: Some(Direction::ASC),
                                      nulls_where: None,
                                  }],
                   group_by: vec![Operand::Function(Function {
                                      function: "sum".to_owned(),
                                      params: vec![Operand::Column("age".to_owned())],
                                  }),
                                  Operand::Column("grade".to_owned()),
                                  Operand::Column("gender".to_owned())],
                   having: vec![Filter {
                                    connector: None,
                                    condition: Condition {
                                        left: Operand::Function(Function {
                                            function: "min".to_owned(),
                                            params: vec![Operand::Column("age".to_owned())],
                                        }),
                                        equality: Equality::GT,
                                        right: Operand::Number(13f64),
                                    },
                                    sub_filters: vec![],
                                }],
                   range: Some(Range::Limit(Limit {
                       limit: 100,
                       offset: Some(25),
                   })),
                   equations: vec![Equation {
                                       left: Operand::Column("x".to_owned()),
                                       right: Operand::Number(123f64),
                                   },
                                   Equation {
                                       left: Operand::Column("y".to_owned()),
                                       right: Operand::Number(456f64),
                                   }],
                   ..Default::default()
               }),
               query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,\
                      gender&having=min(age)=gt.13&order_by=age.desc,height.\
                      asc&limit=100&offset=25&x=123&y=456"));
}

// #[test]
fn test_params_with_string() {
    assert_eq!(Ok(Params {
                   filters: vec![Filter {
                                     connector: None,
                                     condition: Condition {
                                         left: Operand::Column("age".to_owned()),
                                         equality: Equality::LT,
                                         right: Operand::Number(13f64),
                                     },
                                     sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ],
                                 }],
                   equations: vec![Equation {
                                       left: Operand::Column("x".to_owned()),
                                       right: Operand::Number(123f64),
                                   }],
               }),
               params("age=eq.f7521093-734d-488a-9f60-fc9f11f7e750&student=eq.true|gender=eq.\
                       M&x=123"));
}
