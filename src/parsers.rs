use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map_res,
    sequence::terminated,
    IResult,
};
use std::{num::ParseIntError, str::FromStr};

pub fn int<T: FromStr<Err = ParseIntError>>(input: &str) -> IResult<&str, T> {
    map_res(take_while1(|c: char| c.is_digit(10)), str::parse)(input)
}

pub fn take_until_and_consume(search_tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |input| terminated(take_until(search_tag), tag(search_tag))(input)
}
