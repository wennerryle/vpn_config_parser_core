mod constant;
mod definition;

pub use constant::{parse_constant, Constant};
use definition::parse_definition_name;
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

use crate::lexems::hash_comment::parse_hash_comments0;

use super::parse_hash_comment;

// #[pyclass]
#[derive(Default, Debug, PartialEq)]
pub struct Declare {
    pub name: String,
    pub constants: Vec<Constant>,
    pub declarations: Vec<Declare>,
}

pub fn parse_declare(input: &str) -> IResult<&str, Declare> {
    let (rest, definition_name) = parse_definition_name(input)?;

    println!("definition name parsed!");

    println!("rest: {rest} def_name: {definition_name}");
    let (rest, _) = char('{')(rest)?;
    println!("'{{' parsed");

    let (rest, constants) = many0(delimited(
        alt((parse_hash_comments0, multispace0)),
        parse_constant,
        alt((parse_hash_comments0, multispace0)),
    ))(rest)?;

    let (rest, declarations) = many0(parse_declare)(rest)?;

    let (rest, _) = char('}')(rest)?;
    println!("'}}' parsed");

    Ok((
        rest,
        Declare {
            declarations,
            constants,
            name: definition_name.to_string(),
        },
    ))
}

// nodemon -e rs --exec "cargo test -- lexems::declare::test_parse_declaration --show_output"
#[test]
fn test_parse_declaration() {
    let result = parse_declare("declare x_really_weird_things..{avavav dfg dfg#some comment}");
    match result {
        Ok(res) => println!("{res:?}"),
        Err(ref err) => {
            println!("{result:?}\n");
            if let nom::Err::Error(value) = err {
                println!("{value}")
            }
        }
    }
}
