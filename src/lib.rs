#[macro_use]
extern crate nom;

use std::str::{self,FromStr};
use nom::{IResult,digit,alphanumeric,anychar,is_alphanumeric};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Equation {
    pub left: Operand,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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
#[derive(Clone)]
pub enum Connector {
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Filter {
    pub connector: Option<Connector>,
    pub condition: Condition,
    pub sub_filters: Vec<Filter>,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Select {
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
#[derive(Clone)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Limit {
    pub limit: i64,
    pub offset: Option<i64>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table: Operand,
    pub column1: Vec<String>,
    pub column2: Vec<String>,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Param{
    Condition(Condition),
    Equation(Equation)
}


named!(pub value<&str>, 
  map_res!(complete!(recognize!(many1!(is_not_s!("()&|,="))))
    ,str::from_utf8
  )
);

named!(pub column<&str>, 
  map_res!(recognize!(many1!(one_of!("abcdefghijklmnopqrstuvwxyz0123456789_.")))
    ,str::from_utf8
  )
);

named!(pub column_only<&str>, 
  map_res!(recognize!(many1!(one_of!("abcdefghijklmnopqrstuvwxyz0123456789_")))
    ,str::from_utf8
  )
);

/*
named!(column <&str>, map_res!(
        complete!(alphanumeric),
        str::from_utf8
    )
);
*/

named!(pub boolean <bool>,
    alt!(tag!("true") => {|_| true} |
         tag!("false") => {|_| false}
        )
);

named!(pub from <Vec<Operand>>,
    do_parse!(
        tag!("from=") >>
        table: many1!(column) >> 
        ({
            let mut tables = vec![];
                for tbl in table{
                    tables.push(Operand::Column(tbl.to_string())) 
                }
                tables
        })
    )
);


named!(modifier <Modifier>,
    alt_complete!(
        tag!("left") => {|_| Modifier::LEFT } |
        tag!("right") => {|_| Modifier::RIGHT } |
        tag!("full") => {|_| Modifier::FULL }
    )
);

named!(pub join <Join>,
    do_parse!(
        md: opt!(
                do_parse!(
                    md: modifier >> 
                    tag!("_") >> 
                    (md)
                )
            ) >>
        tag!("join=") >>
        table: column >>
        on:many1!(
            do_parse!(
                tag!("&on=") >>
                col1: column >>
                tag!("=") >>
                col2: column>>
                (col1, col2)
            )
        )>>
        ({
        let mut col1 = vec![];
        let mut col2 = vec![];
        for (c1,c2) in on{
           col1.push(c1.to_string());
           col2.push(c2.to_string());
        }
        Join{
            modifier: md,
            join_type: None,
            table: Operand::Column(table.to_string()),
            column1: col1,
            column2: col2 
        }}) 
    )
);

named!(pub number<f64>,
    map_res!(
      map_res!(
        ws!(digit),
        str::from_utf8
      ),
      FromStr::from_str
    )
);

named!(pub operand <Operand>,
   alt_complete!(
        boolean => {|b| Operand::Boolean(b) } |
        function => {|f| Operand::Function(f)} |
        value => {|v:&str| 
            {
                match f64::from_str(v){
                    Ok(f) => Operand::Number(f as f64),
                    Err(_) => {
                        match column(v.as_bytes()){
                            IResult::Done(rest,col) => {
                                if rest == "".as_bytes(){
                                    Operand::Column(col.to_string())
                                }else{
                                    Operand::Value(v.to_string()) 
                                }
                            },
                            _ => Operand::Value(v.to_string())
                        }
                    }
                }
            } 
        }
   ) 
);

named!(pub equality<Equality>,
    alt!(tag!("eq") => {|_| Equality::EQ} | 
         tag!("neq") => {|_| Equality::NEQ} |
         tag!("lte") => {|_| Equality::LTE} |
         tag!("lt") => {|_| Equality::LT} |
         tag!("gte") => {|_| Equality::GTE} |
         tag!("gt") => {|_| Equality::GT} |
         tag!("in") => {|_| Equality::IN} |
         tag!("not_in") => {|_| Equality::NOT_IN} |
         tag!("is") => {|_| Equality::IS} |
         tag!("is_not") => {|_| Equality::IS_NOT} |
         tag!("like") => {|_| Equality::LIKE} |
         tag!("ilike") => {|_| Equality::ILIKE} |
         tag!("st") => {|_| Equality::ST}
    )
);


named!(pub connector <Connector>,
   alt!(tag!("&") => {|_| Connector::AND} |
        tag!("|") => {|_| Connector::OR}
   )
);

named!(pub direction <Direction>,
    alt_complete!(tag!("asc") => {|_| Direction::ASC} |
         tag!("desc") => {|_| Direction::DESC} 
    )
);

named!(pub nulls_where <NullsWhere>,
    alt_complete!(tag!("nullsfirst") => {|_| NullsWhere::FIRST} |
         tag!("nullslast") => {|_| NullsWhere::LAST}
    )
);

/// hard to distinguest `column.direction` from `table.column`
/// i.e: `person.age` and `age.desc`
/// use strict `column_only` instead, which doesn't include `.`
named!(pub order <Order>,
    do_parse!(
        col: column_only >>
        dir: opt!(complete!(preceded!(tag!("."), direction))) >>
        nulls: opt!(complete!(preceded!(tag!("."), nulls_where))) >>
        (Order{
            operand: Operand::Column(col.to_string()),
            direction: dir,
            nulls_where: nulls
        })
    )
);

named!(pub order_by <Vec<Order>>,
    do_parse!(
        tag!("order_by=") >>
        orders: separated_nonempty_list!(tag!(","),order) >>
        (orders)
    )
);



named!(pub param <Param>,
    alt!(condition => {|c| Param::Condition(c)}| 
         equation => {|e| Param::Equation(e)}
    )
);

fn fold_conditions(initial: Condition, remainder: Vec<(Connector, Condition)>) -> Filter{
    let mut sub_filters = vec![];
    for (conn, cond) in remainder{
        let sub_filter = Filter{
                connector: Some(conn),
                condition: cond,
                sub_filters: vec![]
            };
        sub_filters.push(sub_filter);
    }
    Filter{
        connector: None,
        condition: initial,
        sub_filters: sub_filters
    }
}

fn fold_filters(initial: Filter, remainder: Vec<(Connector, Filter)>) -> Filter{
    let mut sub_filters = vec![];
    for (conn, filtr) in remainder{
        sub_filters.push(filtr);
    }
    let mut filter = initial.to_owned();
    filter.sub_filters = sub_filters;
    filter
}


named!(pub filter <Filter>,
    do_parse!(
        initial: condition_expr >>
        remainder: many0!(
               do_parse!(
                    conn: connector >>
                    cond: condition_expr >> 
                        (conn, cond)
               )
            )
            >> (fold_conditions(initial, remainder))
    )
);

named!(filter_expr <Filter>,
    alt_complete!(filter | 
        delimited!(tag!("("), filter_expr, tag!(")")) |
        do_parse!(
            initial: filter >>
            remainder: many0!(
                do_parse!(
                    conn: connector >>
                    filtr: filter_expr >>
                        ( conn, filtr)
                )
            )
            >> (fold_filters(initial, remainder))
        )
    )
);
    

named!(pub function <Function>,
    do_parse!(
        fnc: column >>
        tag!("(") >>
        p: many0!(operand) >>
        tag!(")") >>
        (Function{
            function: fnc.to_string(),
            params: p
        }) 
    )
);

named!(pub params < Vec<Param> >,
    separated_list!(tag!("&"), param)
);

named!(pub equation <Equation>, 
    do_parse!(
        col:column >>
        tag!("=") >>
        r: operand >>
        (Equation{
            left: Operand::Column(col.to_string()),
            right: r
        })
    )
);


named!(pub condition <Condition>,
    do_parse!(
        l: operand >>
        tag!("=") >>
        eq:equality >>
        tag!(".") >>
        r: operand >>
        (Condition{
            left: l, //Operand::Column(l.to_string()),
            equality: eq,
            right: r
        })
    )
);

named!(pub condition_expr <Condition>,
    alt_complete!(condition | complete!(delimited!(tag!("("), condition_expr, tag!(")"))))
);

named!(pub having <Vec<Filter>>,
    preceded!(
        tag!("having="),
        many1!(filter_expr)
    )
);

named!(pub group_by <Vec<Operand>>,
    preceded!(
        tag!("group_by="),
        separated_nonempty_list!(tag!(","),operand)
    )
);

named!(pub range <Range>,
    alt_complete!(
        do_parse!(
            tag!("page=") >>
            pg: number >>
            tag!("&page_size=") >>
            pg_sz: number >>
            (Range::Page(
                Page{
                    page: pg as i64,
                    page_size: pg_sz as i64
                }
            ))
        ) |
        do_parse!(
           tag!("limit=") >>
           lm: number >>
           off: opt!(complete!(preceded!(tag!("&offset="),number))) >>
           (Range::Limit(
                Limit{
                    limit: lm as i64,
                    offset: off.map(|v| v as i64)
                }
            ))
        )
    )
);

named!(pub query <Select>,
    do_parse!(
        fr: opt!(from) >>
        j: complete!(many0!(preceded!(opt!(tag!("&")),join))) >>
        filtr: complete!(many0!(preceded!(opt!(tag!("&")),filter))) >>
        g: opt!(preceded!(tag!("&"),group_by)) >> 
        h: opt!(preceded!(tag!("&"), having)) >>
        ord: opt!(preceded!(tag!("&"), order_by))>>
        rng: opt!(preceded!(tag!("&"), range)) >>
        eq: many0!(preceded!(opt!(tag!("&")), equation)) >>
        (Select{
            from: match fr{Some(fr)=>fr,None=>vec![]},
            join: j,
            filters: filtr,
            group_by: match g{Some(g)=>g,None=>vec![]},
            having: match h{Some(h)=>h,None=>vec![]},
            order_by: match ord{Some(ord)=>ord,None=>vec![]},
            range: rng,
            equations: eq,
        }
        )
    )
);


#[derive(Debug)]
pub enum ParseError{
    NomError(String)
}

pub fn parse(arg: &str) -> Result<Select, ParseError>{
    match query(arg.as_bytes()){
        IResult::Done(_,query) =>
            Ok(query),
        IResult::Error(e) =>
            Err(ParseError::NomError(format!("{}",e))),
        IResult::Incomplete(n)=>
            Err(ParseError::NomError(format!("Incomplete: {:?}",n)))
    }
}


named!(unsigned_float <f64>, map_res!(
  map_res!(
    recognize!(
      alt_complete!(
        delimited!(digit, tag!("."), opt!(complete!(digit))) |
        delimited!(opt!(digit), tag!("."), complete!(digit)) |
        complete!(digit)
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(float <f64>, map!(
  pair!(
    opt!(alt!(tag!("+") | tag!("-"))),
    unsigned_float
  ),
  |(sign, value): (Option<&[u8]>, f64)| {
    sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f64) } else { None }).unwrap_or(1f64) * value
  }
));




#[test]
fn test_boolean_true() {
    assert_eq!(IResult::Done("".as_bytes(), true), boolean("true".as_bytes()));
}
#[test]
fn test_boolean_false() {
    assert_eq!(IResult::Done("".as_bytes(), false), boolean("false".as_bytes()));
}

#[test]
fn test_number() {
    assert_eq!(IResult::Done("".as_bytes(), 123f64), number("123".as_bytes()));
}


#[test]
fn test_column() {
    assert_eq!(IResult::Done("".as_bytes(), Operand::Column("age".to_owned())), operand("age".as_bytes()));
}




#[test]
fn test_table_column() {
    assert_eq!(IResult::Done("".as_bytes(), Operand::Column("person.age".to_owned())),
               operand("person.age".as_bytes()));
}


#[test]
fn test_from() {
    assert_eq!(IResult::Done("".as_bytes(), vec![Operand::Column("person".to_owned())]),
               from("from=person".as_bytes()));
}

#[test]
fn test_equation_preceded() {
    assert_eq!(IResult::Done("".as_bytes(), Select{equations: vec![
                            Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(1f64) }, 
                        ],..Default::default()}),
               query("&x=1".as_bytes()));
}


#[test]
fn test_equation() {
    assert_eq!(IResult::Done("".as_bytes(), Select{
                    equations: vec![
                            Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(1f64) }, 
                    ] ,..Default::default()}),
               query("x=1".as_bytes()));
}

#[test]
fn test_equation_focused_record() {
    assert_eq!(IResult::Done("".as_bytes(), Select{
                    equations: vec![
                            Equation { left: Operand::Column("focused_record".to_owned()), right: Operand::Value("3e51d5f9-5bff-4664-9946-47bf37973636".to_string()) }, 
                    ] ,..Default::default()}),
               query("focused_record=3e51d5f9-5bff-4664-9946-47bf37973636".as_bytes()));
}

#[test]
fn test_left_join() {
    assert_eq!(IResult::Done("".as_bytes(), Join {
                   modifier: Some(Modifier::LEFT),
                   join_type: None,
                   table: Operand::Column("person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("left_join=person&on=person.student_id=student.id".as_bytes()));
}


#[test]
fn test_join() {
    assert_eq!(IResult::Done("".as_bytes(), Join {
                   modifier: None,
                   join_type: None,
                   table: Operand::Column("bazaar.person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("join=bazaar.person&on=person.student_id=student.id".as_bytes()));
}


#[test]
#[should_panic]
fn test_join_without_on() {
    assert_eq!(IResult::Done("".as_bytes(), Join {
                   modifier: None,
                   join_type: None,
                   table: Operand::Column("bazaar.person".to_owned()),
                   column1: vec!["person.student_id".to_owned()],
                   column2: vec!["student.id".to_owned()],
               }),
               join("join=bazaar.person".as_bytes()));
}


#[test]
fn test_function() {
    assert_eq!(IResult::Done("".as_bytes(), Function {
                   function: "min".to_owned(),
                   params: vec![Operand::Column("age".to_owned())],
               }),
               function("min(age)".as_bytes()));
}

#[test]
fn test_order() {
    assert_eq!(IResult::Done("".as_bytes(), Order {
                   operand: Operand::Column("age".to_owned()),
                   direction: Some(Direction::DESC),
                   nulls_where: None,
               }),
               order("age.desc".as_bytes()));
}



#[test]
fn test_euqation() {
    assert_eq!(IResult::Done("".as_bytes(), Equation {
                   left: Operand::Column("x".to_owned()),
                   right: Operand::Number(123f64),
               }),
               equation("x=123".as_bytes()));
}



#[test]
fn test_condition() {
    assert_eq!(IResult::Done("".as_bytes(),Condition {
                   left: Operand::Column("age".to_owned()),
                   equality: Equality::EQ,
                   right: Operand::Number(13f64),
               }),
               condition("age=eq.13".as_bytes()));
}

#[test]
fn test_starts_with() {
    assert_eq!(IResult::Done("".as_bytes(),Condition {
                   left: Operand::Column("name".to_owned()),
                   equality: Equality::ST,
                   right: Operand::Column("le".to_string()),
               }),
               condition("name=st.le".as_bytes()));
}

#[test]
fn test_percent20() {
    let url = "name=st.lee cesar";
    println!("url: {}", url);
    assert_eq!(IResult::Done("".as_bytes(),Condition {
                   left: Operand::Column("name".to_owned()),
                   equality: Equality::ST,
                   right: Operand::Value("lee cesar".to_string()),
               }),
               condition(url.as_bytes()));
}

#[test]
fn test_filter() {
    assert_eq!(IResult::Done("".as_bytes(),Filter {
                   connector: None,
                   condition: Condition {
                       left: Operand::Column("student".to_owned()),
                       equality: Equality::EQ,
                       right: Operand::Boolean(true),
                   },
                   sub_filters: vec![],
               }),
               filter("student=eq.true".as_bytes()))
}







#[test]
fn test_query() {
    let arg = "age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,\
                      gender&having=min(age)=gt.13&order_by=age.desc,height.\
                      asc&limit=100&offset=25&x=123&y=456";
    println!("{}\n", arg);
    let result = query(arg.as_bytes());
    println!("{:#?}\n",result);
    assert_eq!(IResult::Done("".as_bytes(), Select {
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
                                            sub_filters: vec![]
                                        },
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Value("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ],
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
               result);
}

