use crate::Solution;
use fnv::FnvHashSet;
use itertools::iproduct;
use std::error::Error;
use std::hash::Hash;

fn run_solution<T, I>(input: &str, f: fn(T) -> I) -> Result<String, Box<dyn Error>>
where
    T: Copy + Default + Eq + Hash,
    for<'a> &'a mut T: IntoIterator<Item = &'a mut i8>,
    I: Iterator<Item = T>,
{
    let mut grid = FnvHashSet::default();
    for (y, line) in (0..).zip(input.lines()) {
        for (x, state) in (0..).zip(line.chars()) {
            match state {
                '#' => {
                    let mut array = T::default();
                    for (elem, &value) in array.into_iter().zip(&[x, y]) {
                        *elem = value;
                    }
                    grid.insert(array);
                }
                '.' => {}
                _ => return Err("Unrecognized grid state".into()),
            }
        }
    }
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
    Ok(grid.len().to_string())
}

const MODIFIERS: [i8; 3] = [-1, 0, 1];

pub(super) const DAY17: Solution = Solution {
    part1: |input| {
        run_solution(input, |[px, py, pz]: [i8; 3]| {
            iproduct!(&MODIFIERS, &MODIFIERS, &MODIFIERS)
                .map(move |(mx, my, mz)| [px + mx, py + my, pz + mz])
        })
    },
    part2: |input| {
        run_solution(input, |[px, py, pz, pw]: [i8; 4]| {
            iproduct!(&MODIFIERS, &MODIFIERS, &MODIFIERS, &MODIFIERS)
                .map(move |(mx, my, mz, mw)| [px + mx, py + my, pz + mz, pw + mw])
        })
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
