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

    fn process_bag<F, A>(bag: &str, initial: A, mut process_cubes: F) -> BoxResult<A>
    where
        F: FnMut(A, &str, Output) -> BoxResult<A>,
    {
        bag.split("; ").try_fold(initial, |acc, revelation| {
            revelation.split(", ").try_fold(acc, |acc, cube| {
                let (_, [count, color]) = CUBES_BY_COLOR_PATTERN
                    .captures(cube)
                    .ok_or(AocError)?
                    .extract();
                let count = count.parse::<Output>()?;
                process_cubes(acc, color, count)
            })
        })
    }

    fn handle_game_1(bag: &str, game: Output) -> BoxResult<(Output, bool)> {
        let results = Self::process_bag(bag, Vec::new(), |mut acc, color, count| {
            let condition = match color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => return BoxResult::Err(AocError.into()),
            };
            acc.push(condition);
            BoxResult::Ok(acc)
        })?;
        Ok((game, results.into_iter().all(|b| b)))
    }

    fn handle_game_2(bag: &str, game: Output) -> BoxResult<(Output, Output)> {
        let (max_red, max_green, max_blue) = Self::process_bag(
            bag,
            (Output::MIN, Output::MIN, Output::MIN),
            |(max_red, max_green, max_blue), color, count| match color {
                "red" => BoxResult::Ok((Output::max(max_red, count), max_green, max_blue)),
                "green" => BoxResult::Ok((max_red, Output::max(max_green, count), max_blue)),
                "blue" => BoxResult::Ok((max_red, max_green, Output::max(max_blue, count))),
                _ => Err(AocError.into()),
            },
        )?;
        Ok((game, max_red * max_green * max_blue))
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
