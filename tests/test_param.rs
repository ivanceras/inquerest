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
                                right: Operand::Number(13)
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
                                    subfilter: vec![
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
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M"));
}





#[test]
fn test_filters_equations(){
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
                                right: Operand::Number(13)
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
                                    subfilter: vec![
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
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) }]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&x=123"));
}






#[test]
fn test_filters_equations2(){
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
                                right: Operand::Number(13)
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
                                    subfilter: vec![
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
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) },
                        Equation { left: Operand::Column("title".to_owned()), right: Operand::Column("engr".to_owned()) }
                        ]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
}






#[test]
fn test_filters2_equations2(){
    println!("{:#?}", params("age=lt.13&student=eq.true|gender=eq.M&age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
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
                                right: Operand::Number(13)
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
                                    subfilter: vec![
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
                                            subfilter: vec![
                                                    Filter {
                                                        connector: Some(Connector::AND),
                                                        condition: Condition {
                                                            left: Operand::Column(
                                                                "age".to_owned()
                                                            ),
                                                            equality: Equality::LT,
                                                            right: Operand::Number(13)
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
                                                                subfilter: vec![
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
                                                            },
                                                            
                                                        ]
                                                    }
                                            ]
                                        }
                                    ]
                                },
                                
                            ]
                        },
                    ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) },
                        Equation { left: Operand::Column("title".to_owned()), right: Operand::Column("engr".to_owned()) }
                        ]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
}









