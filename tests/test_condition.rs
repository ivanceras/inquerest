extern crate inquerest;

use inquerest::*;


#[test]
fn test_eq(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::EQ, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=eq.13"));
}

#[test]
fn test_neq(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::NEQ, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=neq.13"));
}
#[test]
fn test_lt(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LT, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=lt.13"));
}
#[test]
fn test_lte(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LTE, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=lte.13"));
}

#[test]
#[should_panic]
fn test_ltee(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LTE, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=ltee.13"));
}

#[test]
fn test_gt(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GT, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=gt.13"));
}
#[test]
fn test_gte(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GTE, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=gte.13"));
}

#[test]
#[should_panic]
fn test_lgee(){
    assert_eq!(
        Ok(Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GTE, 
            right:Operand::Column("13".to_owned())
        }),
        condition("age=gtee.13"));
}

#[test]
fn test_condition_equal(){
    assert_eq!(condition("age=lt.13"), condition("(age=lt.13)"))
}