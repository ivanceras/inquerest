extern crate inquerest;

use inquerest::*;

fn main() {
	println!("{:?}",operand("description".as_bytes()));
	println!("{:?}",boolean("true".as_bytes()));
	println!("column name{:?}",column("description".as_bytes()));
	println!("column name{:?}",column("product.description".as_bytes()));
	println!("{:?}",function("sum(total)".as_bytes()));
	println!("eq: {:?}",equality("eq".as_bytes()));
	println!("neq: {:?}",equality("neq".as_bytes()));
	println!("lt: {:?}",equality("lt".as_bytes()));
	println!("lte: {:?}",equality("lte".as_bytes()));	
	println!("lteee: {:?}",equality("lteee".as_bytes()));	
	println!("gt: {:?}",equality("gt".as_bytes()));	
	println!("{:?}",equality("gte".as_bytes()));	
	println!("in: {:?}",equality("in".as_bytes()));	
	println!("not_in: {:?}",equality("not_in".as_bytes()));	
	println!("is_not: {:?}",equality("is_not".as_bytes()));	
	println!("like:{:?}",equality("like".as_bytes()));	
	println!("function:{:?}",function("min(age)".as_bytes()));	
	println!("condition:{:?}",condition("age=lt.13".as_bytes()));	
	println!("condition:{:?}",condition("(age=lt.13)".as_bytes()));	
	println!("direction:{:?}",direction("asc".as_bytes()));	
	println!("direction:{:?}",direction("desc".as_bytes()));	
	println!("order:{:?}",order("age.desc".as_bytes()));
	println!("order:{:?}",order("height.asc".as_bytes()));
	println!("connector:{:?}",connector("|".as_bytes()));
	println!("connector:{:?}",connector("&".as_bytes()));
	println!("\n filter1: {:?}",filter("student=eq.true".as_bytes()));
	println!("\n filter1: {:?}",filter("(student=eq.true)".as_bytes()));
	println!("\n filter1: {:?}",filter("((student=eq.true))".as_bytes()));
	println!("\n filter2: {:?}",filter("student=eq.true|gender=eq.M".as_bytes()));
	println!("\n filter2: {:?}",filter("(student=eq.true&age=lt.13)".as_bytes()));
	println!("\n filter3: {:?}",filter("(student=eq.true)&(gender=eq.M)".as_bytes()));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true".as_bytes()));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true&age=lt.13".as_bytes()));
	println!("\n filter5: {:?}",filter("(student=eq.true)|(student=eq.true)".as_bytes()));
	println!("\n filter6: {:?}",filter("(student=eq.true)|(student=eq.true&age=lt.13)".as_bytes()));
	println!("\n filter6: {:?}",filter("(student=eq.true|student=eq.true)&age=lt.13)".as_bytes()));
	println!("\n filter7: {:#?}",filter("(student=eq.true)|(student=eq.true)&(age=lt.13)".as_bytes()));
	
	assert_eq!(condition("age=lt.13".as_bytes()), condition("(age=lt.13)".as_bytes()))
	
}

