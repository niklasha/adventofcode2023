use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::iter;

use crate::day::*;

pub struct Day12 {}

type Output = usize;

impl Day for Day12 {
    fn tag(&self) -> &str {
        "12"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("#+").unwrap();
}

impl Day12 {
    fn unfold(springs: &str, ranges: &str) -> (String, String) {
        (
            iter::repeat(springs).take(5).join("?"),
            iter::repeat(ranges).take(5).join(","),
        )
    }

    // returns BoxResult<Vec<(is_running, is_range_done)>>
    fn validate_single(c: u8, is_running: bool, len: usize) -> Vec<(bool, bool)> {
        if c == b'?' { vec![b'.', b'#'] } else { vec![c] }
            .into_iter()
            .flat_map(|c| match c {
                b'.' if !is_running => Some((false, false)),
                b'.' if is_running && len == 0 => Some((false, true)),
                b'#' if len > 0 => Some((true, false)),
                b'.' | b'#' => None,
                _ => unreachable!(), // XXX
            })
            .collect_vec()
    }

    fn validate(springs: &[u8], ranges: &Vec<Output>) -> Output {
        let mut states = HashMap::new();
        states.insert((false, ranges.to_owned()), 1 as Output);
        let rv = springs.iter().enumerate().fold(states, |states, (i, &b)| {
            let min_damage = springs.iter().skip(i).filter(|&b| *b == b'#').count();
            let max_damage = springs
                .iter()
                .skip(i)
                .filter(|&b| *b == b'#' || *b == b'?')
                .count();
            let mut new_states = HashMap::new();
            states.into_iter().for_each(|((is_running, ranges), cnt)| {
                let damaged_left: Output = ranges.iter().copied().sum();
                if damaged_left < min_damage || damaged_left > max_damage {
                    //                                println!("PRUNE! {}", damaged_left);
                } else {
                    let v = Self::validate_single(
                        b,
                        is_running,
                        if ranges.is_empty() { 0 } else { ranges[0] },
                    );
                    v.into_iter().for_each(|(is_running, is_range_complete)| {
                        let state = (
                            is_running,
                            if is_range_complete {
                                let mut ranges = ranges.clone();
                                ranges.remove(0);
                                ranges
                            } else if is_running {
                                let mut ranges = ranges.clone();
                                ranges[0] -= 1;
                                ranges
                            } else {
                                ranges.clone()
                            },
                        );
                        *new_states.entry(state).or_insert(0) += cnt;
                    });
                }
            });
            new_states
        });
        let v = rv
            .iter()
            .filter(|((_, v), _)| v.is_empty() || v.len() == 1 && v[0] == 0)
            .map(|(_, &cnt)| cnt)
            .collect_vec();
        let rv = v.into_iter().sum();
        rv
    }

    fn process(input: &mut dyn io::Read, is_folded: bool) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|rs| {
                rs.map_err(|_| AocError).and_then(|s| {
                    let mut tokens = s.split_whitespace();
                    let (springs, ranges) = (
                        tokens.next().ok_or(AocError)?,
                        tokens.next().ok_or(AocError)?,
                    );
                    let (mut springs, mut ranges) = (springs.to_string(), ranges.to_string());
                    if is_folded {
                        (springs, ranges) = Self::unfold(&springs, &ranges);
                    }
                    let springs = springs.as_bytes();
                    let ranges = ranges
                        .split(',')
                        .map(|s| Ok(s.parse()?))
                        .collect::<BoxResult<Vec<Output>>>()
                        .map_err(|_| AocError)?;
                    Ok(Self::validate(springs, &ranges))
                })
            })
            .collect::<Result<Vec<_>, AocError>>()?
            .into_iter()
            .sum())
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
        assert_eq!(Day12 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
",
            6,
        );
        test1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
            21,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day12 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
            525152,
        );
    }
}
