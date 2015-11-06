#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg! memo(r#"
#[cache]
rule -> &'input str
    = [a-z]+ { match_str }
#[pub]
parse -> String
    = rule '+' rule { (format!("hello{}", rule)) }
    / rule ' ' rule { ("hi".to_owned()) }
"#);

fn main() {
	//assert_eq!(memo::parse("abc zzz"), Ok(()));
	println!("{:?}", memo::parse("abc zzz"));
	println!("{:?}", memo::parse("abc+zzz"));
}