use crate::Solution;
use num_complex::Complex;

fn rotate(value: i32) -> Result<Complex<i32>, &'static str> {
    if value % 90 != 0 {
        return Err("Modulo should be divisible by 90");
    }
    Ok(Complex::i().powi(value / 90))
}

pub(super) const DAY12: Solution = Solution {
    part1: |input| {
        let mut position = Complex::new(0, 0);
        let mut direction = Complex::i();
        for line in input.lines() {
            let mut chars = line.chars();
            let action = chars.next().ok_or("Missing action")?;
            let value: i32 = chars.as_str().parse()?;
            match action {
                'N' => position.re += value,
                'S' => position.re -= value,
                'E' => position.im += value,
                'W' => position.im -= value,
                'L' => direction *= rotate(-value)?,
                'R' => direction *= rotate(value)?,
                'F' => position += direction * value,
                _ => return Err("Unrecognized action".into()),
            }
        }
        Ok((position.re.abs() + position.im.abs()).to_string())
    },
    part2: |input| {
        let mut waypoint = Complex::new(1, 10);
        let mut ship = Complex::new(0, 0);
        for line in input.lines() {
            let mut chars = line.chars();
            let action = chars.next().ok_or("Missing action")?;
            let value: i32 = chars.as_str().parse()?;
            match action {
                'N' => waypoint.re += value,
                'S' => waypoint.re -= value,
                'E' => waypoint.im += value,
                'W' => waypoint.im -= value,
                'L' => waypoint *= rotate(-value)?,
                'R' => waypoint *= rotate(value)?,
                'F' => ship += waypoint * value,
                _ => return Err("Unrecognized action".into()),
            }
        }
        Ok((ship.re.abs() + ship.im.abs()).to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY12.part1,
        empty: "" => 0,
        example: lines!("F10" "N3" "F7" "R90" "F11") => 25,
        input: 879,
    );
    test!(
        DAY12.part2,
        empty: "" => 0,
        example: lines!("F10" "N3" "F7" "R90" "F11") => 286,
        input: 18_107,
    );
}
