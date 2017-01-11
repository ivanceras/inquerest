extern crate inquerest;

use inquerest::*;

fn main() {
	println!("\n complex1: {:?}",filter("(student=eq.true)|(student=eq.true)&(age=lt.13)".as_bytes()));
	println!("\n complex1: {:?}",filter("(student=eq.true)|student=eq.true&age=lt.13".as_bytes()));
	println!("\n complex2: {:?}",filter("student=eq.true|student=eq.true&age=lt.13".as_bytes()));
	println!("\n complex_filter: {:?}",filter("(student=eq.true|student=eq.true)".as_bytes()));
	println!("\n complex_filter: {:?}",filter("(student=eq.true|student=eq.true)&age=lt.13".as_bytes()));
	
	
}

