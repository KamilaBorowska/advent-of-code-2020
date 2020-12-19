use crate::parsers::int;
use crate::Solution;
use fnv::FnvHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::eof,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use pcre2::bytes::Regex;

enum Rule {
    Sequence(Vec<Vec<u8>>),
    Char(char),
    Special(String),
}

impl Rule {
    fn compile(&self, out: &mut String, map: &FnvHashMap<u8, Self>) {
        match self {
            &Rule::Char(c) => out.push(c),
            Rule::Sequence(seq) => {
                out.push_str("(?:");
                let mut first = true;
                for alt in seq {
                    if !first {
                        out.push('|');
                    }
                    for rule in alt {
                        map[rule].compile(out, map);
                    }
                    first = false;
                }
                out.push(')');
            }
            Rule::Special(s) => out.push_str(s),
        }
    }

    fn compile_to_string(&self, map: &FnvHashMap<u8, Self>) -> String {
        let mut s = String::new();
        self.compile(&mut s, map);
        s
    }
}

fn rule(input: &str) -> IResult<&str, (u8, Rule)> {
    separated_pair(
        int,
        tag(": "),
        terminated(
            separated_list1(tag(" | "), separated_list1(tag(" "), int)),
            eof,
        )
        .map(Rule::Sequence)
        .or(delimited(char('"'), anychar, char('"')).map(Rule::Char)),
    )(input)
}

pub(super) const DAY19: Solution = Solution {
    part1: |input| {
        let mut lines = input.lines();
        let rules = (&mut lines)
            .take_while(|line| !line.is_empty())
            .map(|line| rule(line).map(|(_, r)| r))
            .collect::<Result<FnvHashMap<_, _>, _>>()?;
        let regex = Regex::new(&format!("^{}$", rules[&0].compile_to_string(&rules)))?;
        Ok(lines
            .filter(|line| regex.is_match(line.as_bytes()).unwrap())
            .count()
            .to_string())
    },
    part2: |input| {
        let mut lines = input.lines();
        let mut rules = (&mut lines)
            .take_while(|line| !line.is_empty())
            .map(|line| rule(line).map(|(_, r)| r))
            .collect::<Result<FnvHashMap<_, _>, _>>()?;
        rules.insert(
            8,
            Rule::Special(format!("(?:{})+", rules[&8].compile_to_string(&rules))),
        );
        rules.insert(
            11,
            Rule::Special(format!(
                "({}(?-1)?{})",
                rules[&42].compile_to_string(&rules),
                rules[&31].compile_to_string(&rules)
            )),
        );
        let regex = Regex::new(&format!("^{}$", rules[&0].compile_to_string(&rules)))?;
        Ok(lines
            .filter(|line| regex.is_match(line.as_bytes()).unwrap())
            .count()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY19.part1,
        example: lines!(
            "0: 4 1 5"
            "1: 2 3 | 3 2"
            "2: 4 4 | 5 5"
            "3: 4 5 | 5 4"
            r#"4: "a""#
            r#"5: "b""#
            ""
            "ababbb"
            "bababa"
            "abbbab"
            "aaabbb"
            "aaaabbb"
        ) => 2,
        input: 176,
    );
    test!(
        DAY19.part2,
        example: lines!(
            "42: 9 14 | 10 1"
            "9: 14 27 | 1 26"
            "10: 23 14 | 28 1"
            r#"1: "a""#
            "11: 42 31"
            "5: 1 14 | 15 1"
            "19: 14 1 | 14 14"
            "12: 24 14 | 19 1"
            "16: 15 1 | 14 14"
            "31: 14 17 | 1 13"
            "6: 14 14 | 1 14"
            "2: 1 24 | 14 4"
            "0: 8 11"
            "13: 14 3 | 1 12"
            "15: 1 | 14"
            "17: 14 2 | 1 7"
            "23: 25 1 | 22 14"
            "28: 16 1"
            "4: 1 1"
            "20: 14 14 | 1 15"
            "3: 5 14 | 16 1"
            "27: 1 6 | 14 18"
            r#"14: "b""#
            "21: 14 1 | 1 14"
            "25: 1 1 | 1 14"
            "22: 14 14"
            "8: 42"
            "26: 14 22 | 1 20"
            "18: 15 15"
            "7: 14 5 | 1 21"
            "24: 14 1"
            ""
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"
            "bbabbbbaabaabba"
            "babbbbaabbbbbabbbbbbaabaaabaaa"
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"
            "bbbbbbbaaaabbbbaaabbabaaa"
            "bbbababbbbaaaaaaaabbababaaababaabab"
            "ababaaaaaabaaab"
            "ababaaaaabbbaba"
            "baabbaaaabbaaaababbaababb"
            "abbbbabbbbaaaababbbbbbaaaababb"
            "aaaaabbaabaaaaababaa"
            "aaaabbaaaabbaaa"
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"
            "babaaabbbaaabaababbaabababaaab"
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        ) => 12,
        input: 352,
    );
}
