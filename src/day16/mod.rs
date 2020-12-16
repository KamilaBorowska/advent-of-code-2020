use crate::{
    parsers::{int, take_until_and_consume},
    Solution,
};
use fnv::FnvHashSet;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::eof,
    multi::{many0, separated_list0, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    Finish, IResult, Parser,
};
use std::ops::RangeInclusive;

struct Notes<'a> {
    rules: Vec<(&'a str, [RangeInclusive<u16>; 2])>,
    your: Vec<u16>,
    nearby: Vec<Vec<u16>>,
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u16>> {
    separated_pair(int, char('-'), int)
        .map(|(a, b)| a..=b)
        .parse(input)
}

impl<'a> Notes<'a> {
    fn parse(input: &'a str) -> Result<Self, nom::error::Error<&str>> {
        let (_, (rules, your, nearby, _, _)) = tuple((
            many0(terminated(
                pair(
                    take_until_and_consume(": "),
                    separated_pair(range, tag(" or "), range).map(|(a, b)| [a, b]),
                ),
                char('\n'),
            )),
            preceded(tag("\nyour ticket:\n"), separated_list1(char(','), int)),
            preceded(
                tag("\n\nnearby tickets:\n"),
                separated_list0(char('\n'), separated_list1(char(','), int)),
            ),
            take_while(|c| c == '\n'),
            eof,
        ))(input)
        .finish()?;
        Ok(Notes {
            rules,
            your,
            nearby,
        })
    }
}

pub(super) const DAY16: Solution = Solution {
    part1: |input| {
        let notes = Notes::parse(input)?;
        Ok(notes
            .nearby
            .iter()
            .flatten()
            .filter(|num| {
                !notes
                    .rules
                    .iter()
                    .flat_map(|(_, ranges)| ranges)
                    .any(|range| range.contains(num))
            })
            .sum::<u16>()
            .to_string())
    },
    part2: |input| {
        let notes = Notes::parse(input)?;
        let mut rules = notes
            .nearby
            .iter()
            .filter(|nearby| {
                nearby.iter().all(|num| {
                    notes
                        .rules
                        .iter()
                        .flat_map(|(_, ranges)| ranges)
                        .any(|range| range.contains(num))
                })
            })
            .fold(
                vec![(1 << notes.rules.len()) - 1; notes.rules.len()],
                |mut valid_possibilities, nearby_ticket| {
                    for (possibility, ticket_pos) in
                        valid_possibilities.iter_mut().zip(nearby_ticket)
                    {
                        for (i, (_, ranges)) in notes.rules.iter().enumerate() {
                            if !ranges.iter().any(|range| range.contains(ticket_pos)) {
                                *possibility &= !(1u32 << i);
                            }
                        }
                    }
                    valid_possibilities
                },
            );
        for _ in 0..rules.len() {
            let powers_of_two = rules
                .iter()
                .filter(|x| x.is_power_of_two())
                .fold(0, |a, b| a | b);
            let mut any_changes = false;
            for num in rules.iter_mut().filter(|x| !x.is_power_of_two()) {
                any_changes = true;
                *num &= !powers_of_two;
            }
            if !any_changes {
                break;
            }
        }
        assert!(rules.iter().all(|x| x.is_power_of_two()));
        let departure_fields: FnvHashSet<u32> = notes
            .rules
            .iter()
            .enumerate()
            .filter(|(_, (field, _))| field.starts_with("departure"))
            .map(|(i, _)| 1 << i)
            .collect();
        Ok(rules
            .iter()
            .enumerate()
            .filter(|(_, value)| departure_fields.contains(value))
            .map(|(i, _)| u64::from(notes.your[i]))
            .product::<u64>()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY16.part1,
        example: lines!(
            "class: 1-3 or 5-7"
            "row: 6-11 or 33-44"
            "seat: 13-40 or 45-50"
            ""
            "your ticket:"
            "7,1,14"
            ""
            "nearby tickets:"
            "7,3,47"
            "40,4,50"
            "55,2,20"
            "38,6,12"
        ) => 71,
        input: 23_054,
    );
    test!(
        DAY16.part2,
        input: 51_240_700_105_297,
    );
}
