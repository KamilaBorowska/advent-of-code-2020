use crate::Solution;
use fnv::FnvHashMap;
use std::collections::hash_map::Entry;
use std::error::Error;

fn run_game(input: &str, steps: u32) -> Result<String, Box<dyn Error>> {
    let mut split = input.split(',').map(str::parse);
    let mut last_num = split
        .next_back()
        .ok_or("No starting numbers were provided")??;
    let mut map = FnvHashMap::default();
    let mut so_far = 0;
    for (i, number) in (0..).zip(split) {
        so_far += 1;
        map.insert(number?, i);
    }
    for i in so_far..steps - 1 {
        last_num = match map.entry(last_num) {
            Entry::Occupied(mut occupied) => {
                let reference = occupied.get_mut();
                let value = i - *reference;
                *reference = i;
                value
            }
            Entry::Vacant(vacant) => {
                vacant.insert(i);
                0
            }
        };
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
