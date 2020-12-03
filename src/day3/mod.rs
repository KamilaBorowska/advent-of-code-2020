use crate::Solution;
use std::iter;

struct Map {
    width: usize,
    map: Vec<bool>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut width = None;
        let map = input
            .lines()
            .flat_map(|line| {
                assert!(width.is_none() || width == Some(line.len()));
                width = Some(line.len());
                line.chars().map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
            })
            .collect();
        Map {
            width: width.unwrap_or(1),
            map,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.map.get(x % self.width + y * self.width).copied()
    }
}

pub(super) const DAY3: Solution = Solution {
    part1: |input| {
        let map = Map::parse(input);
        let mut x = 0;
        let mut y = 0;
        let iter = iter::from_fn(|| {
            x += 3;
            y += 1;
            map.get(x, y)
        });
        let trees: usize = iter.map(usize::from).sum();
        Ok(trees.to_string())
    },
    part2: |input| {
        let map = Map::parse(input);
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product: u64 = slopes
            .iter()
            .map(|(x_mod, y_mod)| -> u64 {
                let mut x = 0;
                let mut y = 0;
                let iter = iter::from_fn(|| {
                    x += x_mod;
                    y += y_mod;
                    map.get(x, y)
                });
                iter.map(u64::from).sum()
            })
            .product();
        Ok(product.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "..##......."
        "#...#...#.."
        ".#....#..#."
        "..#.#...#.#"
        ".#...##..#."
        "..#.##....."
        ".#.#.#....#"
        ".#........#"
        "#.##...#..."
        "#...##....#"
        ".#..#...#.#"
    );
    test!(
        DAY3.part1,
        example: super::EXAMPLE => 7,
        input: 294,
    );
    test!(
        DAY3.part2,
        example: super::EXAMPLE => 336,
        input: 5_774_564_250,
    );
}
