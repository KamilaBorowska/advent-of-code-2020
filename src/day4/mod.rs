use crate::Solution;
use std::ops::RangeInclusive;

fn valid_passports(input: &str) -> impl Iterator<Item = impl Iterator<Item = &str>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .filter(|token| !token.starts_with("cid:"))
        })
        .filter(|fields| fields.clone().count() == 7)
}

pub(super) const DAY4: Solution = Solution {
    part1: |input| Ok(valid_passports(input).count().to_string()),
    part2: |input| {
        Ok(valid_passports(input)
            .map(|mut passport| {
                passport.all(|token| {
                    let mut parts = token.split(':');
                    let name = parts.next().unwrap();
                    let value = parts.next().expect("a value");
                    let between = |value: &str, range: RangeInclusive<u16>| {
                        value
                            .parse()
                            .map(|value| range.contains(&value))
                            .unwrap_or(false)
                    };
                    match name {
                        "byr" => between(value, 1920..=2002),
                        "iyr" => between(value, 2010..=2020),
                        "eyr" => between(value, 2020..=2030),
                        "hgt" => {
                            if let Some(value) = value.strip_suffix("cm") {
                                between(value, 150..=193)
                            } else if let Some(value) = value.strip_suffix("in") {
                                between(value, 59..=76)
                            } else {
                                false
                            }
                        }
                        "hcl" => {
                            value.len() == 7
                                && value.starts_with('#')
                                && value[1..].chars().all(|c| c.is_digit(16))
                        }
                        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                        "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
                        _ => unreachable!(),
                    }
                })
            })
            .filter(|&b| b)
            .count()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY4.part1,
        example: lines!(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"
            "byr:1937 iyr:2017 cid:147 hgt:183cm"
            ""
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"
            "hcl:#cfa07d byr:1929"
            ""
            "hcl:#ae17e1 iyr:2013"
            "eyr:2024"
            "ecl:brn pid:760753108 byr:1931"
            "hgt:179cm"
            ""
            "hcl:#cfa07d eyr:2025 pid:166559648"
            "iyr:2011 ecl:brn hgt:59in"
        ) => 2,
        input: 239,
    );
    test!(
        DAY4.part2,
        example1: lines!(
            "eyr:1972 cid:100"
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            ""
            "iyr:2019"
            "hcl:#602927 eyr:1967 hgt:170cm"
            "ecl:grn pid:012533040 byr:1946"
            ""
            "hcl:dab227 iyr:2012"
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            ""
            "hgt:59cm ecl:zzz"
            "eyr:2038 hcl:74454a iyr:2023"
            "pid:3556412378 byr:2007"
        ) => 0,
        example2: lines!(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"
            "hcl:#623a2f"
            ""
            "eyr:2029 ecl:blu cid:129 byr:1989"
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            ""
            "hcl:#888785"
            "hgt:164cm byr:2001 iyr:2015 cid:88"
            "pid:545766238 ecl:hzl"
            "eyr:2022"
            ""
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        ) => 4,
        input: 188,
    );
}
