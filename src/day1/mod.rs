use crate::Solution;
use fnv::FnvHashSet;

const TARGET: u16 = 2020;

pub(super) const DAY1: Solution = Solution {
    part1: |input| {
        let mut encountered = FnvHashSet::default();
        for line in input.lines() {
            let number = line.parse::<u16>()?;
            encountered.insert(number);
            let second_number = TARGET - number;
            if encountered.contains(&second_number) {
                return Ok((u32::from(number) * u32::from(second_number)).to_string());
            }
        }
        Err("No valid solution has been found".into())
    },
    part2: |input| {
        let numbers = input
            .lines()
            .map(str::parse)
            .collect::<Result<FnvHashSet<u16>, _>>()?;
        for &a in &numbers {
            for &b in &numbers {
                if let Some(c) = TARGET.checked_sub(a + b) {
                    if numbers.contains(&c) {
                        return Ok((u64::from(a) * u64::from(b) * u64::from(c)).to_string());
                    }
                }
            }
        }
        Err("No valid solution has been found".into())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY1.part1,
        fn empty_input_fails() {
            assert!((DAY1.part1)("").is_err());
        }
        example: lines!(1721 979 366 299 675 1456) => 514579,
        input: 876459,
    );
    test!(
        DAY1.part2,
        fn empty_input_fails() {
            assert!((DAY1.part2)("").is_err());
        }
        example: lines!(1721 979 366 299 675 1456) => 241861950,
        input: 116168640,
    );
}
