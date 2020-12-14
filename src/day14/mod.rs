use crate::Solution;
use nom::{
    bytes::complete::{tag, take_while1},
    combinator::{eof, map_res},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

fn int<T: FromStr<Err = ParseIntError>>(input: &str) -> IResult<&str, T> {
    map_res(take_while1(|c: char| c.is_digit(10)), str::parse)(input)
}

fn assignment(input: &str) -> Result<(usize, u64), Box<dyn Error + '_>> {
    let (_, (_, address, _, value, _)) =
        tuple((tag("mem["), int::<usize>, tag("] = "), int::<u64>, eof))(input)?;
    Ok((address, value))
}

fn write_ram(
    memory: &mut HashMap<usize, u64>,
    mut address: usize,
    mut bit_mask: usize,
    shift_mask: usize,
    value: u64,
) {
    while bit_mask != (1 << 36) {
        if bit_mask & shift_mask != 0 {
            write_ram(
                memory,
                address & !bit_mask,
                bit_mask << 1,
                shift_mask,
                value,
            );
            address |= bit_mask;
        }
        bit_mask <<= 1;
    }
    memory.insert(address, value);
}

pub(super) const DAY14: Solution = Solution {
    part1: |input| {
        let mut or_mask = 0;
        let mut and_mask = 0;
        let mut memory = HashMap::new();
        for line in input.lines() {
            if let Some(mask) = line.strip_prefix("mask = ") {
                or_mask = 0;
                and_mask = 0;
                for c in mask.chars() {
                    or_mask <<= 1;
                    and_mask <<= 1;
                    match c {
                        '0' => {}
                        '1' => {
                            or_mask |= 1;
                            and_mask |= 1;
                        }
                        'X' => {
                            and_mask |= 1;
                        }
                        _ => return Err("Unrecognized bitmask character".into()),
                    };
                }
            } else {
                let (address, value) = assignment(line)?;
                memory.insert(address, value & and_mask | or_mask);
            }
        }
        Ok(memory.values().sum::<u64>().to_string())
    },
    part2: |input| {
        let mut or_mask = 0;
        let mut shift_mask = 0;
        let mut memory = HashMap::new();
        for line in input.lines() {
            if let Some(mask) = line.strip_prefix("mask = ") {
                or_mask = 0;
                shift_mask = 0;
                for c in mask.chars() {
                    or_mask <<= 1;
                    shift_mask <<= 1;
                    match c {
                        '0' => {}
                        '1' => {
                            or_mask |= 1;
                        }
                        'X' => {
                            shift_mask |= 1;
                        }
                        _ => return Err("Unrecognized bitmask character".into()),
                    };
                }
            } else {
                let (address, value) = assignment(line)?;
                write_ram(&mut memory, address | or_mask, 1, shift_mask, value);
            }
        }
        Ok(memory.values().sum::<u64>().to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY14.part1,
        example: lines!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            "mem[8] = 11"
            "mem[7] = 101"
            "mem[8] = 0"
        ) => 165,
        input: 10_050_490_168_421,
    );
    test!(
        DAY14.part2,
        example: lines!(
            "mask = 000000000000000000000000000000X1001X"
            "mem[42] = 100"
            "mask = 00000000000000000000000000000000X0XX"
            "mem[26] = 1"
        ) => 208,
        input: 2_173_858_456_958,
    );
}
