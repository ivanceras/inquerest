#[test]
fn test1() {
    let url = "/person?age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
    let query = inquerest::parse_query(url);
    println!("query: {:#?}", query);
    assert_eq!(
        "SELECT * FROM person WHERE age < 42 AND (student = true OR gender = 'M') GROUP BY sum(age), grade, gender HAVING min(age) > 42 ORDER BY age DESC, height ASC LIMIT 100 OFFSET 1900 ROWS",
        query.unwrap().into_sql_statement(None).unwrap().to_string()
    );
}
