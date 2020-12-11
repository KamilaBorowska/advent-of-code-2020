use crate::Solution;

#[derive(Clone, Eq, PartialEq)]
struct Map {
    width: usize,
    map: Vec<SeatState>,
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
                    '.' => SeatState::Floor,
                    'L' => SeatState::Empty,
                    _ => unreachable!(),
                })
            })
            .collect();
        Map {
            width: width.unwrap_or(1),
            map,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<SeatState> {
        self.map
            .get(y.checked_mul(self.width)?..)?
            .get(..self.width)?
            .get(x)
            .copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut SeatState> {
        self.map
            .get_mut(y.checked_mul(self.width)?..)?
            .get_mut(..self.width)?
            .get_mut(x)
    }

    fn height(&self) -> usize {
        self.map.len() / self.width
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum SeatState {
    Empty,
    Occupied,
    Floor,
}

const ADJACENT: [(usize, usize); 8] = [
    (!0, 1),
    (0, 1),
    (1, 1),
    (!0, 0),
    (1, 0),
    (!0, !0),
    (0, !0),
    (1, !0),
];

pub(super) const DAY11: Solution = Solution {
    part1: |input| {
        let mut map = Map::parse(input);
        let width = map.width;
        let height = map.height();
        loop {
            let occupied = |x: usize, y: usize| {
                let map = &map;
                ADJACENT
                    .iter()
                    .filter_map(move |&(mod_x, mod_y)| {
                        map.get(x.wrapping_add(mod_x), y.wrapping_add(mod_y))
                    })
                    .filter(|&elem| elem == SeatState::Occupied)
                    .map(|_| ())
            };
            let mut new_map = map.clone();
            for x in 0..width {
                for y in 0..height {
                    let seat = new_map.get_mut(x, y).unwrap();
                    *seat = match *seat {
                        SeatState::Empty if occupied(x, y).next().is_none() => SeatState::Occupied,
                        SeatState::Occupied if occupied(x, y).nth(3).is_some() => SeatState::Empty,
                        state => state,
                    };
                }
            }
            if new_map == map {
                return Ok(new_map
                    .map
                    .iter()
                    .filter(|&&state| state == SeatState::Occupied)
                    .count()
                    .to_string());
            }
            map = new_map;
        }
    },
    part2: |input| {
        let mut map = Map::parse(input);
        let width = map.width;
        let height = map.height();
        loop {
            let occupied = |x: usize, y: usize| {
                let map = &map;
                ADJACENT
                    .iter()
                    .filter_map(move |&(mod_x, mod_y)| {
                        let mut x = x;
                        let mut y = y;
                        loop {
                            x = x.wrapping_add(mod_x);
                            y = y.wrapping_add(mod_y);
                            let state = map.get(x, y)?;
                            if state != SeatState::Floor {
                                return Some(state);
                            }
                        }
                    })
                    .filter(|&elem| elem == SeatState::Occupied)
                    .map(|_| ())
            };
            let mut new_map = map.clone();
            for x in 0..width {
                for y in 0..height {
                    let seat = new_map.get_mut(x, y).unwrap();
                    *seat = match *seat {
                        SeatState::Empty if occupied(x, y).next().is_none() => SeatState::Occupied,
                        SeatState::Occupied if occupied(x, y).nth(4).is_some() => SeatState::Empty,
                        state => state,
                    };
                }
            }
            if new_map == map {
                return Ok(new_map
                    .map
                    .iter()
                    .filter(|&&state| state == SeatState::Occupied)
                    .count()
                    .to_string());
            }
            map = new_map;
        }
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "L.LL.LL.LL"
        "LLLLLLL.LL"
        "L.L.L..L.."
        "LLLL.LL.LL"
        "L.LL.LL.LL"
        "L.LLLLL.LL"
        "..L.L....."
        "LLLLLLLLLL"
        "L.LLLLLL.L"
        "L.LLLLL.LL"
    );
    test!(
        DAY11.part1,
        example: EXAMPLE => 37,
        input: 2261,
    );
    test!(
        DAY11.part2,
        example2: EXAMPLE => 26,
        input: 2039,
    );
}
