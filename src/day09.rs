use crate::day::*;
use regex::Regex;

pub struct Day09 {}

type Output = i64;

impl Day for Day09 {
    fn tag(&self) -> &str {
        "09"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref NODE_PATTERN: Regex = Regex::new("^(.*) = \\((.*), (.*)\\)$").unwrap();
}

impl Day09 {
    fn extrapolate(v: Vec<Output>, do_last: bool) -> Output {
        if v.iter().all(|&n| n == 0) {
            0
        } else {
            let n = Self::extrapolate(
                v.iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>(),
                do_last,
            );
            if do_last {
                v[v.len() - 1] + n
            } else {
                v[0] - n
            }
        }
    }

    fn process(input: &mut dyn io::Read, do_last: bool) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|rs| {
                rs.map_err(|_| AocError.into()).and_then(|s| {
                    BoxResult::Ok(Self::extrapolate(
                        s.split_whitespace()
                            .map(|n| n.parse::<Output>().expect("parse failed"))
                            .collect::<Vec<_>>(),
                        do_last,
                    ))
                })
            })
            .collect::<BoxResult<Vec<_>>>()?
            .into_iter()
            .sum())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, true)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day09 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
            114,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day09 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
            2,
        );
    }
}
