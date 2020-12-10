use crate::Solution;

pub(super) const DAY10: Solution = Solution {
    part1: |input| {
        let mut numbers = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?;
        numbers.sort_unstable();
        let mut previous = 0;
        let mut diff1 = 0;
        let mut diff3 = 1;
        for number in numbers {
            let diff = number - previous;
            println!("{}", diff);
            if diff == 1 {
                diff1 += 1;
            }
            if diff == 3 {
                diff3 += 1;
            }
            previous = number;
        }
        Ok((diff1 * diff3).to_string())
    },
    part2: |input| {
        let mut numbers = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?;
        numbers.sort_unstable();
        let mut cache = [(1, numbers.last().unwrap() + 3), (0, 0), (0, 0)];
        for &num in numbers.iter().rev() {
            let new_value = cache_sum(cache, num);
            cache = [(new_value, num), cache[0], cache[1]];
        }
        Ok(cache_sum(cache, 0).to_string())
    },
};

fn cache_sum(arr: [(u64, u8); 3], num: u8) -> u64 {
    arr.iter()
        .filter(|&&(_, cached_num)| num + 3 >= cached_num)
        .map(|(cached_value, _)| cached_value)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY10.part1,
        empty: "" => 0,
        example: lines!(16 10 15 5 1 11 7 19 6 12 4) => 35,
        input: 2046,
    );
    test!(
        DAY10.part2,
        example: lines!(16 10 15 5 1 11 7 19 6 12 4) => 8,
        example2: lines!(
            28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39
            11 1 32 25 35 8 17 7 9 4 2 34 10 3
        ) => 19_208,
        input: 1_157_018_619_904,
    );
}
