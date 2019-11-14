extern crate inquerest;

extern crate nom;

use inquerest::*;
use nom::IResult;


//#[test]
fn test_limit_only(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
		Select{
			range: Some(
				Range::Limit(
					Limit {
						limit: 5,
						offset: None
					}
				)
			),
			..Default::default()
		}),
	inquerest::query("limit=5".as_bytes()))
}
//#[test]
fn test_limit_and_offset(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
		Select{
			range: Some(
				Range::Limit(
					Limit {
						limit: 5,
						offset: Some(10)
					}
				)
			),
			..Default::default()
		}),
	inquerest::query("limit=5&offset=10".as_bytes()))
}

#[test]
fn test_filters(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![ ]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Column("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M".as_bytes()));
}


#[test]
fn test_filter_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None}, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
                        ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&(student=eq.true|gender=eq.M)&order_by=age.desc,height.asc".as_bytes()));
}


#[test]
fn test_filter_groupby_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Column("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&order_by=age.desc,height.asc".as_bytes()));
}




#[test]
fn test_equations_filter_groupby_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                    ]
                                },
                                
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Value("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123f64) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456f64) }
                    ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&order_by=age.desc,height.asc&x=123&y=456".as_bytes()));
}



#[test]
fn test_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    order_by: vec![Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }],
                    ..Default::default()
                }
            )
        
        , query("order_by=height.asc".as_bytes()));
}
#[test]
fn test_orderby_nullsfirst(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    order_by: vec![Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: Some(NullsWhere::FIRST) }],
                    ..Default::default()
                }
            )
        
        , query("order_by=height.asc.nullsfirst".as_bytes()));
}
#[test]
fn test_orderby2(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    order_by: vec![
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None },
                        Order { operand: Operand::Column("grade".to_owned()), direction: Some(Direction::DESC), nulls_where: None }
                        ],
                    ..Default::default()
                }
            )
        
        , query("order_by=height.asc,grade.desc".as_bytes()));
}


#[test]
fn test_groupby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    group_by: vec![Operand::Column("height".to_owned())],
                    ..Default::default()
                }
            )
        
        , query("group_by=height".as_bytes()));
}

#[test]
fn test_groupby2(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    group_by: vec![
                        Operand::Function(Function { 
                                            function: "avg".to_owned(), 
                                            params: vec![Operand::Column("grade".to_owned())] 
                                }), 
                        Operand::Column("height".to_owned())
                    ],
                    ..Default::default()
                }
            )
        
        , query("group_by=avg(grade),height".as_bytes()));
}



#[test]
fn test_groupby_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    order_by: vec![
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None },
                        Order { operand: Operand::Column("grade".to_owned()), direction: Some(Direction::DESC), nulls_where: None }
                        ],
                    group_by: vec![
                       Operand::Function(Function { 
                                    function: "avg".to_owned(), 
                                    params: vec![Operand::Column("grade".to_owned())] 
                                }),
                        Operand::Column("height".to_owned())
                    ],
                    ..Default::default()
                }
            )
        
        , query("group_by=avg(grade),height&order_by=height.asc,grade.desc".as_bytes()));
}





#[test]
fn test_equations_filter_groupby_having_orderby(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Value("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                                        right: Operand::Number(13f64) 
                                    }, 
                                sub_filters: vec![] 
                            }
                        ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123f64) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456f64) }
                    ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&x=123&y=456".as_bytes()));
}





#[test]
fn test_equations_filter_groupby_having_orderby_page(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Value("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                                        right: Operand::Number(13f64) 
                                    }, 
                                sub_filters: vec![] 
                            }
                        ],
                    range: Some(Range::Page( Page{ page: 20, page_size:100 } )),
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123f64) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456f64) }
                    ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&page=20&page_size=100&x=123&y=456".as_bytes()));
}




#[test]
fn test_equations_filter_groupby_having_orderby_limit(){
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![ ]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Value("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                                        right: Operand::Number(13f64) 
                                    }, 
                                sub_filters: vec![] 
                            }
                        ],
                    range: Some(Range::Limit( Limit{ limit: 100, offset: Some(25) } )),
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123f64) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456f64) }
                    ],
                    ..Default::default()
                }
            )
        
        , query("age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&limit=100&offset=25&x=123&y=456".as_bytes()));
}






#[test]
fn test_equations_from_join_filter_groupby_having_orderby_limit(){
    
    println!("{:#?}",query("from=bazaar.person,student&left_join=person_student&on=student.id=person.student_id&age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&limit=100&offset=25&x=123&y=456".as_bytes()));
    
    assert_eq!(
        	IResult::Done("".as_bytes(), 
                Select {
                    from: vec![
                            Operand::Column(
                                "bazaar.person".to_owned()
                            ),
                            Operand::Column(
                                "student".to_owned()
                            )
                        ],
                        join: vec![
                            Join {
                                modifier: Some(
                                    Modifier::LEFT
                                ),
                                join_type: None,
                                table: Operand::Column(
                                    "person_student".to_owned()
                                ),
                                column1: vec![
                                    "student.id".to_owned()
                                ],
                                column2: vec![
                                    "person.student_id".to_owned()
                                ]
                            }
                        ],
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13f64)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![ ]
                                },
                                Filter {
                                    connector: Some(
                                        Connector::OR
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("gender".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Value("M".to_owned())
                                    },
                                    sub_filters: vec![]
                                }
                                
                            ]
                        }
                    ],
                    order_by: vec![
                        Order { operand: Operand::Column("age".to_owned()), direction: Some(Direction::DESC), nulls_where: None }, 
                        Order { operand: Operand::Column("height".to_owned()), direction: Some(Direction::ASC), nulls_where: None }
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
                                        right: Operand::Number(13f64) 
                                    }, 
                                sub_filters: vec![] 
                            }
                        ],
                    range: Some(Range::Limit( Limit{ limit: 100, offset: Some(25) } )),
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123f64) }, 
                        Equation { left: Operand::Column("y".to_owned()), right: Operand::Number(456f64) }
                    ],
                    ..Default::default()
                }
            )
        
        , query("from=bazaar.person,student&left_join=person_student&on=student.id=person.student_id&age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&limit=100&offset=25&x=123&y=456".as_bytes()));
}


#[test]
fn test_one_equation_only(){
    let arg = "focused_record=f5f031e2-0d3d-11e6-ae81-1c6f65c301cc";
    let result = parse(arg);
    assert!(result.is_ok())
}


#[test]
fn test_valid_operand_qouted(){
   let arg = "focused_record=[85ea7227-e31e-41af-955e-0513177ddb9a]";
    let result = parse(arg);
    println!("result: {:?}", result);
    assert!(result.is_ok())
}

#[test]
fn test_focused(){
    let arg = "focused=0";
    let result = parse(arg);
    assert!(result.is_ok())
}

