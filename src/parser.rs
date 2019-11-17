use pom::parser::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::{self, FromStr};
use utils::*;

mod utils;

#[derive(Debug, PartialEq)]
pub struct Select {
    pub from: FromTable,
    pub filter: Option<Filter>,
    pub group_by: Vec<Operand>,
    pub having: Option<Filter>,
    pub selection: Vec<Operand>, // column selection
    pub order_by: Vec<Order>,
    pub range: Option<Range>,
}

#[derive(Debug, PartialEq)]
pub struct FromTable {
    pub from: Table,
    pub join: Option<(JoinType, Box<FromTable>)>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    name: String,
}

/// Only 3 join types is supported
/// - left join
///     product<-users
/// - right join
///     product->users
/// - inner_join
///     product-><-users
///
#[derive(Debug, PartialEq)]
pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Column(Column),
    Function(Function),
    Value(Value),
}

#[derive(Debug, PartialEq)]
pub struct Column {
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Operand>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Connector {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Eq,     // = ,  eq
    Neq,    // != , neq
    Lt,     // <,  lt
    Lte,    // <=, lte
    Gt,     // >, gt
    Gte,    // >=, gte
    In,     // IN, in
    NotIn,  // NOT IN, not_in
    Is,     // IS, is
    IsNot,  // IS NOT, is_not
    Like,   // LIKE, like
    Ilike,  // ILIKE case insensitive like, postgresql specific
    Starts, // Starts with, which will become ILIKE 'value%'
}
#[derive(Debug, PartialEq)]
pub struct Condition {
    pub left: Operand,
    pub operator: Operator,
    pub right: Operand,
}

#[derive(Debug, PartialEq)]
pub enum Filter {
    Simple {
        left: Condition,
        right: Option<(Connector, Condition)>,
    },
    Complex {
        left: Box<Filter>,
        right: Option<(Connector, Box<Filter>)>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
pub enum NullsWhere {
    First,
    Last,
}

#[derive(Debug, PartialEq)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}

#[derive(Debug, PartialEq, Default)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, PartialEq, Default)]
pub struct Limit {
    pub limit: i64,
    pub offset: Option<i64>,
}

/// any whitespace character
fn space<'a>() -> Parser<'a, char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
}

/// a valid identifier
fn ident<'a>() -> Parser<'a, char, String> {
    (is_a(alpha_or_underscore) + is_a(alphanum_or_underscore).repeat(0..))
        .map(|(ch1, rest_ch)| format!("{}{}", ch1, String::from_iter(rest_ch)))
}

/// table.column_name
fn table_column_name<'a>() -> Parser<'a, char, String> {
    (ident() - sym('.') + ident()).map(|(table, column)| format!("{}.{}", table, column)) | ident()
}

/// a number including decimal
fn number<'a>() -> Parser<'a, char, f64> {
    let integer = one_of("123456789") - one_of("0123456789").repeat(0..) | sym('0');
    let frac = sym('.') + one_of("0123456789").repeat(1..);
    let exp = one_of("eE") + one_of("+-").opt() + one_of("0123456789").repeat(1..);
    let number = sym('-').opt() + integer + frac.opt() + exp.opt();
    number
        .collect()
        .map(String::from_iter)
        .convert(|s| f64::from_str(&s))
}

fn integer<'a>() -> Parser<'a, char, i64> {
    let int = one_of("123456789") - one_of("0123456789").repeat(0..) | sym('0');
    int.collect()
        .map(String::from_iter)
        .convert(|s| i64::from_str(&s))
}

/// quoted string literal
fn quoted_string<'a>() -> Parser<'a, char, String> {
    let special_char = sym('\\')
        | sym('/')
        | sym('"')
        | sym('b').map(|_| '\x08')
        | sym('f').map(|_| '\x0C')
        | sym('n').map(|_| '\n')
        | sym('r').map(|_| '\r')
        | sym('t').map(|_| '\t');
    let escape_sequence = sym('\\') * special_char;
    let char_string = (none_of("\\\"") | escape_sequence)
        .repeat(1..)
        .map(String::from_iter);
    let utf16_char = tag("\\u")
        * is_a(|c: char| c.is_digit(16))
            .repeat(4)
            .map(String::from_iter)
            .convert(|digits| u16::from_str_radix(&digits, 16));
    let utf16_string = utf16_char.repeat(1..).map(|chars| {
        decode_utf16(chars)
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect::<String>()
    });
    let string = sym('"') * (char_string | utf16_string).repeat(0..) - sym('"');
    string.map(|strings| strings.concat())
}

/// string with no quote
fn string<'a>() -> Parser<'a, char, String> {
    let char_string = none_of("=&()").repeat(1..).map(String::from_iter);
    let string = char_string.repeat(0..);
    string.map(|strings| strings.concat())
}

fn column<'a>() -> Parser<'a, char, Column> {
    table_column_name().map(|name| Column { name })
}

fn table<'a>() -> Parser<'a, char, Table> {
    table_column_name().map(|name| Table { name })
}

fn bool<'a>() -> Parser<'a, char, bool> {
    tag("true").map(|_| true) | tag("false").map(|_| false)
}

fn null<'a>() -> Parser<'a, char, Value> {
    tag("null").map(|_| Value::Null)
}

fn value<'a>() -> Parser<'a, char, Value> {
    null()
        | bool().map(|v| Value::Bool(v))
        | number().map(|n| Value::Number(n))
        | string().map(|s| Value::String(s))
}

fn connector<'a>() -> Parser<'a, char, Connector> {
    sym('|').map(|_| Connector::Or) | sym('&').map(|_| Connector::And)
}

fn operator<'a>() -> Parser<'a, char, Operator> {
    tag("eq").map(|_| Operator::Eq)
        | tag("neq").map(|_| Operator::Neq)
        | tag("lte").map(|_| Operator::Lte)
        | tag("lt").map(|_| Operator::Lt)
        | tag("gte").map(|_| Operator::Gte)
        | tag("gt").map(|_| Operator::Gt)
        | tag("in").map(|_| Operator::In)
        | tag("not_in").map(|_| Operator::NotIn)
        | tag("is_not").map(|_| Operator::IsNot)
        | tag("like").map(|_| Operator::Like)
        | tag("ilike").map(|_| Operator::Ilike)
        | tag("starts").map(|_| Operator::Starts)
}

fn operand<'a>() -> Parser<'a, char, Operand> {
    null().map(Operand::Value)
        | bool().map(|v| Operand::Value(Value::Bool(v)))
        | number().map(|v| Operand::Value(Value::Number(v)))
        | column().map(Operand::Column)
        | function().map(Operand::Function)
        | value().map(Operand::Value)
}

fn operands<'a>() -> Parser<'a, char, Vec<Operand>> {
    list(call(operand), space() - sym(',') - space())
}

fn function<'a>() -> Parser<'a, char, Function> {
    (ident() - sym('(') + operands() - sym(')')).map(|(name, params)| Function { name, params })
}

/// Example:
/// age=gt.42
/// name=allan
fn condition<'a>() -> Parser<'a, char, Condition> {
    (operand() - sym('=') + (operator() - sym('.')).opt() + operand()).map(
        |((left, operator), right)| Condition {
            left,
            operator: operator.unwrap_or(Operator::Eq),
            right,
        },
    )
}

/// Example: age=gt.42&is_active=true
fn simple_filter<'a>() -> Parser<'a, char, Filter> {
    (condition() + (connector() + condition()).opt())
        .map(|(left, right)| Filter::Simple { left, right })
}

fn grouped_filter<'a>() -> Parser<'a, char, Filter> {
    sym('(') * call(filter) - sym(')')
}

fn first_grouped_filter<'a>() -> Parser<'a, char, Filter> {
    (grouped_filter() + (connector() + call(filter))).map(|(left, (connector, right))| {
        Filter::Complex {
            left: Box::new(left),
            right: Some((connector, Box::new(right))),
        }
    })
}

fn filter<'a>() -> Parser<'a, char, Filter> {
    first_grouped_filter() | grouped_filter() | simple_filter()
}

fn from_table<'a>() -> Parser<'a, char, FromTable> {
    (table() + (join_type() + call(from_table)).opt()).map(|(from, join)| FromTable {
        from,
        join: join.map(|(join_type, from_table)| (join_type, Box::new(from_table))),
    })
}

fn join_type<'a>() -> Parser<'a, char, JoinType> {
    tag("-><-").map(|_| JoinType::InnerJoin)
        | tag("<-").map(|_| JoinType::LeftJoin)
        | tag("->").map(|_| JoinType::RightJoin)
}

fn page_size<'a>() -> Parser<'a, char, i64> {
    (tag("page_size") - sym('=')) * integer()
}

fn page<'a>() -> Parser<'a, char, Page> {
    ((tag("page") - sym('=')) * integer() - sym('&') + page_size())
        .map(|(page, page_size)| Page { page, page_size })
}

fn offset<'a>() -> Parser<'a, char, i64> {
    (tag("offset") - sym('=')) * integer()
}

fn limit<'a>() -> Parser<'a, char, Limit> {
    ((tag("limit") - sym('=')) * integer() + (sym('&') * offset()).opt())
        .map(|(limit, offset)| Limit { limit, offset })
}

/// Example: age=gt.42&is_active=true|gender=eq."M"&class="Human"

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_page_with_size() {
        let input = to_chars("page=2&page_size=10");
        let ret = page().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            Page {
                page: 2,
                page_size: 10,
            }
        )
    }

    #[test]
    fn test_limit() {
        let input = to_chars("limit=10");
        let ret = limit().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            Limit {
                limit: 10,
                offset: None,
            }
        )
    }

    #[test]
    fn test_limit_with_offset() {
        let input = to_chars("limit=10&offset=20");
        let ret = limit().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            Limit {
                limit: 10,
                offset: Some(20),
            }
        )
    }

    #[test]
    fn test_from_right_join() {
        let input = to_chars("product->users");
        let ret = from_table().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            FromTable {
                from: Table {
                    name: "product".into()
                },
                join: Some((
                    JoinType::RightJoin,
                    Box::new(FromTable {
                        from: Table {
                            name: "users".into()
                        },
                        join: None,
                    }),
                ),),
            }
        );
    }

    #[test]
    fn test_from_left_join() {
        let input = to_chars("product<-users");
        let ret = from_table().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            FromTable {
                from: Table {
                    name: "product".into()
                },
                join: Some((
                    JoinType::LeftJoin,
                    Box::new(FromTable {
                        from: Table {
                            name: "users".into()
                        },
                        join: None,
                    }),
                ),),
            }
        );
    }

    #[test]
    fn test_from_inner_join() {
        let input = to_chars("product-><-users");
        let ret = from_table().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            FromTable {
                from: Table {
                    name: "product".into()
                },
                join: Some((
                    JoinType::InnerJoin,
                    Box::new(FromTable {
                        from: Table {
                            name: "users".into()
                        },
                        join: None,
                    }),
                ),),
            }
        );
    }

    #[test]
    fn test_from_table() {
        let input = to_chars("product->users<-customer");
        let ret = from_table().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            FromTable {
                from: Table {
                    name: "product".into()
                },
                join: Some((
                    JoinType::RightJoin,
                    Box::new(FromTable {
                        from: Table {
                            name: "users".into()
                        },
                        join: Some((
                            JoinType::LeftJoin,
                            Box::new(FromTable {
                                from: Table {
                                    name: "customer".into()
                                },
                                join: None,
                            }),
                        ),),
                    }),
                ),),
            }
        );
    }

    #[test]
    fn test_more_complex_filter2() {
        let input = to_chars("(age=gt.42&is_active=true)|(gender=eq.'M'&class='Human')");
        let ret = filter().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            Filter::Complex {
                left: Box::new(Filter::Simple {
                    left: Condition {
                        left: Operand::Column(Column { name: "age".into() }),
                        operator: Operator::Gt,
                        right: Operand::Value(Value::Number(42.0))
                    },
                    right: Some((
                        Connector::And,
                        Condition {
                            left: Operand::Column(Column {
                                name: "is_active".into()
                            }),
                            operator: Operator::Eq,
                            right: Operand::Value(Value::Bool(true))
                        }
                    ))
                }),
                right: Some((
                    Connector::Or,
                    Box::new(Filter::Simple {
                        left: Condition {
                            left: Operand::Column(Column {
                                name: "gender".into()
                            }),
                            operator: Operator::Eq,
                            right: Operand::Value(Value::String("'M'".into()))
                        },
                        right: Some((
                            Connector::And,
                            Condition {
                                left: Operand::Column(Column {
                                    name: "class".into()
                                }),
                                operator: Operator::Eq,
                                right: Operand::Value(Value::String("'Human'".into()))
                            },
                        ))
                    })
                ))
            }
        );
    }

    #[test]
    fn test_complex_filter1() {
        let input = to_chars("(age=gt.42&is_active=true)|gender=eq.'M'");
        let ret = filter().parse(&input).expect("must be parsed");
        println!("{:#?}", ret);
        assert_eq!(
            ret,
            Filter::Complex {
                left: Box::new(Filter::Simple {
                    left: Condition {
                        left: Operand::Column(Column { name: "age".into() }),
                        operator: Operator::Gt,
                        right: Operand::Value(Value::Number(42.0))
                    },
                    right: Some((
                        Connector::And,
                        Condition {
                            left: Operand::Column(Column {
                                name: "is_active".into()
                            }),
                            operator: Operator::Eq,
                            right: Operand::Value(Value::Bool(true))
                        }
                    ))
                }),
                right: Some((
                    Connector::Or,
                    Box::new(Filter::Simple {
                        left: Condition {
                            left: Operand::Column(Column {
                                name: "gender".into()
                            }),
                            operator: Operator::Eq,
                            right: Operand::Value(Value::String("'M'".into()))
                        },
                        right: None,
                    })
                ))
            }
        );
    }

    #[test]
    fn test_grouped_filter() {
        let input = to_chars("(gender=eq.'M'&class='Human')");
        let ret = filter().parse(&input).expect("must be parsed");
        println!("ret: {:#?}", ret);
        assert_eq!(
            ret,
            Filter::Simple {
                left: Condition {
                    left: Operand::Column(Column {
                        name: "gender".into()
                    }),
                    operator: Operator::Eq,
                    right: Operand::Value(Value::String("'M'".into()))
                },
                right: Some((
                    Connector::And,
                    Condition {
                        left: Operand::Column(Column {
                            name: "class".into()
                        }),
                        operator: Operator::Eq,
                        right: Operand::Value(Value::String("'Human'".into()))
                    }
                ))
            }
        );
    }

    #[test]
    fn test_filter_simple_filter_and() {
        let input = to_chars("age=gt.42&is_active=true");
        let ret = filter().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Filter::Simple {
                left: Condition {
                    left: Operand::Column(Column { name: "age".into() }),
                    operator: Operator::Gt,
                    right: Operand::Value(Value::Number(42.0))
                },
                right: Some((
                    Connector::And,
                    Condition {
                        left: Operand::Column(Column {
                            name: "is_active".into()
                        }),
                        operator: Operator::Eq,
                        right: Operand::Value(Value::Bool(true))
                    }
                ))
            }
        );
    }
    #[test]
    fn test_filter_simple_filteror() {
        let input = to_chars("age=gt.42|is_active=true");
        let ret = simple_filter().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Filter::Simple {
                left: Condition {
                    left: Operand::Column(Column { name: "age".into() }),
                    operator: Operator::Gt,
                    right: Operand::Value(Value::Number(42.0))
                },
                right: Some((
                    Connector::Or,
                    Condition {
                        left: Operand::Column(Column {
                            name: "is_active".into()
                        }),
                        operator: Operator::Eq,
                        right: Operand::Value(Value::Bool(true))
                    }
                ))
            }
        );
    }

    #[test]
    fn test_condition_gt() {
        let input = to_chars("age=gt.42");
        let ret = condition().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Condition {
                left: Operand::Column(Column { name: "age".into() }),
                operator: Operator::Gt,
                right: Operand::Value(Value::Number(42.0))
            }
        );
    }
    #[test]
    fn test_condition_lte() {
        let input = to_chars("age=lte.42");
        let ret = condition().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Condition {
                left: Operand::Column(Column { name: "age".into() }),
                operator: Operator::Lte,
                right: Operand::Value(Value::Number(42.0))
            }
        );
    }

    #[test]
    fn test_condition_default_eq() {
        let input = to_chars("age=42");
        let ret = condition().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Condition {
                left: Operand::Column(Column { name: "age".into() }),
                operator: Operator::Eq,
                right: Operand::Value(Value::Number(42.0))
            }
        );
    }

    #[test]
    fn test_function() {
        let input = to_chars("max(seq_no)");
        let ret = function().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Function {
                name: "max".into(),
                params: vec![Operand::Column(Column {
                    name: "seq_no".into()
                })]
            }
        );
    }

    #[test]
    fn test_column() {
        let input = to_chars("product_id");
        let ret = column().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Column {
                name: "product_id".into()
            }
        );
    }

    #[test]
    fn test_value_bool() {
        let input = to_chars("true");
        let ret = value().parse(&input).expect("must be parsed");
        assert_eq!(ret, Value::Bool(true));
    }
    #[test]
    fn test_value_bool2() {
        let input = to_chars("false");
        let ret = value().parse(&input).expect("must be parsed");
        assert_eq!(ret, Value::Bool(false));
    }
    #[test]
    fn test_value_number() {
        let input = to_chars("0.1312312");
        let ret = value().parse(&input).expect("must be parsed");
        assert_eq!(ret, Value::Number(0.1312312));
    }
    #[test]
    fn test_value_number2() {
        let input = to_chars("3.14159");
        let ret = value().parse(&input).expect("must be parsed");
        assert_eq!(ret, Value::Number(3.14159));
    }

    #[test]
    fn test_string() {
        let input = to_chars("product_id");
        let ret = string().parse(&input).expect("must be parsed");
        assert_eq!(ret, "product_id");
    }

    #[test]
    fn test_iregular_string() {
        let input = to_chars("a string value\"pr'oduct_id");
        let ret = string().parse(&input).expect("must be parsed");
        assert_eq!(ret, "a string value\"pr\'oduct_id");
    }

    #[test]
    fn test_bool_operand() {
        let input = to_chars("true");
        let ret = operand().parse(&input).expect("must be parsed");
        assert_eq!(ret, Operand::Value(Value::Bool(true)));
    }
    #[test]
    fn test_null_operand() {
        let input = to_chars("null");
        let ret = operand().parse(&input).expect("must be parsed");
        assert_eq!(ret, Operand::Value(Value::Null));
    }
}
