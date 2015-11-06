#![feature(plugin)]
#![plugin(peg_syntax_ext)]
use arithmetic::*;

#[derive(Debug)]
pub struct Equation{
    left: String,
    right: String
}


peg! arithmetic(r#"
use super::Equation;

#[pub]
expression -> i64
	= sum
sum -> i64
	= l:product "+" r:product { l+r }
	/ product
product -> i64
	= l:atom "*" r:atom { l*r }
	/ atom
atom -> i64
	= number
	/ "(" v:sum ")" { v }
number -> i64
	= [0-9]+ { match_str.parse().unwrap() }
#[pub]
name -> String
  = [a-zA-Z0-9_]+ { match_str.to_string() }

#[pub]
keyval -> (i64, i64)
    = k:number ":" + v:number { (k, v) }
#[pub]
equation -> Equation
    = l:name "=" + r:name { Equation{left:l, right:r} }
"#);



fn main() {
	println!("{:?}",expression("1+1"));
	println!("{:?}",expression("5*5"));
	println!("{:?}",expression("222+3333"));
	println!("{:?}",expression("2+3*4"));
	println!("{:?}",expression("(2+2)*3"));
	println!("{:?}",expression("(22+)+1"));
	println!("{:?}",expression("1++1"));
	println!("{:?}",expression("3)+1"));
	println!("{:?}",name("lee"));
	println!("{:?}",keyval("1:2"));
	println!("{:?}",equation("lee=2"));
}