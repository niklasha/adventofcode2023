use crate::day::*;

pub struct Day01 {}

type Output = usize;

impl Day for Day01 {
    fn tag(&self) -> &str {
        "01"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref TOKENS: Vec<&'static str> =
        vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
}

impl Day01 {
    fn to_digit(s: &str, is_spelled_out: bool) -> Option<Output> {
        s.chars().next().and_then(|d| {
            if d.is_ascii_digit() {
                d.to_digit(10).map(|d| d as Output)
            } else if is_spelled_out {
                TOKENS.iter().position(|t| s.starts_with(t)).map(|p| p + 1)
            } else {
                None
            }
        })
    }

    fn process(input: &mut dyn io::Read, is_spelled_out: bool) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|rs| {
                rs.map_err(|e| e.into()).and_then(|s| {
                    let mut d = (0..s.len()).flat_map(|o| Self::to_digit(&s[o..], is_spelled_out));
                    let d1 = d.next().ok_or(AocError)?;
                    let dn = d.last().unwrap_or(d1);
                    Ok((d1 * 10 + dn) as Output)
                })
            })
            .collect::<BoxResult<Vec<_>>>()
            .map(|v| v.into_iter().sum())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, false)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day01 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
            142,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day01 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
            281,
        );
    }
}
