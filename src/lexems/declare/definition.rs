// Example:
//
// declare root
// {
// 	uint ConfigRevision 504
//
// OR (with or without spaces(' ', '\t', '\r', '\n'))
//
// declare root{..
//
// declare root {}
//
// declare  root   {}

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{multispace0, multispace1},
    sequence::delimited,
    IResult,
};

// from vpn_config_parser import parse_hash_comment
pub fn parse_definition_name(input: &str) -> IResult<&str, &str> {
    let (rest, _) = tag("declare")(input)?;
    let (rest, result) = delimited(
        multispace1,
        take_while(|char| !matches!(char, ' ' | '\r' | '\n' | '\t' | '{')),
        multispace0,
    )(rest)?;

    Ok((rest, result))
}

#[test]
fn test_parsing_declare_definition() {
    assert_eq!(
        parse_definition_name("declare some_name {"),
        Ok(("{", "some_name"))
    );

    assert_eq!(
        parse_definition_name("declare    some_name  \n\r\t     {some_stuff"),
        Ok(("{some_stuff", "some_name"))
    );

    assert_eq!(
        parse_definition_name("declare  \n\t\r  some_name  \n\r\t     {some_stuff"),
        Ok(("{some_stuff", "some_name"))
    );

    assert_eq!(
        parse_definition_name("declare some_name {"),
        Ok(("{", "some_name"))
    );

    assert_eq!(
        parse_definition_name("declare some_name{"),
        Ok(("{", "some_name"))
    );
}
