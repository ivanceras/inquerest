## Inquerest

[![Build Status](https://travis-ci.org/ivanceras/inquerest.svg?branch=master)](https://travis-ci.org/ivanceras/inquerest)

A complex url parameter parser for rest filter queries

### Example

```
age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&page=20&page_size=100&x=123&y=456

```
Will resolve into

```rust
Query {
        filters: [
            Filter {
                connector: None,
                condition: Condition {
                    left: Column("age"),
                    equality: LT,
                    right: Number(13)
                },
                subfilter: [
                    Filter {
                        connector: Some(AND),
                        condition: Condition {
                            left: Column("student"),
                            equality: EQ,
                            right: Boolean(true)
                        },
                        subfilter: [
                            Filter {
                                connector: Some(OR),
                                condition: Condition {
                                    left: Column("gender"),
                                    equality: EQ,
                                    right: Column("M")
                                },
                            }
                        ]
                    }
                ]
            }
        ],
        group_by: [
            Function(
                Function {
                    function: "sum",
                    params: [Column("age")]
                }
            ),
            Column("grade"),
            Column("gender")
        ],
        having: [
            Filter {
                connector: None,
                condition: Condition {
                    left: Function(
                        Function {
                            function: "min",
                            params: [Column("age")]
                        }
                    ),
                    equality: GT,
                    right: Number(13)
                },
            }
        ],
        order_by: [
            Order {
                column: "age",
                direction: DESC
            },
            Order {
                column: "height",
                direction: ASC
            }
        ],
        page: Some(20),
        page_size: Some(100),
        equations: [
            Equation {
                left: Column("x"),
                right: Number(123)
            },
            Equation {
                left: Column("y"),
                right: Number(456)
            }
        ]
    }

```


Inspired by [postgrest](https://github.com/begriffs/postgrest) [filter expressions](https://github.com/begriffs/postgrest/wiki/Routing)

##Similar projects

* [rustless/queryst](https://github.com/rustless/queryst)

