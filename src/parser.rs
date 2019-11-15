use pom::parser::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::{self, FromStr};
use utils::*;

mod utils;

#[derive(Debug, PartialEq)]
pub struct Equation {
    pub left: Operand,
    pub right: Operand,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Operand>,
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
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

//TODO: rename to Operator, BinaryOperator
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
pub struct Filter {
    pub left: Condition,
    pub right: Option<(Connector, Condition)>,
}

#[derive(Debug, PartialEq, Default)]
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

#[derive(Debug, PartialEq)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}

#[derive(Debug, PartialEq)]
pub enum JoinType {
    Cross,
    Inner,
    Outer,
    Natural,
}
#[derive(Debug, PartialEq)]
pub enum Modifier {
    Left,
    Right,
    Full,
}

#[derive(Debug, PartialEq)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table: Operand,
    pub column1: Vec<String>,
    pub column2: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Param {
    Condition(Condition),
    Equation(Equation),
}

fn space<'a>() -> Parser<'a, char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
}

fn ident<'a>() -> Parser<'a, char, String> {
    (is_a(alpha_or_underscore) + is_a(alphanum_or_underscore).repeat(0..))
        .map(|(ch1, rest_ch)| format!("{}{}", ch1, String::from_iter(rest_ch)))
}

/// table.column_name
fn table_column_name<'a>() -> Parser<'a, char, String> {
    (ident() - sym('.') + ident()).map(|(table, column)| format!("{}.{}", table, column)) | ident()
}

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

fn string<'a>() -> Parser<'a, char, String> {
    let char_string = none_of("=&()").repeat(1..).map(String::from_iter);
    let string = char_string.repeat(0..);
    string.map(|strings| strings.concat())
}

fn column<'a>() -> Parser<'a, char, Column> {
    table_column_name().map(|name| Column { name })
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

/// Example: age=gt.42&&is_active=true
fn filter<'a>() -> Parser<'a, char, Filter> {
    (condition() + (connector() + condition()).opt()).map(|(left, right)| Filter { left, right })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let input = to_chars("age=gt.42&is_active=true");
        let ret = filter().parse(&input).expect("must be parsed");
        assert_eq!(
            ret,
            Filter {
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
