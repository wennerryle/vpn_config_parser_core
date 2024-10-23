mod constant;
mod definition;

pub use constant::{parse_constant, Constant};
use definition::parse_definition_name;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;
use pyo3::{pyclass, pymethods, PyResult};

use crate::lexems::hash_comment::parse_hash_comments0;

#[pyclass(module = "vpn_config_parser")]
#[derive(Default, Debug, PartialEq)]
pub struct Declare {
    pub name: String,
    pub constants: Vec<Constant>,
    pub declarations: Vec<Declare>,
}

#[pymethods]
impl Declare {
    #[getter]
    fn get_name(&self) -> PyResult<&str> {
        Ok(&self.name)
    }
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
