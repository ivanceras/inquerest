extern crate inquerest;
extern crate nom;

use inquerest::*;
use nom::IResult;

#[test]
fn test_desc(){
    assert_eq!(
        IResult::Done("".as_bytes(), Order{
            operand: Operand::Column("age".to_owned()), 
            direction: Some(Direction::DESC), 
			nulls_where: None,
        }),
        order("age.desc".as_bytes()));
}


#[test]
fn test_asc(){
    assert_eq!(
        IResult::Done("".as_bytes(), Order{
            operand: Operand::Column("age".to_owned()), 
            direction: Some(Direction::ASC), 
			nulls_where: None,
        }),
        order("age.asc".as_bytes()));
}


#[test]
fn test_table_column_order(){
    assert_eq!(
        IResult::Done("".as_bytes(), Order{
            operand: Operand::Column("person.age".to_owned()), 
            direction: Some(Direction::DESC), 
			nulls_where: None,
        }),
        order("person.age.desc".as_bytes()));
}


#[test]
fn test_order_by(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![Order{
            operand: Operand::Column("age".to_owned()), 
            direction: Some(Direction::ASC), 
			nulls_where: None,
        }]),
        order_by("order_by=age.asc".as_bytes()));
}

#[test]
fn test_order_by2(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![Order{
                operand: Operand::Column("age".to_owned()), 
                direction: Some(Direction::ASC), 
				nulls_where: None,
            },
            Order{
                operand: Operand::Column("grade".to_owned()), 
                direction: Some(Direction::DESC), 
				nulls_where: None,
            }
            ]),
        order_by("order_by=age.asc,grade.desc".as_bytes()));
}


#[test]
fn test_order_by3(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![Order{
                operand: Operand::Column("age".to_owned()), 
                direction: Some(Direction::DESC), 
				nulls_where: None,
            },
            Order{
                operand: Operand::Column("grade".to_owned()), 
                direction: Some(Direction::DESC), 
				nulls_where: None,
            },
            Order{
                operand: Operand::Column("height".to_owned()), 
                direction: Some(Direction::ASC), 
				nulls_where: None,
            }
            ]),
        order_by("order_by=age.desc,grade.desc,height.asc".as_bytes()));
}


#[test]
fn test_order_by4(){
    assert_eq!(
        IResult::Done("".as_bytes(), vec![Order{
                operand: Operand::Column("person.age".to_owned()), 
                direction: Some(Direction::DESC), 
				nulls_where: None,
            },
            Order{
                operand: Operand::Column("student.grade".to_owned()), 
                direction: Some(Direction::DESC), 
				nulls_where: None,
            },
            Order{
                operand: Operand::Column("height".to_owned()), 
                direction: Some(Direction::ASC), 
				nulls_where: None,
            }
            ]),
        order_by("order_by=person.age.desc,student.grade.desc,height.asc".as_bytes()));
}
