extern crate inquerest;
extern crate nom;

use inquerest::*;
use nom::IResult;

#[test]
fn test_group(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![Operand::Column("age".to_owned())]),
        group_by("group_by=age".as_bytes()));
}

#[test]
fn test_group2(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![
            Operand::Column("age".to_owned()),
            Operand::Column("grade".to_owned()),
            ]),
        group_by("group_by=age,grade".as_bytes()));
}

#[test]
fn test_group3(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![
            Operand::Column("age".to_owned()),
            Operand::Column("grade".to_owned()),
            Operand::Column("gender".to_owned()),
            ]),
        group_by("group_by=age,grade,gender".as_bytes()));
}

#[test]
fn test_group_sum(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![
            Operand::Function(
                        Function{
                                function: "sum".to_owned(),
                                params: vec![Operand::Column("age".to_owned())]
                            }
                    ),
            Operand::Column("grade".to_owned()),
            Operand::Column("gender".to_owned()),
            ]),
        group_by("group_by=sum(age),grade,gender".as_bytes()));
}
