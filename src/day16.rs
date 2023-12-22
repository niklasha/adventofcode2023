use crate::day::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day16 {}

type Output = usize;

impl Day for Day16 {
    fn tag(&self) -> &str {
        "16"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("^(.*)([-=])(.*)$").unwrap();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Coord {
    // Compute the next coordinate in a direction.
    fn walk(self, dir: Dir, size: Coord) -> Option<Self> {
        if self.0 == 0 && dir == Dir::West
            || self.1 == 0 && dir == Dir::North
            || self.0 == size.0 - 1 && dir == Dir::East
            || self.1 == size.1 - 1 && dir == Dir::South
        {
            None
        } else {
            Some(Coord(
                (self.0 as i64
                    + match dir {
                        Dir::East => 1,
                        Dir::West => -1,
                        _ => 0,
                    }) as usize,
                (self.1 as i64
                    + match dir {
                        Dir::South => 1,
                        Dir::North => -1,
                        _ => 0,
                    }) as usize,
            ))
        }
    }
}

impl Day16 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<Coord, u8>, Coord)> {
        io::BufReader::new(input).lines().enumerate().try_fold(
            (HashMap::new(), Coord(0, 0)),
            |(mut map, mut size), (y, rs)| {
                if let Ok(x) = rs.as_ref().map(|s| s.len()) {
                    if x > size.0 {
                        size.0 = x
                    }
                }
                let _ = rs.map_err(|_| AocError).map(|s| {
                    s.bytes().enumerate().for_each(|(x, b)| {
                        map.insert(Coord(x, y), b);
                    })
                });
                size.1 = y + 1;
                Ok((map, size))
            },
        )
    }

    fn energize(tiles: &HashMap<Coord, u8>, size: Coord, start: Coord, dir: Dir) -> Output {
        let mut seen = HashSet::new();
        let _ = (0..).try_fold(vec![(start, dir)], |beams, _| {
            let beams = beams
                .into_iter()
                .filter(|key| {
                    let rv = !seen.contains(key);
                    seen.insert(*key);
                    rv
                })
                .flat_map(|(coord, dir)| {
                    match tiles.get(&coord) {
                        Some(b'-') if dir == Dir::South || dir == Dir::North => {
                            vec![Dir::East, Dir::West]
                        }
                        Some(b'|') if dir == Dir::East || dir == Dir::West => {
                            vec![Dir::South, Dir::North]
                        }
                        Some(b'/') if dir == Dir::South => vec![Dir::West],
                        Some(b'/') if dir == Dir::North => vec![Dir::East],
                        Some(b'/') if dir == Dir::East => vec![Dir::North],
                        Some(b'/') if dir == Dir::West => vec![Dir::South],
                        Some(b'\\') if dir == Dir::South => vec![Dir::East],
                        Some(b'\\') if dir == Dir::North => vec![Dir::West],
                        Some(b'\\') if dir == Dir::East => vec![Dir::South],
                        Some(b'\\') if dir == Dir::West => vec![Dir::North],
                        _ => vec![dir],
                    }
                    .into_iter()
                    .flat_map(move |dir| coord.walk(dir, size).map(|coord| (coord, dir)))
                    .collect_vec()
                })
                .collect_vec();
            if beams.is_empty() {
                Err(())
            } else {
                Ok(beams)
            }
        });
        seen.iter().map(|(coord, _)| coord).unique().count()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Ok(Self::energize(&tiles, size, Coord(0, 0), Dir::East))
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        let verticals = (0..size.0)
            .flat_map(|x| {
                vec![
                    (Coord(x, 0), Dir::South),
                    (Coord(x, size.1 - 1), Dir::North),
                ]
            })
            .collect_vec();
        let horizontals = (0..size.1)
            .flat_map(|y| vec![(Coord(0, y), Dir::East), (Coord(size.0 - 1, y), Dir::West)])
            .collect_vec();
        verticals
            .into_iter()
            .chain(horizontals)
            .map(|(coord, dir)| Self::energize(&tiles, size, coord, dir))
            .max()
            .ok_or(AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day16 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
            46,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day16 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
            51,
        );
    }
}
