use std::cmp::Ordering;
use crate::day::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Range;

pub struct Day14 {}

type Output = usize;

impl Day for Day14 {
    fn tag(&self) -> &str {
        "14"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
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

#[derive(Clone, Eq, PartialEq)]
struct Dish(HashMap<Coord, u8>);

impl Hash for Dish {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut v = self.0
            .iter()
            .filter(|&(_, &b)| b == b'O')
            .collect_vec();
        v.sort_by(|&(&a, _), &(&b, _)| {
            let x = a.0.cmp(&b.0);
            if x == Ordering::Equal { a.1.cmp(&b.1) } else { x }
        });
        v.hash(state)
    }
}

impl Dish {
    fn print(&self, size: Coord) {
        for y in 0..size.1 {
            let s = (0..size.0).fold(String::new(), |mut s, x| {
                s.push_str(&match self.0.get(&Coord(x, y)) {
                    None => String::new(),
                    Some(b) => String::from(*b as char),
                });
                s
            });
            println!("{}", s);
        }
    }
}
impl Day14 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(Dish, Coord)> {
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .try_fold(
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
            .map(|(map, size)| (Dish(map), size))
    }

    fn tilt(dish: &mut Dish, size: Coord, dir: Dir) {
        let (outer, inner, make_coord): (Vec<_>, Vec<_>, Box<dyn Fn(Output, Output) -> Coord>) =
            match dir {
                Dir::East => (
                    (0..size.1).rev().collect_vec(),
                    (0..size.0).collect_vec(),
                    Box::new(|i, o| Coord(o, i)),
                ),
                Dir::South => (
                    (0..size.0).rev().collect_vec(),
                    (0..size.0).collect_vec(),
                    Box::new(|i, o| Coord(i, o)),
                ),
                Dir::West => (
                    (0..size.1).collect_vec(),
                    (0..size.0).collect_vec(),
                    Box::new(|i, o| Coord(o, i)),
                ),
                Dir::North => (
                    (0..size.0).collect_vec(),
                    (0..size.1).collect_vec(),
                    Box::new(|i, o| Coord(i, o)),
                ),
            };
        for o in &outer {
            for i in &inner {
                let (i, o) = (*i, *o);
                match dish.0.get(&make_coord(i, o)) {
                    Some(b'O') => {
                        let mut v = outer.iter().copied().take_while(|&n| n != o).collect_vec();
                        v.reverse();
                        let o1 = v
                            .into_iter()
                            .position(|o| {
                                dish.0.get(&make_coord(i, o)).map_or(false, |&b| b != b'.')
                            })
                            .map_or(outer[0], |p| match dir {
                                Dir::North | Dir::West => o - p,
                                Dir::South | Dir::East => o + p,
                            });
                        if o != o1 {
                            if let Some(b) = dish.0.remove(&make_coord(i, o)) {
                                //                                println!("{}: {} -> {}", i, o, o1);
                                dish.0.insert(make_coord(i, o), b'.');
                                dish.0.insert(make_coord(i, o1), b);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn load(dish: &Dish, size: Coord, dir: Dir) -> Output {
        // XXX assumes dir is North
        dish.0
            .iter()
            .filter(|&(_, &b)| b == b'O')
            .map(|(Coord(_, y), _)| size.1 - *y)
            .sum()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (mut dish, size) = Self::parse(input)?;
        Self::tilt(&mut dish, size, Dir::North);
        Ok(Self::load(&dish, size, Dir::North))
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (mut dish, size) = Self::parse(input)?;
        let mut seen = HashMap::new();
        let mut left = 4000000000u64;
        let mut i = 0u64;
        let mut cycle_seen = false;
        while left > 0 {
            for dir in [Dir::North, Dir::West, Dir::South, Dir::East] {
                seen.insert(dish.clone(), i);
                Self::tilt(&mut dish, size, dir);
                i += 1;
                left -= 1;
                if !cycle_seen {
                    if let Some(j) = seen.get(&dish) {
                        let period = i - j;
                        left %= period;
                        cycle_seen = true;
                    }
                }
            }
        }
        Ok(Self::load(&dish, size, Dir::North))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day14 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
            136,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day14 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
            64,
        );
    }
}
