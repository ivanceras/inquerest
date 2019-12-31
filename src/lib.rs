//! Inquerest can parse complex url query into a SQL abstract syntax tree.
//!
//!```rust,ignore
//! Select {
//!         from_table: FromTable {
//!             from: Table {
//!                 name: "person",
//!             },
//!             join: None,
//!         },
//!         filter: Some(
//!             BinaryOperation(
//!                 BinaryOperation {
//!                     left: BinaryOperation(
//!                         BinaryOperation {
//!                             left: Column(
//!                                 Column {
//!                                     name: "age",
//!                                 },
//!                             ),
//!                             operator: Lt,
//!                             right: Value(
//!                                 Number(
//!                                     42.0,
//!                                 ),
//!                             ),
//!                         },
//!                     ),
//!                     operator: And,
//!                     right: Nested(
//!                         BinaryOperation(
//!                             BinaryOperation {
//!                                 left: BinaryOperation(
//!                                     BinaryOperation {
//!                                         left: Column(
//!                                             Column {
//!                                                 name: "student",
//!                                             },
//!                                         ),
//!                                         operator: Eq,
//!                                         right: Value(
//!                                             Bool(
//!                                                 true,
//!                                             ),
//!                                         ),
//!                                     },
//!                                 ),
//!                                 operator: Or,
//!                                 right: BinaryOperation(
//!                                     BinaryOperation {
//!                                         left: Column(
//!                                             Column {
//!                                                 name: "gender",
//!                                             },
//!                                         ),
//!                                         operator: Eq,
//!                                         right: Value(
//!                                             String(
//!                                                 "M",
//!                                             ),
//!                                         ),
//!                                     },
//!                                 ),
//!                             },
//!                         ),
//!                     ),
//!                 },
//!             ),
//!         ),
//!         group_by: Some(
//!             [
//!                 Function(
//!                     Function {
//!                         name: "sum",
//!                         params: [
//!                             Column(
//!                                 Column {
//!                                     name: "age",
//!                                 },
//!                             ),
//!                         ],
//!                     },
//!                 ),
//!                 Column(
//!                     Column {
//!                         name: "grade",
//!                     },
//!                 ),
//!                 Column(
//!                     Column {
//!                         name: "gender",
//!                     },
//!                 ),
//!             ],
//!         ),
//!         having: Some(
//!             BinaryOperation(
//!                 BinaryOperation {
//!                     left: Function(
//!                         Function {
//!                             name: "min",
//!                             params: [
//!                                 Column(
//!                                     Column {
//!                                         name: "age",
//!                                     },
//!                                 ),
//!                             ],
//!                         },
//!                     ),
//!                     operator: Gt,
//!                     right: Value(
//!                         Number(
//!                             42.0,
//!                         ),
//!                     ),
//!                 },
//!             ),
//!         ),
//!         projection: None,
//!         order_by: Some(
//!             [
//!                 Order {
//!                     expr: Column(
//!                         Column {
//!                             name: "age",
//!                         },
//!                     ),
//!                     direction: Some(
//!                         Desc,
//!                     ),
//!                 },
//!                 Order {
//!                     expr: Column(
//!                         Column {
//!                             name: "height",
//!                         },
//!                     ),
//!                     direction: Some(
//!                         Asc,
//!                     ),
//!                 },
//!             ],
//!         ),
//!         range: Some(
//!             Page(
//!                 Page {
//!                     page: 20,
//!                     page_size: 100,
//!                 },
//!             ),
//!         ),
//!     }
//! ```
//! Which translate to the sql statement:
//! ```sql
//! SELECT * FROM person WHERE age < 42 AND (student = true OR gender = 'M') GROUP BY sum(age), grade, gender HAVING min(age) > 42 ORDER BY age DESC, height ASC LIMIT 100 OFFSET 1900 ROWS
//! ```
//! Note: However, you don't want to convert to the sql statement directly to avoid sql injection
//! attack. You need to validate the tables and columns if it is allowed to be accessed by the
//! user. You also need to extract the values yourself and supply it as a parameterized value into
//! your ORM.
//!
pub use restq;

pub use restq::{
    ast::{
        Expr,
        Select,
    },
    parser::filter_expr,
    to_chars,
    Error,
};

/// Parse a path and query in a url to a Select AST
/// Example:
/// ```rust
///     use inquerest::*;
///
///     let url = "/person?age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
///     let query = inquerest::parse_query(url);
///     println!("query: {:#?}", query);
///     println!(
///         "sql query: {}",
///         query.unwrap().into_sql_statement(None).unwrap().to_string()
///     );
/// ```
pub fn parse_query(input: &str) -> Result<Select, Error> {
    let input_chars = to_chars(input);
    restq_http::parse_select_chars(&input_chars)
}

/// Parse the query in a url to an Expression
///
/// Example:
/// ```rust
///     use inquerest::*;
///
///     let filter = "age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
///     let result = parse_filter(filter);
///     println!("filter_only: {:#?}", result);
/// ```
pub fn parse_filter(input: &str) -> Result<Expr, Error> {
    let input_chars = to_chars(input);
    parse_filter_chars(&input_chars)
}

fn parse_filter_chars(input: &[char]) -> Result<Expr, Error> {
    Ok(filter_expr().parse(input)?)
}
