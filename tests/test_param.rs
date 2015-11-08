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
fn test_filter_orderby(){
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
}


#[test]
fn test_filter_groupby_orderby(){
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
                    group_by: vec![
                        Group { field: Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }) 
                        }, 
                        Group { field: Operand::Column("grade".to_owned()) }, 
                        Group { field: Operand::Column("gender".to_owned()) }
                    ],
                    equations: vec![]
                }
            )
        
        , params("(age=lt.13)&(student=eq.true)|(gender=eq.M)&group_by=sum(age),grade,gender&order_by=age.desc,height.asc"));
}




#[test]
fn test_equations_filter_groupby_orderby(){
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
                    group_by: vec![
                        Group { field: Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }) 
                        }, 
                        Group { field: Operand::Column("grade".to_owned()) }, 
                        Group { field: Operand::Column("gender".to_owned()) }
                    ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Column("123".to_owned()) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Column("456".to_owned()) }]
                }
            )
        
        , params("(age=lt.13)&(student=eq.true)|(gender=eq.M)&group_by=sum(age),grade,gender&order_by=age.desc,height.asc&x=123&y=456"));
}



#[test]
fn test_orderby(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![],
                    equations: vec![]
                }
            )
        
        , params("order_by=height.asc"));
}
#[test]
fn test_orderby2(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC },
                        Order { column: "grade".to_owned(), direction: Direction::DESC }
                        ],
                    group_by: vec![],
                    equations: vec![]
                }
            )
        
        , params("order_by=height.asc,grade.desc"));
}


#[test]
fn test_groupby(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![],
                    order_by: vec![],
                    group_by: vec![Group { field: Operand::Column("height".to_owned()) }],
                    equations: vec![]
                }
            )
        
        , params("group_by=height"));
}

#[test]
fn test_groupby2(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![],
                    order_by: vec![],
                    group_by: vec![
                        Group { field: Operand::Function(Function { 
                                                            function: "avg".to_owned(), 
                                                            params: vec![Operand::Column("grade".to_owned())] 
                                                        }) 
                                },
                        Group { field: Operand::Column("height".to_owned()) }
                    ],
                    equations: vec![]
                }
            )
        
        , params("group_by=avg(grade),height"));
}



#[test]
fn test_groupby_orderby(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC },
                        Order { column: "grade".to_owned(), direction: Direction::DESC }
                        ],
                    group_by: vec![
                        Group { field: Operand::Function(Function { 
                                                            function: "avg".to_owned(), 
                                                            params: vec![Operand::Column("grade".to_owned())] 
                                                        }) 
                                },
                        Group { field: Operand::Column("height".to_owned()) }
                    ],
                    equations: vec![]
                }
            )
        
        , params("group_by=avg(grade),height&order_by=height.asc,grade.desc"));
}
