use crate::day::*;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub struct Day04 {}

type Output = usize;

impl Day for Day04 {
    fn tag(&self) -> &str {
        "04"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref GAME_PATTERN: Regex = Regex::new("^Card (.+): (.*) \\| (.*)$").unwrap();
}

impl Day04 {
    fn process<F>(input: &mut dyn io::Read, mut handle_game: F) -> BoxResult<Output>
    where
        F: FnMut(Output) -> Output,
    {
        Ok(io::BufReader::new(input)
            .lines()
            .enumerate()
            .map(|(_, rs)| {
                rs.map_err(|e| e.into()).and_then(|s| {
                    let (_, [_, winning, hand]) =
                        GAME_PATTERN.captures(&s).ok_or(AocError)?.extract();
                    let winning: HashSet<Output> = winning
                        .split_whitespace()
                        .map(|card| card.parse())
                        .collect::<Result<_, _>>()?;
                    let hand: HashSet<Output> = hand
                        .split_whitespace()
                        .map(|card| card.parse())
                        .collect::<Result<_, _>>()?;
                    Ok(handle_game(hand.intersection(&winning).count()))
                })
            })
            .collect::<BoxResult<Vec<_>>>()?
            .iter()
            .sum())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, |count| 1 << count >> 1)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut extras = VecDeque::new();
        Self::process(input, |count| {
            let rv = 1 + extras.pop_front().unwrap_or(0);
            for i in 0..count {
                if i < extras.len() {
                    extras[i] += rv;
                } else {
                    extras.push_back(rv);
                }
            }
            rv
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day04 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            13,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day04 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            30,
        );
    }
}
