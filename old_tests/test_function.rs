extern crate inquerest;
extern crate nom;

use inquerest::*;
use nom::IResult;

#[test]
fn test_min(){
    assert_eq!(
        IResult::Done("".as_bytes(), Function{
            function: "min".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("min(age)".as_bytes()));
}

#[test]
fn test_max(){
    assert_eq!(
        IResult::Done("".as_bytes(), Function{
            function: "max".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("max(age)".as_bytes()));
}

#[test]
fn test_sum(){
    assert_eq!(
        IResult::Done("".as_bytes(), Function{
            function: "sum".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("sum(age)".as_bytes()));
}


#[test]
fn test_max_sum(){
    assert_eq!(
        IResult::Done("".as_bytes(), 
            Function{
                function:"max".to_owned(),
                params: vec![Operand::Function(Function{
                                function: "sum".to_owned(),
                                params: vec![Operand::Column("age".to_owned())], 
                            })]
            }
        ),
        function("max(sum(age))".as_bytes()));
}
