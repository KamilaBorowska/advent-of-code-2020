use crate::parsers::int;
use crate::Solution;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    sequence::tuple,
    Finish,
};
use std::error::Error;

struct Rule<'a> {
    first: usize,
    second: usize,
    required_char: char,
    password: &'a str,
}

impl<'a> Rule<'a> {
    fn parse(line: &'a str) -> Result<Self, nom::error::Error<&'a str>> {
        let (password, (first, _, second, _, required_char, _)) =
            tuple((int, char('-'), int, char(' '), anychar, tag(": ")))(line).finish()?;
        Ok(Rule {
            first,
            second,
            required_char,
            password,
        })
    }
}

fn count_valid_passwords(
    input: &str,
    f: impl Fn(Rule) -> bool,
) -> Result<String, Box<dyn Error + '_>> {
    let mut valid_passwords = 0;
    for line in input.lines() {
        let rule = Rule::parse(line)?;
        valid_passwords += usize::from(f(rule))
    }
    Ok(valid_passwords.to_string())
}

pub(super) const DAY2: Solution = Solution {
    part1: |input| {
        count_valid_passwords(input, |rule| {
            let char_count = rule
                .password
                .chars()
                .filter(|&c| rule.required_char == c)
                .count();
            (rule.first..=rule.second).contains(&char_count)
        })
    },
    part2: |input| {
        count_valid_passwords(
            input,
            |Rule {
                 first,
                 second,
                 required_char,
                 password,
             }| {
                (password.chars().nth(first - 1) == Some(required_char))
                    ^ (password.chars().nth(second - 1) == Some(required_char))
            },
        )
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY2.part1,
        empty: "" => 0,
        example: "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc" => 2,
        input: 607,
    );
    test!(
        DAY2.part2,
        empty: "" => 0,
        example: "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc" => 1,
        input: 321,
    );
}
