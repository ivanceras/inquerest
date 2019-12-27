pub use restq;

use restq::{
    ast::{
        Expr,
        Select,
    },
    Error,
};

pub fn parse_query(input: &str) -> Result<Select, Error> {
    restq::parse_query(input)
}

pub fn parse_filter(input: &str) -> Result<Expr, Error> {
    restq::parse_filter(input)
}
