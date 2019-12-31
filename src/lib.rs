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
///     let url = "/person?age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
///     let query = inquerest::parse_query(url);
///     println!("query: {:#?}", query);
///     println!(
///         "sql query: {}",
///         query.unwrap().into_statement(None).unwrap().to_string()
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
