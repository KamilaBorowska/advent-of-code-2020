use crate::Solution;
use fnv::FnvHashSet;

pub(super) const DAY6: Solution = Solution {
    part1: |input| {
        let sum: usize = input
            .split("\n\n")
            .map(|answers| {
                answers
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<FnvHashSet<_>>()
                    .len()
            })
            .sum();
        Ok(sum.to_string())
    },
    part2: |input| {
        let sum: usize = input
            .split("\n\n")
            .map(|answers| {
                let mut split_answers = answers.lines();
                let mut first: FnvHashSet<char> = if let Some(answer) = split_answers.next() {
                    answer.chars().collect()
                } else {
                    return 0;
                };
                for answer in split_answers {
                    let other_set: FnvHashSet<char> = answer.chars().collect();
                    first.retain(|elem| other_set.contains(elem));
                }
                first.len()
            })
            .sum();
        Ok(sum.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE2: &str = lines!(
        "abc"
        ""
        "a"
        "b"
        "c"
        ""
        "ab"
        "ac"
        ""
        "a"
        "a"
        "a"
        "a"
        ""
        "b"
    );
    test!(
        DAY6.part1,
        empty: "" => 0,
        example1: lines!(
            "abcx"
            "abcy"
            "abcz"
        ) => 6,
        example2: EXAMPLE2 => 11,
        input: 6763,
    );
    test!(
        DAY6.part2,
        empty: "" => 0,
        example2: EXAMPLE2 => 6,
        input: 3512,
    );
}
