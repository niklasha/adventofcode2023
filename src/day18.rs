use crate::day::*;
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct Day18 {}

type Output = isize;

impl Day for Day18 {
    fn tag(&self) -> &str {
        "18"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("^([UDLR]) (\\d*) \\(#([0-9a-f]{6})\\)$").unwrap();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coord(isize, isize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Coord {
    // Compute the next coordinate in a direction.
    fn walk(self, dir: Dir) -> Self {
        Coord(
            self.0
                + match dir {
                    Dir::East => 1,
                    Dir::West => -1,
                    _ => 0,
                },
            self.1
                + match dir {
                    Dir::South => 1,
                    Dir::North => -1,
                    _ => 0,
                },
        )
    }
}

impl Day18 {
    fn parse_naive(
        input: &mut dyn io::Read,
        use_colour: bool,
    ) -> BoxResult<(HashMap<Coord, (Dir, Option<Dir>)>, Coord, Coord)> {
        let (mut map, coord, from_dir, start) = io::BufReader::new(input).lines().try_fold(
            (HashMap::new(), Coord(0, 0), None, None),
            |(mut map, coord, from_dir, start), rs| {
                let s = rs.map_err(|_| AocError)?;
                let (_, [dir, len, colour]) = PATTERN.captures(&s).ok_or(AocError)?.extract();
                let dir = if use_colour {
                    match colour.chars().last() {
                        Some('3') => Ok(Dir::North),
                        Some('1') => Ok(Dir::South),
                        Some('2') => Ok(Dir::West),
                        Some('0') => Ok(Dir::East),
                        _ => Err(AocError),
                    }
                } else {
                    match dir {
                        "U" => Ok(Dir::North),
                        "D" => Ok(Dir::South),
                        "L" => Ok(Dir::West),
                        "R" => Ok(Dir::East),
                        _ => Err(AocError),
                    }
                }?;
                let len: usize = if use_colour {
                    usize::from_str_radix(&colour[..(colour.len() - 1)], 16)?
                } else {
                    len.parse()?
                };
                BoxResult::Ok((0..len).fold(
                    (map, coord, from_dir, start),
                    |(mut map, coord, from_dir, start), _| {
                        map.insert(coord, (dir, from_dir));
                        let coord = coord.walk(dir);
                        (map, coord, Some(dir), start.or(Some(coord)))
                    },
                ))
            },
        )?;
        if coord != Coord(0, 0) {
            Err(AocError)?;
        }
        let start_mut = map.get_mut(&Coord(0, 0)).ok_or(AocError)?;
        (*start_mut).1 = from_dir;
        if let (Some(min), Some(max)) =
            map.iter()
                .fold((None, None), |(min, max), (Coord(x, y), _)| {
                    let min_x = min.map_or(*x, |Coord(min, _)| isize::min(min, *x));
                    let max_x = max.map_or(*x, |Coord(max, _)| isize::max(max, *x));
                    let min_y = min.map_or(*y, |Coord(_, min)| isize::min(min, *y));
                    let max_y = max.map_or(*y, |Coord(_, max)| isize::max(max, *y));
                    (Some(Coord(min_x, min_y)), Some(Coord(max_x, max_y)))
                })
        {
            Ok((map, min, max))
        } else {
            Err(AocError)?
        }
    }

    fn parse(
        input: &mut dyn io::Read,
        use_colour: bool,
    ) -> BoxResult<BTreeMap<isize, Vec<(isize, isize)>>> {
        Ok(io::BufReader::new(input)
            .lines()
            .try_fold(
                (BTreeMap::new(), Coord(0, 0)),
                |(mut map, mut coord), rs| {
                    let s = rs.map_err(|_| AocError)?;
                    let (_, [dir, len, colour]) = PATTERN.captures(&s).ok_or(AocError)?.extract();
                    let dir = if use_colour {
                        match colour.chars().last() {
                            Some('3') => Ok(Dir::North),
                            Some('1') => Ok(Dir::South),
                            Some('2') => Ok(Dir::West),
                            Some('0') => Ok(Dir::East),
                            _ => Err(AocError),
                        }
                    } else {
                        match dir {
                            "U" => Ok(Dir::North),
                            "D" => Ok(Dir::South),
                            "L" => Ok(Dir::West),
                            "R" => Ok(Dir::East),
                            _ => Err(AocError),
                        }
                    }?;
                    let len: isize = if use_colour {
                        isize::from_str_radix(&colour[..(colour.len() - 1)], 16)?
                    } else {
                        len.parse()?
                    };
                    match dir {
                        Dir::East => {
                            let extent = map.entry(coord.1).or_insert(vec![]);
                            (*extent).push((coord.0, coord.0 + len));
                            (*extent).sort();
                            coord.0 += len;
                        }
                        Dir::South => {
                            coord.1 += len;
                        }
                        Dir::West => {
                            let extent = map.entry(coord.1).or_insert(vec![]);
                            (*extent).push((coord.0 - len, coord.0));
                            (*extent).sort();
                            coord.0 -= len;
                        }
                        Dir::North => {
                            coord.1 -= len;
                        }
                    }
                    BoxResult::Ok((map, coord))
                },
            )?
            .0)
    }

    fn compute_naive(
        map: HashMap<Coord, (Dir, Option<Dir>)>,
        min: Coord,
        max: Coord,
    ) -> BoxResult<Output> {
        Ok((min.1..=max.1)
            .map(|y| {
                let (_, sum, _) = (min.0..=max.0).fold(
                    (false, 0, None),
                    |(mut is_inside, mut sum, mut horizontal_from_north), x| {
                        if let Some((dir, Some(from_dir))) = map.get(&Coord(x, y)) {
                            match (from_dir, dir) {
                                (Dir::North, Dir::North) | (Dir::South, Dir::South) => {
                                    is_inside = !is_inside;
                                }
                                (Dir::South, Dir::East) | (Dir::West, Dir::North) => {
                                    horizontal_from_north = Some(true);
                                }
                                (Dir::North, Dir::East) | (Dir::West, Dir::South) => {
                                    horizontal_from_north = Some(false);
                                }
                                (Dir::North, Dir::West) | (Dir::East, Dir::South)
                                if horizontal_from_north == Some(true) =>
                                    {
                                        is_inside = !is_inside;
                                        horizontal_from_north = None;
                                    }
                                (Dir::South, Dir::West) | (Dir::East, Dir::North)
                                if horizontal_from_north == Some(false) =>
                                    {
                                        is_inside = !is_inside;
                                        horizontal_from_north = None;
                                    }
                                _ => {}
                            }
                            sum += 1;
                        } else {
                            if is_inside {
                                sum += 1;
                            }
                        }
                        (is_inside, sum, horizontal_from_north)
                    },
                );
                sum
            }).enumerate().map(|(i, s)| { println!("{}: {}", i, s); s })
            .sum())
    }

    fn union(a: &Vec<(isize, isize)>, b: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        let mut u = a.clone();
        u.extend_from_slice(b);
        u.sort();
        u.iter()
            .fold((None, vec![]), |(last, mut u), &e @ (x0, x1)| {
                let last = if let Some(last) = last {
                    if x0 <= last {
                        if x1 > last {
                            u.push((last + 1, x1));
                            x1
                        } else {
                            last
                        }
                    } else {
                        u.push(e);
                        x1
                    }
                } else {
                    u.push(e);
                    x1
                };
                (Some(last), u)
            })
            .1
    }

    fn extend(extents: &mut Vec<(isize, isize)>, extent: (isize, isize)) {
        if let Some(last) = extents.last_mut() {
            if last.1 + 1 == extent.0 {
                last.1 = extent.1;
            } else {
                extents.push(extent);
            }
        } else {
            extents.push(extent);
        }
    }

    fn get_next(current: &Vec<(isize, isize)>, new: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        let mut ei = current.iter();
        let mut ni = new.iter();
        let mut next = vec![];
        let mut extent: Option<(isize, isize)> = None;
        let mut new_extent: Option<(isize, isize)> = None;
        while true {
            extent = extent.or_else(|| ei.next().copied());
            new_extent = new_extent.or_else(|| ni.next().copied());
            if extent.is_none() {
                if let Some(new_extent) = new_extent {
                    Self::extend(&mut next, new_extent);
                }
                next.extend(ni);
                break;
            }
            if new_extent.is_none() {
                if let Some(extent) = extent {
                    Self::extend(&mut next, extent);
                }
                next.extend(ei);
                break;
            }
            if let (Some(e @ (e0, e1)), Some(n @ (n0, n1))) = (extent, new_extent) {
                if e1 <= n0 {
                    Self::extend(&mut next, e);
                    if n0 == e1 {
                        new_extent = Some((e1 + 1, n1))
                    }
                    extent = None;
                } else if n1 <= e0 {
                    if n1 < e0 {
                        Self::extend(&mut next, n);
                    } else {
                        Self::extend(&mut next, (n0, e0 - 1));
                    }
                    new_extent = None;
                } else if e0 <= n0 {
                    if e0 < n0 {
                        Self::extend(&mut next, (e0, n0));
                    }
                    extent = if n1 < e1 { Some((n1, e1)) } else { None };
                    new_extent = None;
                }
            }
        }
        next
    }

    fn compute(all_extents: BTreeMap<isize, Vec<(isize, isize)>>) -> BoxResult<Output> {
        Ok(all_extents
            .iter()
            .fold(
                (vec![] as Vec<(isize, isize)>, 0 as Output, None),
                |(extents, sum, last_y), (&y, new_extents)| {
                    // Add the cornerless rows we have jumped over
                    let sum = sum + if let Some(last_y) = last_y {
                        extents
                            .iter()
                            .map(|&(x0, x1)| (y - last_y - 1) * (x1 - x0 + 1))
                            .sum()
                    } else {
                        sum
                    };

                    // Compute the extents to use below this row
                    let next_extents = Self::get_next(&extents, new_extents);

                    // Add the union of the extents from above and the upcoming to the sum
                    let sum = sum
                        + Self::union(&extents, &next_extents)
                        .iter()
                        .map(|(x0, x1)| x1 - x0 + 1)
                        .sum::<Output>();
                    (next_extents, sum, Some(y))
                },
            )
            .1)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (all_extents) = Self::parse(input, false)?;
        Self::compute(all_extents)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (all_extents) = Self::parse(input, true)?;
        Self::compute(all_extents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day18 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
            62,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day18 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
            952408144115,
        );
    }
}
