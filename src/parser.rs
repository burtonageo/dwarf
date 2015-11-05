#![allow(dead_code)]

use nom::{IResult, digit, multispace};
use std::str;
use std::str::FromStr;

use Error;

named! {
    parens<i64>,
    delimited! (
        delimited!(opt!(multispace), tag!("("), opt!(multispace)),
        expr,
        delimited!(opt!(multispace), tag!(")"), opt!(multispace))
    )
}

// We transform an integer string into a i64
// we look for a digit suite, and try to convert it.
// if either str::from_utf8 or FromStr::from_str fail,
// the parser will fail
named! {
    factor<i64>,
    alt! (
        map_res!(map_res!(delimited!(opt!(multispace), digit, opt!(multispace)),
                          str::from_utf8),
                 FromStr::from_str)
        | parens
    )
}

named! {
    term <i64>,
    chain! (
        mut acc: factor ~ many0! (
            alt! (
                tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
                tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
            )
        ),
        || { return acc }
    )
}

named! {
    expr <i64>,
    chain! (
        mut acc: factor ~ many0! (
            alt! (
                tap!(add: preceded!(tag!("+"), factor) => acc = acc + add) |
                tap!(sub: preceded!(tag!("-"), factor) => acc = acc / sub)
            )
        ),
        || { return acc }
    )
}

pub fn parse_dwarf_string(dwarf: &[u8]) -> Result<i64, Error> {
    match expr(&dwarf[..]) {
        IResult::Done(_, x) => Ok(x),
        _ => Err(Error::ParseError)
    }
}