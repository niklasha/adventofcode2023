use crate::day::*;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

pub struct Day10 {}

type Output = usize;

impl Day for Day10 {
    fn tag(&self) -> &str {
        "10"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    // Check if a part reachable in this direction is valid to connect to.
    fn valid(self, b: u8) -> bool {
        match self {
            Dir::East => b == b'-' || b == b'J' || b == b'7',
            Dir::South => b == b'|' || b == b'L' || b == b'J',
            Dir::West => b == b'-' || b == b'L' || b == b'F',
            Dir::North => b == b'|' || b == b'7' || b == b'F',
        }
    }

    // Return the direction going out of a part in this direction.
    fn walk(self, b: u8) -> BoxResult<Self> {
        match (self, b) {
            (Dir::East, b'-') | (Dir::West, b'-') | (Dir::South, b'|') | (Dir::North, b'|') => {
                Ok(self)
            }
            (Dir::East, b'J') | (Dir::West, b'L') => Ok(Dir::North),
            (Dir::East, b'7') | (Dir::West, b'F') => Ok(Dir::South),
            (Dir::South, b'L') | (Dir::North, b'F') => Ok(Dir::East),
            (Dir::South, b'J') | (Dir::North, b'7') => Ok(Dir::West),
            _ => Err(AocError.into()),
        }
    }
}

impl Coord {
    // Compute the next coordinate in a direction.
    fn walk(self, dir: Dir) -> Option<Self> {
        if self.0 == 0 && dir == Dir::West || self.1 == 0 && dir == Dir::North {
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

impl Day10 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<Coord, u8>, Coord)> {
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .try_fold(HashMap::new(), |mut map, (y, rs)| {
                let _ = rs.map_err(|_| AocError).map(|s| {
                    s.bytes().enumerate().for_each(|(x, b)| {
                        map.insert(Coord(x, y), b);
                    })
                });
                Ok(map)
            })
            .and_then(|map| {
                Ok((
                    map.clone(), // XXX
                    map.iter()
                        .find(|(_, &b)| b == b'S')
                        .map(|(c, _)| *c)
                        .ok_or(AocError)?,
                ))
            })
    }

    fn start_dir(map: &HashMap<Coord, u8>, c: Coord) -> BoxResult<Dir> {
        [Dir::East, Dir::South, Dir::West, Dir::North]
            .into_iter()
            .find(|&dir| {
                let &b = map.get(&c.walk(dir).unwrap()).unwrap(); // XXX
                dir.valid(b)
            })
            .ok_or(AocError.into())
    }

    // Infer a loop part given its neighbours.
    fn infer(map: &mut HashMap<Coord, u8>, start: &Coord) -> BoxResult<()> {
        let v = [Dir::East, Dir::South, Dir::West, Dir::North]
            .into_iter()
            .map(|dir| {
                if let Some(neighbour) = start.walk(dir) {
                    let &b = map.get(&neighbour).unwrap(); // XXX
                    dir.valid(b)
                } else {
                    false
                }
            })
            .collect::<Vec<bool>>();
        *map.get_mut(start).ok_or(AocError)? = match v[..] {
            [true, true, false, false] => Ok(b'F'),
            [true, false, true, false] => Ok(b'-'),
            [true, false, false, true] => Ok(b'L'),
            [false, true, true, false] => Ok(b'7'),
            [false, true, false, true] => Ok(b'|'),
            [false, false, true, true] => Ok(b'J'),
            _ => Err(AocError),
        }?;
        Ok(())
    }

    fn detect_loop(map: &HashMap<Coord, u8>, start: Coord) -> BoxResult<Vec<Coord>> {
        let dir = Self::start_dir(map, start)?;
        let mut seen = HashMap::new();
        seen.insert(start, 0);
        let seen = (1 as Output..)
            .try_fold((start, dir, seen), |(c, dir, mut seen), i| {
                let next = c.walk(dir).ok_or(Err(AocError))?;
                if let Vacant(e) = seen.entry(next) {
                    e.insert(i);
                    let &b = map.get(&next).ok_or(AocError).map_err(Err)?;
                    Ok((next, dir.walk(b).unwrap(), seen)) // XXX
                } else {
                    Err(Ok(seen))
                }
            })
            .and(Err(Err(AocError)))
            .or_else(|rv| rv)?;
        Ok(seen
            .into_iter()
            .sorted_by_key(|&(_, i)| i)
            .map(|(c, _)| c)
            .collect_vec())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (map, start) = Self::parse(input)?;
        Ok(Self::detect_loop(&map, start)?.len() / 2)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (mut map, start) = Self::parse(input)?;
        Self::infer(&mut map, &start)?;
        let v = Self::detect_loop(&map, start)?;
        // Erase all junk
        map.iter_mut().for_each(|(c, b)| {
            if !v.contains(c) {
                *b = b'.'
            }
        });
        let &min_x = map.iter().map(|(Coord(x, _), _)| x).min().ok_or(AocError)?;
        let &max_x = map.iter().map(|(Coord(x, _), _)| x).max().ok_or(AocError)?;
        let &min_y = map.iter().map(|(Coord(_, y), _)| y).min().ok_or(AocError)?;
        let &max_y = map.iter().map(|(Coord(_, y), _)| y).max().ok_or(AocError)?;
        Ok((min_y..=max_y).fold(0 as Output, |n, y| {
            (min_x..=max_x)
                .fold(
                    (n, false, None),
                    |(mut n, mut is_inside, mut horizontal_from_north), x| {
                        let c = Coord(x, y);
                        match map.get(&c).unwrap() /* XXX */ {
                        b'|'  => {
                            is_inside = !is_inside;
                        }
                        b'L' => {
                            horizontal_from_north = Some(true);
                        }
                        b'F' => {
                            horizontal_from_north = Some(false);
                        }
                        b'7' if horizontal_from_north == Some(true) => {
                            is_inside = !is_inside;
                            horizontal_from_north = None;
                        }
                        b'J' if horizontal_from_north == Some(false) => {
                            is_inside = !is_inside;
                            horizontal_from_north = None;
                        }
                        b'.' => if is_inside {
                            n += 1;
                        }
                        _ => (),
                    }
                        (n, is_inside, horizontal_from_north)
                    },
                )
                .0
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day10 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
            4,
        );
        test1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
            8,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day10 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
            4,
        );
        test2(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
            4,
        );
        test2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
            8,
        );
        test2(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
            10,
        );
    }
}
