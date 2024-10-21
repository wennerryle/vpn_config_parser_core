use std::io::{self, BufRead};

use lexems::{parse_constant, parse_hash_comments0};
use nom::{
    branch::alt, character::complete::multispace0, combinator::opt, multi::many0,
    sequence::delimited,
};
mod lexems;

fn main() {
    let stdin = io::stdin();

    let res = stdin
        .lock()
        .lines()
        .filter(Result::is_ok)
        .take_while(|re| !re.as_ref().unwrap().is_empty())
        .map(Result::unwrap)
        .reduce(|it, acc| it + "\n" + &acc);

    if let Some(input) = res {
        let parsed_contant = many0(delimited(
            alt((parse_hash_comments0, multispace0)),
            parse_constant,
            opt(parse_hash_comments0),
        ))(&input);

        println!("{parsed_contant:?}");
    }
}
