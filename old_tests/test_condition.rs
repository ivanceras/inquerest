extern crate inquerest;
extern crate nom;

use inquerest::*;
use nom::IResult;


#[test]
fn test_eq(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::EQ, 
            right:Operand::Number(13f64)
        }),
        condition("age=eq.13".as_bytes()));
}

#[test]
fn test_neq(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::NEQ, 
            right:Operand::Number(13f64)
        }),
        condition("age=neq.13".as_bytes()));
}
#[test]
fn test_lt(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LT, 
            right:Operand::Number(13f64)
        }),
        condition("age=lt.13".as_bytes()));
}
#[test]
fn test_lte(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LTE, 
            right:Operand::Number(13f64)
        }),
        condition("age=lte.13".as_bytes()));
}

#[test]
#[should_panic]
fn test_ltee(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::LTE, 
            right:Operand::Number(13f64)
        }),
        condition("age=ltee.13".as_bytes()));
}

#[test]
fn test_gt(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GT, 
            right:Operand::Number(13f64)
        }),
        condition("age=gt.13".as_bytes()));
}
#[test]
fn test_gte(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GTE, 
            right:Operand::Number(13f64)
        }),
        condition("age=gte.13".as_bytes()));
}

#[test]
#[should_panic]
fn test_lgee(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
            left:Operand::Column("age".to_owned()), 
            equality:Equality::GTE, 
            right:Operand::Number(13f64)
        }),
        condition("age=gtee.13".as_bytes()));
}


//#[test]
fn test_function(){
    assert_eq!(
        IResult::Done("".as_bytes(), Condition{
                left: Operand::Function(
                        Function{
                            function: "min".to_owned(),
                            params: vec![Operand::Column("grade".to_owned())], 
                        }
                    ),
                equality: Equality::GTE,
                right:Operand::Number(3f64)
            }),
        condition("min(grade)=gte.3".as_bytes()));
}
