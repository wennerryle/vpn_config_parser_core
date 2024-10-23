// Examples, (I'd like to do this with spaces between values):
// bool Disabled false
// byte Key V2arjN0mVGd9457Zow6q4uJmBe0=
// string LocalHostname nbb-reports

use nom::{
    bytes::complete::take_while,
    character::complete::{multispace0, space1},
    sequence::preceded,
    IResult,
};
use pyo3::pyclass;

#[derive(PartialEq, Debug, Clone)]
#[pyclass(module = "vpn_config_parser", get_all)]
pub struct Constant {
    r#type: String,
    key: String,
    value: String,
}

pub fn parse_constant(input: &str) -> IResult<&str, Constant> {
    fn take_until_delimiter(input: &str) -> IResult<&str, &str> {
        take_while(|char: char| !matches!(char, ' ' | '\r' | '\n' | '#' | '}'))(input)
    }

    let (rest, _) = multispace0(input)?;

    let (rest, r#type) = take_until_delimiter(rest)?;
    let (rest, key) = preceded(space1, take_until_delimiter)(rest)?;
    let (rest, value) = preceded(space1, take_until_delimiter)(rest)?;

    let (rest, _) = multispace0(rest)?;

    Ok((
        rest,
        Constant {
            r#type: r#type.to_string(),
            key: key.to_string(),
            value: value.to_string(),
        },
    ))
}

#[test]
fn test_constant_parsing() {
    assert_eq!(
        parse_constant(&"input someKey someValue"),
        Ok((
            "",
            Constant {
                r#type: "input".to_string(),
                key: "someKey".to_string(),
                value: "someValue".to_string(),
            }
        ))
    );

    assert_eq!(
        parse_constant(&"input someKey someValue rest_things"),
        Ok((
            "rest_things",
            Constant {
                r#type: "input".to_string(),
                key: "someKey".to_string(),
                value: "someValue".to_string(),
            }
        ))
    );

    assert_eq!(
        parse_constant(&"input   someKey  someValue rest_things\r\n"),
        Ok((
            "rest_things\r\n",
            Constant {
                r#type: "input".to_string(),
                key: "someKey".to_string(),
                value: "someValue".to_string(),
            }
        ))
    );
}
