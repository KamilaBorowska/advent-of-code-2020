use crate::Solution;
use std::cmp::Ordering;

fn find_number_that_does_not_sum_with_previous(numbers: &[u64]) -> Option<u64> {
    numbers.windows(25 + 1).find_map(|slice| {
        let (&num, preamble) = slice.split_last().expect("non-empty slice");
        for a in preamble {
            for b in preamble {
                if num == a + b {
                    return None;
                }
            }
        }
        Some(num)
    })
}

pub(super) const DAY9: Solution = Solution {
    part1: |input| {
        let numbers = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()?;
        find_number_that_does_not_sum_with_previous(&numbers)
            .map(|num| num.to_string())
            .ok_or_else(|| "no valid solution has been found".into())
    },
    part2: |input| {
        let numbers = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()?;
        let target = find_number_that_does_not_sum_with_previous(&numbers)
            .ok_or("no valid solution has been found")?;
        let mut sum = 0;
        let mut start = 0;
        let mut end = 0;
        loop {
            match sum.cmp(&target) {
                Ordering::Less => {
                    sum += numbers[end];
                    end += 1;
                }
                Ordering::Equal => {
                    let range = &numbers[start..end];
                    return Ok(
                        (range.iter().min().unwrap() + range.iter().max().unwrap()).to_string()
                    );
                }
                Ordering::Greater => {
                    sum -= numbers[start];
                    start += 1;
                }
            }
        }
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY9.part1,
        input: 1_398_413_738,
    );
    test!(
        DAY9.part2,
        input: 169_521_051,
    );
}
