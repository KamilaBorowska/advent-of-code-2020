use crate::parsers::int;
use crate::Solution;
use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::eof,
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

pub(super) const DAY18: Solution = Solution {
    part1: |input| {
        fn plus(input: &str) -> IResult<&str, u64> {
            let (i, init) = paren(input)?;
            fold_many0(
                pair(delimited(char(' '), one_of("+*"), char(' ')), paren),
                init,
                |acc, (op, val)| match op {
                    '*' => acc * val,
                    '+' => acc + val,
                    _ => unreachable!(),
                },
            )(i)
        }

        fn paren(input: &str) -> IResult<&str, u64> {
            delimited(char('('), plus, char(')')).or(int).parse(input)
        }

        let mut sum = 0;
        for line in input.lines() {
            let (_, out) = terminated(plus, eof)(line)?;
            sum += out;
        }
        Ok(sum.to_string())
    },
    part2: |input| {
        fn plus(input: &str) -> IResult<&str, u64> {
            let (i, init) = paren(input)?;
            fold_many0(preceded(tag(" + "), paren), init, |acc, val| acc + val)(i)
        }

        fn mul(input: &str) -> IResult<&str, u64> {
            let (i, init) = plus(input)?;
            fold_many0(preceded(tag(" * "), plus), init, |acc, val| acc * val)(i)
        }

        fn paren(input: &str) -> IResult<&str, u64> {
            delimited(char('('), mul, char(')')).or(int).parse(input)
        }

        let mut sum = 0;
        for line in input.lines() {
            let (_, out) = terminated(mul, eof)(line)?;
            sum += out;
        }
        Ok(sum.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY18.part1,
        example1: "1 + 2 * 3 + 4 * 5 + 6" => 71,
        example2: "1 + (2 * 3) + (4 * (5 + 6))" => 51,
        example3: "2 * 3 + (4 * 5)" => 26,
        example4: "5 + (8 * 3 + 9 + 3 * 4 * 3)" => 437,
        example5: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 12_240,
        example6: "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 13_632,
        input: 510_009_915_468,
    );
    test!(
        DAY18.part2,
        example1: "1 + 2 * 3 + 4 * 5 + 6" => 231,
        example2: "1 + (2 * 3) + (4 * (5 + 6))" => 51,
        example3: "2 * 3 + (4 * 5)" => 46,
        example4: "5 + (8 * 3 + 9 + 3 * 4 * 3)" => 1_445,
        example5: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 669_060,
        example6: "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 23_340,
        input: 321_176_691_637_769,
    );
}
