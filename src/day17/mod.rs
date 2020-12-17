use crate::Solution;
use fnv::FnvHashSet;
use std::hash::Hash;

fn run_solution<T, I>(mut grid: FnvHashSet<T>, f: fn(T) -> I) -> String
where
    T: Copy + Eq + Hash,
    I: Iterator<Item = T>,
{
    for _ in 0..6 {
        let mut positions_to_check: FnvHashSet<_> = grid.iter().copied().flat_map(f).collect();
        positions_to_check.retain(|pos| {
            match f(*pos)
                .filter(|mod_pos| pos != mod_pos && grid.contains(mod_pos))
                .take(3 + 1)
                .count()
            {
                2 => grid.contains(pos),
                3 => true,
                _ => false,
            }
        });
        grid = positions_to_check;
    }
    grid.len().to_string()
}

const MODIFIERS: [i8; 3] = [-1, 0, 1];

pub(super) const DAY17: Solution = Solution {
    part1: |input| {
        let mut grid = FnvHashSet::default();
        for (y, line) in (0..).zip(input.lines()) {
            for (x, state) in (0..).zip(line.chars()) {
                match state {
                    '#' => {
                        grid.insert((x, y, 0));
                    }
                    '.' => {}
                    _ => return Err("Unrecognized grid state".into()),
                }
            }
        }
        Ok(run_solution(grid, |(px, py, pz)| {
            MODIFIERS
                .iter()
                .flat_map(|&x| {
                    MODIFIERS
                        .iter()
                        .flat_map(move |&y| MODIFIERS.iter().map(move |&z| (x, y, z)))
                })
                .map(move |(mx, my, mz)| (px + mx, py + my, pz + mz))
        }))
    },
    part2: |input| {
        let mut grid = FnvHashSet::default();
        for (y, line) in (0..).zip(input.lines()) {
            for (x, state) in (0..).zip(line.chars()) {
                match state {
                    '#' => {
                        grid.insert((x, y, 0, 0));
                    }
                    '.' => {}
                    _ => return Err("Unrecognized grid state".into()),
                }
            }
        }
        Ok(run_solution(grid, |(px, py, pz, pw)| {
            MODIFIERS
                .iter()
                .flat_map(|&x| {
                    MODIFIERS.iter().flat_map(move |&y| {
                        MODIFIERS
                            .iter()
                            .flat_map(move |&z| MODIFIERS.iter().map(move |&w| (x, y, z, w)))
                    })
                })
                .map(move |(mx, my, mz, mw)| (px + mx, py + my, pz + mz, pw + mw))
        }))
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY17.part1,
        example: ".#.\n..#\n###" => 112,
        input: 388,
    );
    test!(
        DAY17.part2,
        example: ".#.\n..#\n###" => 848,
        input: 2_280,
    );
}
