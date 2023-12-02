use crate::day::*;
use regex::Regex;

pub struct Day02 {}

type Output = usize;

impl Day for Day02 {
    fn tag(&self) -> &str {
        "02"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref GAME_PATTERN: Regex = Regex::new("^Game (.+): (.*)$").unwrap();
    static ref CUBES_BY_COLOR_PATTERN: Regex = Regex::new("^(.+) (.*)$").unwrap();
}

impl Day02 {
    fn process<F, R>(input: &mut dyn io::Read, handle_game: F) -> BoxResult<Vec<(Output, R)>>
        where
            F: Fn(&str, Output) -> BoxResult<(Output, R)>,
    {
        io::BufReader::new(input)
            .lines()
            .map(|rs| {
                rs.map_err(|e| e.into()).and_then(|s| {
                    let (_, [game, bag]) = GAME_PATTERN.captures(&s).ok_or(AocError)?.extract();
                    let game = game.parse::<Output>()?;
                    handle_game(bag, game)
                })
            })
            .collect::<BoxResult<Vec<_>>>()
    }

    fn handle_game_1(bag: &str, game: Output) -> BoxResult<(Output, bool)> {
        Ok((
            game,
            bag.split("; ")
                .map(|revelation| {
                    Ok(revelation
                        .split(", ")
                        .map(|cubes| {
                            let (_, [count, color]) = CUBES_BY_COLOR_PATTERN
                                .captures(cubes)
                                .ok_or(AocError)?
                                .extract();
                            let count = count.parse::<Output>()?;
                            match color {
                                "red" => Ok(count <= 12),
                                "green" => Ok(count <= 13),
                                "blue" => Ok(count <= 14),
                                _ => Err(AocError.into()),
                            }
                        })
                        .collect::<BoxResult<Vec<_>>>()?
                        .into_iter()
                        .all(|b| b))
                })
                .collect::<BoxResult<Vec<_>>>()?
                .into_iter()
                .all(|b| b),
        ))
    }

    fn handle_game_2(bag: &str, game: Output) -> BoxResult<(Output, usize)> {
        let min = bag.split("; ").try_fold(
            (Output::MIN, Output::MIN, Output::MIN),
            |min, revelation| {
                revelation.split(", ").try_fold(min, |min, cubes| {
                    let (_, [count, color]) = CUBES_BY_COLOR_PATTERN
                        .captures(cubes)
                        .ok_or(AocError)?
                        .extract();
                    let count = count.parse::<Output>()?;
                    match color {
                        "red" => BoxResult::Ok((Output::max(min.0, count), min.1, min.2)),
                        "green" => BoxResult::Ok((min.0, Output::max(min.1, count), min.2)),
                        "blue" => BoxResult::Ok((min.0, min.1, Output::max(min.2, count))),
                        _ => Err(AocError.into()),
                    }
                })
            },
        )?;
        Ok((game, min.0 * min.1 * min.2))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(input, Self::handle_game_1);
        r.map(|v| {
            v.into_iter()
                .filter(|(_, ok)| *ok)
                .map(|(game, _)| game)
                .sum()
        })
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(input, Self::handle_game_2);
        r.map(|v| v.into_iter().map(|(_, power)| power).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day02 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            8,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day02 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            2286,
        );
    }
}
