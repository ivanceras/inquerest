extern crate inquerest;

use inquerest::*;

#[test]
fn test_order1(){
    assert_eq!(
        Ok(Order{
            column: "age".to_owned(), 
            direction: Direction::DESC, 
        }),
        order("age.desc"));
}