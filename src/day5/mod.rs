use crate::Solution;

fn parse(input: &str) -> u16 {
    input
        .chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => unreachable!(),
        })
        .fold(0, |a, b| a << 1 | b)
}

pub(super) const DAY5: Solution = Solution {
    part1: |input| {
        Ok(input
            .lines()
            .map(parse)
            .max()
            .ok_or("No board passes were passed in")?
            .to_string())
    },
    part2: |input| {
        let mut seats: Vec<u16> = input.lines().map(parse).collect();
        seats.sort();
        let solution = seats
            .windows(2)
            .find(|window| window[0] + 1 != window[1])
            .ok_or("No valid solution has been found")?[0]
            + 1;
        Ok(solution.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY5.part1,
        fn empty_input_fails() {
            assert!((DAY5.part1)("").is_err());
        }
        example1: "FBFBBFFRLR" => 357,
        example2: "BFFFBBFRRR" => 567,
        example3: "FFFBBBFRRR" => 119,
        example4: "BBFFBBFRLL" => 820,
        input: 987,
    );
    test!(
        DAY5.part2,
        fn empty_input_fails() {
            assert!((DAY5.part2)("").is_err());
        }
        input: 603,
    );
}
