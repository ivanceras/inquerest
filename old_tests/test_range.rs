extern crate inquerest;
extern crate nom;
use inquerest::*;
use nom::IResult;

#[test]
fn test_limit_only(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
				Range::Limit(
					Limit {
						limit: 5,
						offset: None
					}
				)
		),
	inquerest::range("limit=5".as_bytes()))
}

#[test]
fn test_limit_offset(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
				Range::Limit(
					Limit {
						limit: 5,
						offset: Some(100)
					}
				)
		),
	inquerest::range("limit=5&offset=100".as_bytes()))
}

#[test]
fn test_page(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
				Range::Page(
					Page {
						page: 5,
						page_size: 20
					}
				)
		),
	inquerest::range("page=5&page_size=20".as_bytes()))
}

#[test]
#[should_panic]
fn test_page_only(){
	assert_eq!(
		IResult::Done("".as_bytes(), 
				Range::Page(
					Page {
						page: 5,
						page_size: 0
					}
				)
		),
	inquerest::range("page=5".as_bytes()))
}
