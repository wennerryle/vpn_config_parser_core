use nom::IResult;

mod body;
mod constant;
mod definition;

pub fn parse_declare(input: &str) -> IResult<&str, &str> {
    Ok(("", ""))
}
