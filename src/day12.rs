use lazy_static::lazy_static;
use regex::Regex;
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
    fn validate_1(springs: &[u8], vec: &Vec<Output>) -> bool {
        let s = String::from_utf8(springs.to_owned()).unwrap();
        let groups = PATTERN.find_iter(&s).collect_vec();
        if groups.len() != vec.len() {
            return false;
        }
        groups
            .into_iter()
            .zip(vec.iter())
            .all(|(springs, &len)| springs.len() == len)
    }

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

    fn validate_2(springs: &[u8], ranges: &Vec<Output>) -> Output {
        let len = springs.len();
        let rv =
            springs
                .iter()
                .enumerate()
                .fold(vec![(false, ranges.to_owned())], |states, (i, &b)| {
                    let min_damage = springs.iter().skip(i).filter(|&b| *b == b'#').count();
                    let max_damage = springs
                        .iter()
                        .skip(i)
                        .filter(|&b| *b == b'#' || *b == b'?')
                        .count();
                    let min_operational = springs.iter().skip(i).filter(|&b| *b == b'.').count();
                    let max_operational = springs
                        .iter()
                        .skip(i)
                        .filter(|&b| *b == b'.' || *b == b'?')
                        .count();
                    let left = len - i;
                    // println!(
                    //     "left {} min_damage {} max_damage {} states.len() {}",
                    //     left,
                    //     min_damage,
                    //     max_damage,
                    //     states.len()
                    // );
                    states
                        .into_iter()
                        .flat_map(|(is_running, ranges)| {
                            let damaged_left: Output = ranges.iter().copied().sum();
                            let operational_left = ranges.len();
                            //println!("damaged_left {}", damaged_left);
                            if damaged_left < min_damage || damaged_left > max_damage {
                                //                            println!("PRUNE! {}", damaged_left);
                                vec![]
                            } else {
                                Self::validate_single(
                                    b,
                                    is_running,
                                    if ranges.is_empty() { 0 } else { ranges[0] },
                                )
                                .into_iter()
                                .map(move |(is_running, is_range_complete)| {
                                    (
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
                                    )
                                })
                                .collect_vec()
                            }
                            .into_iter()
                        })
                        .collect()
                });
        //        println!("{} {:?}", rv.len(), rv);
        let rv = rv
            .iter()
            .filter(|(_, v)| v.is_empty() || v.len() == 1 && v[0] == 0)
            .count();
//        println!("{}", rv);
        rv
    }

    fn process_1(input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|rs| {
                rs.map_err(|_| AocError).and_then(|s| {
                    let mut tokens = s.split_whitespace();
                    let (springs, ranges) = (
                        tokens.next().ok_or(AocError)?,
                        tokens.next().ok_or(AocError)?,
                    );
                    let unknowns = springs.match_indices('?').map(|(i, _)| i).collect_vec();
                    let springs = springs.as_bytes();
                    let ranges = ranges
                        .split(',')
                        .map(|s| Ok(s.parse()?))
                        .collect::<BoxResult<Vec<Output>>>()
                        .map_err(|_| AocError)?;
                    Ok((0..(1 << unknowns.len()))
                        .filter(|&mask| {
                            let mut springs = springs.to_owned();
                            for i in 0..unknowns.len() {
                                springs[unknowns[i]] =
                                    if (mask & (1 << i)) != 0 { b'.' } else { b'#' }
                            }
                            Self::validate_1(&springs, &ranges)
                        })
                        .count())
                })
            })
            .collect::<Result<Vec<_>, AocError>>()?
            .into_iter()
            //            .map(|c| { println!("{}", c); c })
            .sum())
    }

    fn process_2(input: &mut dyn io::Read, is_folded: bool) -> BoxResult<Output> {
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
                    let unknowns = springs.match_indices('?').map(|(i, _)| i).collect_vec();
                    let springs = springs.as_bytes();
                    let ranges = ranges
                        .split(',')
                        .map(|s| Ok(s.parse()?))
                        .collect::<BoxResult<Vec<Output>>>()
                        .map_err(|_| AocError)?;
                    Ok(Self::validate_2(springs, &ranges))
                })
            })
            .collect::<Result<Vec<_>, AocError>>()?
            .into_iter()
            .sum())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process_2(input, false)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process_2(input, true)
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
    .###.##....# 3,2,1",
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
