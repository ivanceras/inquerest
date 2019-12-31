# inquerest

Inquerest can parse complex url query into a SQL abstract syntax tree.

Example this url:
```rust
 /person?age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100
```
will be parsed into:

```rust
Select {
        from_table: FromTable {
            from: Table {
                name: "person",
            },
            join: None,
        },
        filter: Some(
            BinaryOperation(
                BinaryOperation {
                    left: BinaryOperation(
                        BinaryOperation {
                            left: Column(
                                Column {
                                    name: "age",
                                },
                            ),
                            operator: Lt,
                            right: Value(
                                Number(
                                    42.0,
                                ),
                            ),
                        },
                    ),
                    operator: And,
                    right: Nested(
                        BinaryOperation(
                            BinaryOperation {
                                left: BinaryOperation(
                                    BinaryOperation {
                                        left: Column(
                                            Column {
                                                name: "student",
                                            },
                                        ),
                                        operator: Eq,
                                        right: Value(
                                            Bool(
                                                true,
                                            ),
                                        ),
                                    },
                                ),
                                operator: Or,
                                right: BinaryOperation(
                                    BinaryOperation {
                                        left: Column(
                                            Column {
                                                name: "gender",
                                            },
                                        ),
                                        operator: Eq,
                                        right: Value(
                                            String(
                                                "M",
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
        group_by: Some(
            [
                Function(
                    Function {
                        name: "sum",
                        params: [
                            Column(
                                Column {
                                    name: "age",
                                },
                            ),
                        ],
                    },
                ),
                Column(
                    Column {
                        name: "grade",
                    },
                ),
                Column(
                    Column {
                        name: "gender",
                    },
                ),
            ],
        ),
        having: Some(
            BinaryOperation(
                BinaryOperation {
                    left: Function(
                        Function {
                            name: "min",
                            params: [
                                Column(
                                    Column {
                                        name: "age",
                                    },
                                ),
                            ],
                        },
                    ),
                    operator: Gt,
                    right: Value(
                        Number(
                            42.0,
                        ),
                    ),
                },
            ),
        ),
        projection: None,
        order_by: Some(
            [
                Order {
                    expr: Column(
                        Column {
                            name: "age",
                        },
                    ),
                    direction: Some(
                        Desc,
                    ),
                },
                Order {
                    expr: Column(
                        Column {
                            name: "height",
                        },
                    ),
                    direction: Some(
                        Asc,
                    ),
                },
            ],
        ),
        range: Some(
            Page(
                Page {
                    page: 20,
                    page_size: 100,
                },
            ),
        ),
    }
```
Which translate to the sql statement:
```sql
SELECT * FROM person WHERE age < 42 AND (student = true OR gender = 'M') GROUP BY sum(age), grade, gender HAVING min(age) > 42 ORDER BY age DESC, height ASC LIMIT 100 OFFSET 1900 ROWS
```
Note: However, you don't want to convert to the sql statement directly to avoid sql injection
attack. You need to validate the tables and columns if it is allowed to be accessed by the
user. You also need to extract the values yourself and supply it as a parameterized value into
your ORM.

##### Please support this project:
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)

License: MIT
