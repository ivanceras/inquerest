extern crate inquerest;

use inquerest::*;

/*
fn main1(){ 
    println!("\n filter1: {:?}",filter("student=eq.true"));
	println!("\n filter1: {:?}",filter("(student=eq.true)"));
	println!("\n filter1: {:?}",filter("((student=eq.true))"));
	println!("\n filter2: {:?}",filter("student=eq.true|gender=eq.M"));
	println!("\n filter2: {:?}",filter("(student=eq.true&age=lt.13)"));
	println!("\n filter3: {:?}",filter("(student=eq.true)&(gender=eq.M)"));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true"));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true&age=lt.13"));
	println!("\n filter5: {:?}",filter("(student=eq.true)|(student=eq.true)"));
	println!("\n filter6: {:?}",filter("(student=eq.true)|(student=eq.true&age=lt.13)"));
	println!("\n filter6: {:?}",filter("(student=eq.true|student=eq.true)&age=lt.13)"));
	println!("\n filter7: {:#?}",filter("(student=eq.true)|(student=eq.true)&(age=lt.13)"));
}
*/

#[test]
fn test_simple(){
    assert_eq!(
        Ok(Filter{
                connector: None,
                condition: Condition{left:Operand::Column("student".to_owned()),
                                    equality:Equality::EQ,
                                    right: Operand::Column("true".to_owned())
                            },
                subfilter: vec![]
                }),
        filter("student=eq.true"))
}

#[test]
fn and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13"))
}

#[test]
fn and_and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               },
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                           left:  Operand::Column("grade".to_owned()), 
                                           equality: Equality::GTE, 
                                           right:  Operand::Column("3".to_owned()) 
                                       }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13&grade=gte.3"))
}

#[test]
fn or_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13"))
}

#[test]
fn or_or_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               },
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                           left:  Operand::Column("grade".to_owned()), 
                                           equality: Equality::GTE, 
                                           right:  Operand::Column("3".to_owned()) 
                                       }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13|grade=gte.3"))
}


#[test]
fn and_or_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               },
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                           left:  Operand::Column("grade".to_owned()), 
                                           equality: Equality::GTE, 
                                           right:  Operand::Column("3".to_owned()) 
                                       }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13|grade=gte.3"))
}


#[test]
fn or_and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Column("true".to_owned()) }, 
                       subfilter: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Column("13".to_owned()) 
                                           }, 
                                   subfilter: vec![] 
                               },
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                           left:  Operand::Column("grade".to_owned()), 
                                           equality: Equality::GTE, 
                                           right:  Operand::Column("3".to_owned()) 
                                       }, 
                                   subfilter: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13&grade=gte.3"))
}

#[test]
fn equal_filter(){
    assert_eq!(filter("student=eq.true"),filter("((student=eq.true))"))
}

#[test]
fn equal_and_and(){
    assert_eq!(filter("(student=eq.true)&(age=lt.13)&(grade=gte.3)"),filter("student=eq.true&age=lt.13&grade=gte.3"))
}

#[test]
fn equal_or_or(){
    assert_eq!(filter("(student=eq.true)|(age=lt.13)|(grade=gte.3)"),filter("student=eq.true|age=lt.13|grade=gte.3"))
}


#[test]
fn equal_enclosed_or(){
    assert_eq!(filter("(student=eq.true|age=lt.13)"),filter("student=eq.true|age=lt.13"))
}

#[test]
fn equal_enclosed_and(){
    assert_eq!(filter("(student=eq.true&age=lt.13)"),filter("student=eq.true&age=lt.13"))
}


#[test]
#[should_panic]
fn equal_enclosed_complex(){
    assert_eq!(filter("(student=eq.true|age=lt.13)&gt=gt.3"),filter("student=eq.true|age=lt.13&gt=gt.3"))
}