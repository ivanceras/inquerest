extern crate inquerest;
extern crate nom;

use inquerest::*;
use nom::IResult;


#[test]
fn test_column(){
    assert_eq!(
        IResult::Done("".as_bytes(), Operand::Column("age".to_owned())),
        operand("age".as_bytes()));
}

#[test]
fn test_number(){
    assert_eq!(
        IResult::Done("".as_bytes(), 123f64),
        number("123".as_bytes()));
}


#[test]
fn test_table_column(){
    assert_eq!(
        IResult::Done("".as_bytes(), Operand::Column("person.age".to_owned())),
        operand("person.age".as_bytes()));
}


#[test]
fn test_function(){
    assert_eq!(
        IResult::Done("".as_bytes(), Operand::Function(Function{
            function: "max".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        })),
        operand("max(age)".as_bytes()));
}

