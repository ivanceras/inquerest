use inquerest::{
    self,
    restq::ast::TableLookup,
};

fn main() {
    let url = "/person?age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
    let query = inquerest::parse_query(url);
    println!("query: {:#?}", query);
    println!(
        "sql query: {}",
        query.unwrap().into_sql_statement(None).unwrap().to_string()
    );

    let filter = "age=lt.42&(student=eq.true|gender=eq.'M')&group_by=sum(age),grade,gender&having=min(age)=gt.42&order_by=age.desc,height.asc&page=20&page_size=100";
    let result = inquerest::parse_filter(filter);
    println!("filter_only: {:#?}", result);
}
