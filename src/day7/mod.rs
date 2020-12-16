use crate::parsers::{int, take_until_and_consume};
use crate::Solution;
use fnv::FnvHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{eof, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    Finish, Parser,
};
use once_cell::unsync::OnceCell;

type Contents<'a> = Vec<(usize, &'a str)>;

struct Bag<'a> {
    name: &'a str,
    contents: Contents<'a>,
}

impl<'a> Bag<'a> {
    fn parse(input: &'a str) -> Result<Self, nom::error::Error<&'a str>> {
        let (_, (name, contents, _, _)) = tuple((
            take_until_and_consume(" bags contain "),
            tag("no other bags").map(|_| Vec::new()).or(separated_list1(
                tag(", "),
                separated_pair(
                    int,
                    char(' '),
                    terminated(take_until_and_consume(" bag"), opt(tag("s"))),
                ),
            )),
            char('.'),
            eof,
        ))(input)
        .finish()?;
        Ok(Self { name, contents })
    }
}

fn contains_golden<'a>(
    bags: &FnvHashMap<&'a str, (Contents<'a>, OnceCell<bool>)>,
    (contents, cell): &(Contents<'a>, OnceCell<bool>),
) -> bool {
    *cell.get_or_init(|| {
        contents
            .iter()
            .any(|(_, key)| *key == "shiny gold" || contains_golden(bags, &bags[key]))
    })
}

fn count_bags<'a>(
    bags: &FnvHashMap<&'a str, (Contents<'a>, OnceCell<usize>)>,
    (contents, cell): &(Contents<'a>, OnceCell<usize>),
) -> usize {
    *cell.get_or_init(|| {
        contents
            .iter()
            .map(|(count, key)| count * (1 + count_bags(bags, &bags[key])))
            .sum::<usize>()
    })
}

pub(super) const DAY7: Solution = Solution {
    part1: |input| {
        let mut bags = FnvHashMap::default();
        for line in input.lines() {
            let Bag { name, contents } = Bag::parse(line)?;
            bags.insert(name, (contents, OnceCell::new()));
        }
        let count = bags
            .values()
            .filter(|value| contains_golden(&bags, value))
            .count();
        Ok(count.to_string())
    },
    part2: |input| {
        let mut bags = FnvHashMap::default();
        for line in input.lines() {
            let Bag { name, contents } = Bag::parse(line)?;
            bags.insert(name, (contents, OnceCell::new()));
        }
        Ok(count_bags(&bags, &bags["shiny gold"]).to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "light red bags contain 1 bright white bag, 2 muted yellow bags."
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
        "bright white bags contain 1 shiny gold bag."
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags."
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
        "faded blue bags contain no other bags."
        "dotted black bags contain no other bags."
    );
    test!(
        DAY7.part1,
        empty: "" => 0,
        example: EXAMPLE => 4,
        input: 248,
    );
    test!(
        DAY7.part2,
        example1: EXAMPLE => 32,
        example2: lines!(
            "shiny gold bags contain 2 dark red bags."
            "dark red bags contain 2 dark orange bags."
            "dark orange bags contain 2 dark yellow bags."
            "dark yellow bags contain 2 dark green bags."
            "dark green bags contain 2 dark blue bags."
            "dark blue bags contain 2 dark violet bags."
            "dark violet bags contain no other bags."
        ) => 126,
        input: 57281,
    );
}
