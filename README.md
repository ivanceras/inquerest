## Inquerest

[![Latest Version](https://img.shields.io/crates/v/inquerest.svg?style=flat-square)](https://crates.io/crates/inquerest)
[![Build Status](https://img.shields.io/travis/ivanceras/inquerest.svg?style=flat-square)](https://travis-ci.org/ivanceras/inquerest)
[![Coverage Status](https://img.shields.io/coveralls/ivanceras/inquerest.svg?style=flat-square)](https://coveralls.io/github/ivanceras/inquerest)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)
[![Build status](https://ci.appveyor.com/api/projects/status/gu8t6gc5uxjfakge/branch/master?svg=true&style=flat-square)](https://ci.appveyor.com/project/ivanceras/inquerest/branch/master)

A complex url parameter parser for rest filter queries


### Example

```
age=lt.13&(student=eq.true|gender=eq.M)&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&page=20&page_size=100&x=123&y=456

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
        range: Some(Page( 
			Page{ page: 20, page_size:100 } 
		)),
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
## More examples in

* [examples](https://github.com/ivanceras/inquerest/tree/master/examples)
* [tests](https://github.com/ivanceras/inquerest/tree/master/tests)

Inspired by [Postgrest](https://github.com/begriffs/postgrest)  [Filter expressions](https://github.com/begriffs/postgrest/wiki/Routing)

##Similar projects

* [rustless/queryst](https://github.com/rustless/queryst)


If you like this library, please consider supporting the project on [Gratipay](https://gratipay.com/~ivanceras/). 

