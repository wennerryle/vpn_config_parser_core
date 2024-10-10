// Example:
//
// # Software Configuration File
// # ---------------------------
// #
// # You may edit this file when the VPN Server / Client / Bridge program is not running.
// #
// # In prior to edit this file manually by your text editor,
// # shutdown the VPN Server / Client / Bridge background service.
// # Otherwise, all changes will be lost.
// #
//
// Or somewhere in code:
// bool Disabled false # some comment

use nom::{
    bytes::complete::{tag, take_while},
    sequence::preceded,
    IResult,
};

// from vpn_config_parser import parse_hash_comment
#[rustfmt::skip]
pub fn parse_hash_comment(input: &str) -> IResult<&str, &str> {
    let (rest, result) = preceded(
        tag("#"),
        take_while(|char| char != '\n' && char != '\r')
    )(input)?;

    let rest = rest.trim_start();

    Ok((rest, result))
}

#[test]
fn test_hash_comment_parsing() {
    assert_eq!(parse_hash_comment("#comment"), Ok(("", "comment")));
    assert_eq!(parse_hash_comment("#comment\r"), Ok(("", "comment")));
    assert_eq!(parse_hash_comment("# comment"), Ok(("", " comment")));
    assert_eq!(parse_hash_comment("#comment\r\n"), Ok(("", "comment")));

    assert_eq!(
        parse_hash_comment("#comment\r\nsome_check"),
        Ok(("some_check", "comment"))
    );

    assert_eq!(
        parse_hash_comment("#comment\nsome_check"),
        Ok(("some_check", "comment"))
    );
}
