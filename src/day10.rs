use crate::day::*;
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
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

impl Dir {
    fn valid(self, b: u8) -> bool {
        match self {
            Dir::EAST => b == b'-' || b == b'J' || b == b'7',
            Dir::SOUTH => b == b'|' || b == b'L' || b == b'J',
            Dir::WEST => b == b'-' || b == b'L' || b == b'F',
            Dir::NORTH => b == b'|' || b == b'7' || b == b'F',
        }
    }

    fn walk(self, b: u8) -> BoxResult<Self> {
        match (self, b) {
            (Dir::EAST, b'-') | (Dir::WEST, b'-') | (Dir::SOUTH, b'|') | (Dir::NORTH, b'|') => {
                Ok(self)
            }
            (Dir::EAST, b'J') | (Dir::WEST, b'L') => Ok(Dir::NORTH),
            (Dir::EAST, b'7') | (Dir::WEST, b'F') => Ok(Dir::SOUTH),
            (Dir::SOUTH, b'L') | (Dir::NORTH, b'F') => Ok(Dir::EAST),
            (Dir::SOUTH, b'J') | (Dir::NORTH, b'7') => Ok(Dir::WEST),
            _ => Err(AocError.into()),
        }
    }
}

impl Coord {
    fn walk(self, dir: Dir) -> Self {
        Coord(
            (self.0 as i64
                + match dir {
                    Dir::EAST => 1,
                    Dir::WEST => -1,
                    _ => 0,
                }) as usize,
            (self.1 as i64
                + match dir {
                    Dir::SOUTH => 1,
                    Dir::NORTH => -1,
                    _ => 0,
                }) as usize,
        )
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
        [Dir::EAST, Dir::SOUTH, Dir::WEST, Dir::NORTH]
            .into_iter()
            .find(|&dir| {
                let &b = map.get(&c.walk(dir)).unwrap(); // XXX
                dir.valid(b)
            })
            .ok_or(AocError.into())
    }

    fn detect_loop(map: &HashMap<Coord, u8>, start: Coord) -> BoxResult<Vec<Coord>> {
        let dir = Self::start_dir(&map, start)?;
        let mut seen = HashMap::new();
        seen.insert(start, 0);
        let seen = (1 as Output..)
            .try_fold((start, dir, seen), |(c, dir, mut seen), i| {
                let next = c.walk(dir);
                if seen.contains_key(&next) {
                    Err(Ok(seen))
                } else {
                    seen.insert(next, i);
                    let &b = map.get(&next).ok_or(AocError).map_err(|e| Err(e))?;
                    Ok((next, dir.walk(b).unwrap(), seen)) // XXX
                }
            })
            .and_then(|_| Err(Err(AocError)))
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
        let v = [Dir::EAST, Dir::SOUTH, Dir::WEST, Dir::NORTH]
            .into_iter()
            .map(|dir| {
                let &b = map.get(&start.walk(dir)).unwrap(); // XXX
                dir.valid(b)
            }).collect::<Vec<bool>>();
        *map.get_mut(&start).ok_or(AocError)? = match v[..] {
            [true, true, false, false] => Ok(b'F'),
            [true, false, true, false] => Ok(b'-'),
            [true, false, false, true] => Ok(b'L'),
            [false, true, true, false] => Ok(b'7'),
            [false, true, false, true] => Ok(b'|'),
            [false, false, true, true] => Ok(b'J'),
            _ => Err(AocError)
        }?;
        let v = Self::detect_loop(&map, start)?;
        // Erase all junk
        map.iter_mut().for_each(|(c, mut b)| {
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
                .fold((n, false, None), |(mut n, mut is_inside, mut horizontal), x| {
                    let c = Coord(x, y);
                    println!("{:?} {} {} {}", c, map.get(&c).unwrap(), n, is_inside);
                    match map.get(&c).unwrap() /* XXX */ {
                        b'|'  => {
                            is_inside = !is_inside;
                        }
                        b'L' => {
                            horizontal = Some(true);
                        }
                        b'F' => {
                            horizontal = Some(false);
                        }
                        b'7' if horizontal == Some(true) => {
                            is_inside = !is_inside;
                            horizontal = None;
                        }
                        b'J' if horizontal == Some(false) => {
                            is_inside = !is_inside;
                            horizontal = None;
                        }
                        b'.' => if is_inside {
                            println!("hooray");
                            n = n + 1;
                        }
                        _ => (),
                    }
                    (n, is_inside, horizontal)
                })
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
//         test2(
//             "F7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L",
//             10,
//         );
    }
}
