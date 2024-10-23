mod constant;
mod definition;

pub use constant::{parse_constant, Constant};
use definition::parse_definition_name;
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

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
    let (rest, _) = parse_hash_comments0(input)?;
    let (rest, definition_name) = parse_definition_name(rest)?;

    let (rest, constants) = many0(delimited(
        opt(parse_hash_comments0),
        parse_constant,
        opt(parse_hash_comments0),
    ))(rest)?;

    let (rest, declarations) = many0(parse_declare)(rest)?;

    let (rest, _) = multispace0(rest)?; // if empty block

    let (rest, _) = char('}')(rest)?;

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
