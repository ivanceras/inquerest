extern crate inquerest;

use inquerest::*;


#[test]
fn test_filters(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column(
                                    "age".to_owned()
                                ),
                                equality: Equality::LT,
                                right: Operand::Column(
                                    "13".to_owned()
                                )
                            },
                            subfilter: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column(
                                            "student".to_owned()
                                        ),
                                        equality: Equality::EQ,
                                        right: Operand::Column(
                                            "true".to_owned()
                                        )
                                    },
                                    subfilter: vec![]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column(
                                            "gender".to_owned()
                                        ),
                                        equality: Equality::EQ,
                                        right: Operand::Column(
                                            "M".to_owned()
                                        )
                                    },
                                    subfilter: vec![]
                                }
                            ]
                        }
                    ],
                    order_by: vec![],
                    group_by: vec![],
                    equations: vec![]
                }
            )
        
        , params("(age=lt.13)&(student=eq.true)|(gender=eq.M)"));
}


#[test]
fn test_filter_order(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column(
                                    "age".to_owned()
                                ),
                                equality: Equality::LT,
                                right: Operand::Column(
                                    "13".to_owned()
                                )
                            },
                            subfilter: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column(
                                            "student".to_owned()
                                        ),
                                        equality: Equality::EQ,
                                        right: Operand::Column(
                                            "true".to_owned()
                                        )
                                    },
                                    subfilter: vec![]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column(
                                            "gender".to_owned()
                                        ),
                                        equality: Equality::EQ,
                                        right: Operand::Column(
                                            "M".to_owned()
                                        )
                                    },
                                    subfilter: vec![]
                                }
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![],
                    equations: vec![]
                }
            )
        
        , params("(age=lt.13)&(student=eq.true)|(gender=eq.M)&order_by=age.desc,height.asc"));
    
    println!("{:#?}", params("(age=lt.13)&(student=eq.true)|(gender=eq.M)&order_by=age.desc,height.asc"));
}

