extern crate inquirest;

use inquirest::*;

#[test]
fn test_condition1(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LT, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=lt.13"));
}

#[test]
fn test_condition_equal(){
    assert_eq!(condition("age=lt.13"), condition("(age=lt.13)"))
}