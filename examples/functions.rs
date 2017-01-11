extern crate inquerest;

use inquerest::*;

fn main() {
	println!("function:{:?}",function("min(age)".as_bytes()));	
	println!("condition:{:?}",condition("min(grade)=gte.lee".as_bytes()));
}

