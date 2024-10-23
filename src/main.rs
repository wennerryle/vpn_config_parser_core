use std::fs;

use lexems::parse_declare;
mod lexems;

fn main() {
    let file = fs::read_to_string("example.config").expect("Not found file example.config");

    let parsed_constant = parse_declare(&file);

    // let parsed_contant = many0(delimited(
    //     opt(parse_hash_comments0),
    //     parse_constant,
    //     opt(parse_hash_comments0),
    // ))(&input);

    println!("{parsed_constant:?}");
}
