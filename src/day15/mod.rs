use crate::Solution;
use std::convert::TryFrom;
use std::error::Error;

fn run_game(input: &str, steps: u32) -> Result<String, Box<dyn Error>> {
    let mut split = input.split(',').map(str::parse::<u32>);
    let mut last_num = split
        .next_back()
        .ok_or("No starting numbers were provided")??;
    let mut map = vec![0; usize::try_from(steps)?];
    let mut so_far = 1;
    for number in split {
        map[usize::try_from(number?)?] = so_far;
        so_far += 1;
    }
    for i in so_far..steps {
        let access = &mut map[usize::try_from(last_num)?];
        last_num = if *access == 0 { 0 } else { i - *access };
        *access = i;
    }
    Ok(last_num.to_string())
}

pub(super) const DAY15: Solution = Solution {
    part1: |input| run_game(input, 2020),
    part2: |input| run_game(input, 30_000_000),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY15.part1,
        example1: "0,3,6" => 436,
        example2: "1,3,2" => 1,
        example3: "2,1,3" => 10,
        example4: "1,2,3" => 27,
        example5: "2,3,1" => 78,
        example6: "3,2,1" => 438,
        example7: "3,1,2" => 1_836,
        input: "0,14,6,20,1,4" => 257,
    );
    test!(
        DAY15.part2,
        input: "0,14,6,20,1,4" => 8_546_398,
    );
}
