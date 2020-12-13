use crate::Solution;
use std::convert::TryFrom;
use std::num::ParseIntError;

pub(super) const DAY13: Solution = Solution {
    part1: |input| {
        let mut lines = input.lines();
        let earliest: u32 = lines.next().ok_or("missing earliest timestamp")?.parse()?;
        let buses = lines
            .next()
            .ok_or("missing buses")?
            .split(',')
            .filter(|&elem| elem != "x")
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()?;
        (0..)
            .filter_map(|num| {
                buses
                    .iter()
                    .find(|&bus| (num + earliest) % bus == 0)
                    .map(|bus| (bus * num).to_string())
            })
            .next()
            .ok_or_else(|| "no valid solution available".into())
    },
    part2: |input| {
        let mut lines = input.lines();
        let buses = lines
            .nth(1)
            .ok_or("missing buses")?
            .split(',')
            .enumerate()
            .filter(|&(_, elem)| elem != "x")
            .map(|(i, num)| Ok((i, num.parse()?)))
            .collect::<Result<Vec<(usize, u64)>, ParseIntError>>()?;
        let mut timestamp = 0;
        let mut jump = buses[0].1;
        for &(delta, bus) in &buses[1..] {
            let delta = u64::try_from(delta)?;
            while (timestamp + delta) % bus != 0 {
                timestamp += jump;
            }
            jump *= bus;
        }
        Ok(timestamp.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY13.part1,
        example: "939\n7,13,x,x,59,x,31,19" => 295,
        input: 3_035,
    );
    test!(
        DAY13.part2,
        example1: "\n7,13,x,x,59,x,31,19" => 1_068_781,
        example2: "\n17,x,13,19" => 3_417,
        example3: "\n67,7,59,61" => 754_018,
        example4: "\n67,x,7,59,61" => 779_210,
        example5: "\n67,7,x,59,61" => 1_261_476,
        example6: "\n1789,37,47,1889" => 1_202_161_486,
        input: 725_169_163_285_238,
    );
}
