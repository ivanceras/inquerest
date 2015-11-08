extern crate inquerest;

use inquerest::*;


#[test]
fn test_filters(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![],
                    group_by: vec![],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M"));
}


#[test]
fn test_filter_orderby(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("age=lt.13&(student=eq.true|gender=eq.M)&order_by=age.desc,height.asc"));
}


#[test]
fn test_filter_groupby_orderby(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![
                            Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }) 
                            , 
                            Operand::Column("grade".to_owned()), 
                            Operand::Column("gender".to_owned())
                    ],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&order_by=age.desc,height.asc"));
}




#[test]
fn test_equations_filter_groupby_orderby(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![
                        Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }
                            ), 
                       Operand::Column("grade".to_owned()), 
                       Operand::Column("gender".to_owned()) 
                    ],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456) }]
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&order_by=age.desc,height.asc&x=123&y=456"));
}



#[test]
fn test_orderby(){
    assert_eq!(
        	Ok(
                Query {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("order_by=height.asc"));
}
#[test]
fn test_orderby2(){
    assert_eq!(
        	Ok(
                Query {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC },
                        Order { column: "grade".to_owned(), direction: Direction::DESC }
                        ],
                    group_by: vec![],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("order_by=height.asc,grade.desc"));
}


#[test]
fn test_groupby(){
    assert_eq!(
        	Ok(
                Query {
                    filters: vec![],
                    order_by: vec![],
                    group_by: vec![Operand::Column("height".to_owned())],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("group_by=height"));
}

#[test]
fn test_groupby2(){
    assert_eq!(
        	Ok(
                Query {
                    filters: vec![],
                    order_by: vec![],
                    group_by: vec![
                        Operand::Function(Function { 
                                            function: "avg".to_owned(), 
                                            params: vec![Operand::Column("grade".to_owned())] 
                                }), 
                        Operand::Column("height".to_owned())
                    ],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("group_by=avg(grade),height"));
}



#[test]
fn test_groupby_orderby(){
    assert_eq!(
        	Ok(
                Query {
                    filters: vec![],
                    order_by: vec![
                        Order { column: "height".to_owned(), direction: Direction::ASC },
                        Order { column: "grade".to_owned(), direction: Direction::DESC }
                        ],
                    group_by: vec![
                       Operand::Function(Function { 
                                    function: "avg".to_owned(), 
                                    params: vec![Operand::Column("grade".to_owned())] 
                                }),
                        Operand::Column("height".to_owned())
                    ],
                    having: vec![],
                    page: None,
                    page_size: None,
                    equations: vec![]
                }
            )
        
        , query("group_by=avg(grade),height&order_by=height.asc,grade.desc"));
}





#[test]
fn test_equations_filter_groupby_having_orderby(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![
                        Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }
                            ), 
                       Operand::Column("grade".to_owned()), 
                       Operand::Column("gender".to_owned()) 
                    ],
                    having: vec![
                            Filter { connector: None, 
                                    condition: Condition { 
                                        left: Operand::Function(
                                                Function { 
                                                    function: "min".to_owned(), 
                                                    params: vec![Operand::Column("age".to_owned())] 
                                                }), 
                                        equality: Equality::GT, 
                                        right: Operand::Number(13) 
                                    }, 
                                subfilter: vec![] 
                            }
                        ],
                    page: None,
                    page_size: None,
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456) }]
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&x=123&y=456"));
}





#[test]
fn test_equations_filter_groupby_having_orderby_page(){
    assert_eq!(
        	Ok(
                Query {
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
                    order_by: vec![
                        Order { column: "age".to_owned(), direction: Direction::DESC }, 
                        Order { column: "height".to_owned(), direction: Direction::ASC }
                        ],
                    group_by: vec![
                        Operand::Function(
                                    Function { 
                                        function: "sum".to_owned(), 
                                        params: vec![Operand::Column("age".to_owned())] 
                                    }
                            ), 
                       Operand::Column("grade".to_owned()), 
                       Operand::Column("gender".to_owned()) 
                    ],
                    having: vec![
                            Filter { connector: None, 
                                    condition: Condition { 
                                        left: Operand::Function(
                                                Function { 
                                                    function: "min".to_owned(), 
                                                    params: vec![Operand::Column("age".to_owned())] 
                                                }), 
                                        equality: Equality::GT, 
                                        right: Operand::Number(13) 
                                    }, 
                                subfilter: vec![] 
                            }
                        ],
                    page: Some(20),
                    page_size: Some(100),
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456) }]
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&page=20&page_size=100&x=123&y=456"));
}


