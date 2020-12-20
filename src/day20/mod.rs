use crate::Solution;
use fnv::FnvHashMap;
use std::str::Lines;

struct Map {
    width: usize,
    map: Vec<bool>,
}

impl Map {
    fn parse(lines: Lines<'_>) -> Self {
        let mut width = None;
        let map = lines
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
        self.map
            .get(y.checked_mul(self.width)?..)?
            .get(..self.width)?
            .get(x)
            .copied()
    }

    fn height(&self) -> usize {
        self.map.len() / self.width
    }
}

pub(super) const DAY20: Solution = Solution {
    part1: |input| {
        let mut borders = FnvHashMap::default();
        for tile in input.split("\n\n") {
            let mut lines = tile.lines();
            let id: u64 = lines
                .next()
                .ok_or("Missing first line")?
                .strip_prefix("Tile ")
                .and_then(|l| l.strip_suffix(":"))
                .ok_or("Missing initial line")?
                .parse()?;
            let map = Map::parse(lines);
            let current_borders: Vec<Vec<bool>> = vec![
                (0..map.width).map(|x| map.get(x, 0).unwrap()).collect(),
                (0..map.width)
                    .map(|x| map.get(x, map.height() - 1).unwrap())
                    .collect(),
                (0..map.height()).map(|y| map.get(0, y).unwrap()).collect(),
                (0..map.height())
                    .map(|y| map.get(map.width - 1, y).unwrap())
                    .collect(),
                (0..map.width)
                    .rev()
                    .map(|x| map.get(x, 0).unwrap())
                    .collect(),
                (0..map.width)
                    .rev()
                    .map(|x| map.get(x, map.height() - 1).unwrap())
                    .collect(),
                (0..map.height())
                    .rev()
                    .map(|y| map.get(0, y).unwrap())
                    .collect(),
                (0..map.height())
                    .rev()
                    .map(|y| map.get(map.width - 1, y).unwrap())
                    .collect(),
            ];
            for border in current_borders {
                borders
                    .entry(border)
                    .and_modify(|v| {
                        if *v != Some(id) {
                            *v = None
                        }
                    })
                    .or_insert(Some(id));
            }
        }
        let mut counts = FnvHashMap::default();
        for id in borders.values().flatten() {
            *counts.entry(id).or_insert(0) += 1;
        }
        Ok(counts
            .iter()
            .filter(|&(_, &v)| v == 4)
            .map(|(&k, _)| k)
            .product::<u64>()
            .to_string())
    },
    part2: |_| unimplemented!(),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY20.part1,
        example: lines!(
            "Tile 2311:"
            "..##.#..#."
            "##..#....."
            "#...##..#."
            "####.#...#"
            "##.##.###."
            "##...#.###"
            ".#.#.#..##"
            "..#....#.."
            "###...#.#."
            "..###..###"
            ""
            "Tile 1951:"
            "#.##...##."
            "#.####...#"
            ".....#..##"
            "#...######"
            ".##.#....#"
            ".###.#####"
            "###.##.##."
            ".###....#."
            "..#.#..#.#"
            "#...##.#.."
            ""
            "Tile 1171:"
            "####...##."
            "#..##.#..#"
            "##.#..#.#."
            ".###.####."
            "..###.####"
            ".##....##."
            ".#...####."
            "#.##.####."
            "####..#..."
            ".....##..."
            ""
            "Tile 1427:"
            "###.##.#.."
            ".#..#.##.."
            ".#.##.#..#"
            "#.#.#.##.#"
            "....#...##"
            "...##..##."
            "...#.#####"
            ".#.####.#."
            "..#..###.#"
            "..##.#..#."
            ""
            "Tile 1489:"
            "##.#.#...."
            "..##...#.."
            ".##..##..."
            "..#...#..."
            "#####...#."
            "#..#.#.#.#"
            "...#.#.#.."
            "##.#...##."
            "..##.##.##"
            "###.##.#.."
            ""
            "Tile 2473:"
            "#....####."
            "#..#.##..."
            "#.##..#..."
            "######.#.#"
            ".#...#.#.#"
            ".#########"
            ".###.#..#."
            "########.#"
            "##...##.#."
            "..###.#.#."
            ""
            "Tile 2971:"
            "..#.#....#"
            "#...###..."
            "#.#.###..."
            "##.##..#.."
            ".#####..##"
            ".#..####.#"
            "#..#.#..#."
            "..####.###"
            "..#.#.###."
            "...#.#.#.#"
            ""
            "Tile 2729:"
            "...#.#.#.#"
            "####.#...."
            "..#.#....."
            "....#..#.#"
            ".##..##.#."
            ".#.####..."
            "####.#.#.."
            "##.####..."
            "##..#.##.."
            "#.##...##."
            ""
            "Tile 3079:"
            "#.#.#####."
            ".#..######"
            "..#......."
            "######...."
            "####.#..#."
            ".#...#.##."
            "#.#####.##"
            "..#.###..."
            "..#......."
            "..#.###..."
        ) => 20_899_048_083_289,
        input: 17_712_468_069_479,
    );
}
