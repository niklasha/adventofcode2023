use crate::day::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day21 {}

type Output = usize;

impl Day for Day21 {
    fn tag(&self) -> &str {
        "21"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 64));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 26501365));
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    fn walk(self, dir: Dir, size: Coord, is_infinite: bool) -> Option<Self> {
        if !is_infinite
            && (self.0 == 0 && dir == Dir::West
                || self.1 == 0 && dir == Dir::North
                || self.0 == size.0 - 1 && dir == Dir::East
                || self.1 == size.1 - 1 && dir == Dir::South)
        {
            None
        } else {
            Some(Coord(
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
            ))
        }
    }

    fn narrow(&self, size: Coord) -> Coord {
        Coord(self.0.rem_euclid(size.0), self.1.rem_euclid(size.1))
    }

    fn moves(
        self,
        tiles: &HashMap<Coord, Tile>,
        size: Coord,
        is_infinite: bool,
    ) -> Vec<(Dir, Coord)> {
        [Dir::East, Dir::South, Dir::West, Dir::North]
            .into_iter()
            .flat_map(|dir| {
                self.walk(dir, size, is_infinite)
                    .filter(|coord| {
                        tiles
                            .get(&coord.narrow(size))
                            .map_or(false, |tile| *tile == Tile::Plot)
                    })
                    .map(|coord| (dir, coord))
            })
            .collect_vec()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Plot,
    Rock,
}

impl Day21 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<Coord, Tile>, Coord, Coord)> {
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .try_fold(
                (HashMap::new(), Coord(0, 0), None),
                |(mut map, mut size, mut start), (y, rs)| {
                    if let Ok(x) = rs.as_ref().map(|s| s.len()) {
                        if x as isize > size.0 {
                            size.0 = x as isize
                        }
                    }
                    let _ = rs.map_err(|_| AocError).map(|s| {
                        s.bytes().enumerate().for_each(|(x, b)| {
                            map.insert(
                                Coord(x as isize, y as isize),
                                match b {
                                    b'.' => Tile::Plot,
                                    b'S' => {
                                        start = Some(Coord(x as isize, y as isize));
                                        Tile::Plot
                                    }
                                    b'#' => Tile::Rock,
                                    _ => panic!(),
                                },
                            );
                        })
                    });
                    size.1 = y as isize + 1;
                    Ok((map, size, start))
                },
            )
            .map(|(map, size, start)| (map, size, start.unwrap()))
    }

    fn get_cycle(tiles: &HashMap<Coord, Tile>, size: Coord, start: Coord) -> (usize, usize) {
        let mut seen = VecDeque::<(Vec<Coord>, usize)>::new();
        (0usize..)
            .try_fold(vec![start], |states, i| {
                let states = states
                    .into_iter()
                    .flat_map(|coord| {
                        coord
                            .moves(tiles, size, true)
                            .into_iter()
                            .map(|(_, coord)| coord)
                            .collect_vec()
                    })
                    .unique()
                    .sorted()
                    .collect_vec();

                let zoom = |map: (isize, isize)| {
                    states
                        .iter()
                        .copied()
                        .filter(|&Coord(x, y)| {
                            x.div_euclid(size.0) == map.0 && y.div_euclid(size.1) == map.1
                        })
                        .map(|Coord(x, y)| Coord(x.rem_euclid(size.0), y.rem_euclid(size.1)))
                        .sorted()
                        .collect_vec()
                };

                let zoomed = zoom((0, 0));
                if let Some((_, start)) = seen.iter().find(|(state, _)| state == &zoomed) {
                    Err((zoomed.len(), seen.pop_back().unwrap().0.len()))
                } else {
                    seen.push_back((zoomed, i));
                    if seen.len() > 10 {
                        seen.pop_front();
                    }
                    Ok(states)
                }
            })
            .unwrap_err()
    }

    fn track(
        tiles: &HashMap<Coord, Tile>,
        size: Coord,
        start: Coord,
        steps: usize,
        stable_size: Option<(usize, usize)>,
    ) -> Output {
        let mut stable_states = HashMap::new();
        let mut entrypoints = HashMap::new();
        let mut stable_maps = HashMap::new();

        let states = (0..steps).fold(vec![start], |states, i| {
            let states = states
                .into_iter()
                .flat_map(|coord| {
                    coord
                        .moves(tiles, size, stable_size.is_some())
                        .into_iter()
                        .map(|(_, coord)| coord)
                        .collect_vec()
                })
                .unique()
                .sorted()
                .collect_vec();

            // Look for new maps entered, and figure out entrypoints and their time.
            let new_entrypoints = states
                .iter()
                .map(|coord| {
                    (
                        (coord.0.div_euclid(size.0), coord.1.div_euclid(size.1)),
                        Coord(coord.0.rem_euclid(size.0), coord.1.rem_euclid(size.1)),
                    )
                })
                .filter(|(map, _)| !entrypoints.contains_key(map))
                .collect_vec();
            if !new_entrypoints.is_empty() {
                new_entrypoints.into_iter().for_each(|(map, coord)| {
                    entrypoints
                        .entry(map)
                        .or_insert(HashSet::new())
                        .insert((coord, i));
                });
                //println!("entrypoints {:?}", entrypoints);
            }

            // zoom in on a specified map
            let zoom = |map: (isize, isize)| {
                states
                    .iter()
                    .copied()
                    .filter(|&Coord(x, y)| {
                        x.div_euclid(size.0) == map.0 && y.div_euclid(size.1) == map.1
                    })
                    .map(|Coord(x, y)| Coord(x.rem_euclid(size.0), y.rem_euclid(size.1)))
                    .sorted()
                    .collect_vec()
            };

            // Record stable states
            if let Some(stable_size) = stable_size {
                let cnt = zoom((0, 0)).len();
                if cnt == stable_size.0 || cnt == stable_size.1 {
                    let zoomed = zoom((0, 0));
                    if stable_states
                        .insert(zoomed.clone(), cnt == stable_size.0)
                        .is_none()
                    {
                        println!("i {} cnt {}", i, cnt);
                        Self::print(tiles, size, &zoomed, (0, 0));
                    }
                }
            }

            // Check if a formerly unstable map has reached a stable state and if so
            // record it with its time to stability.
            let new_stable_maps = states
                .iter()
                .map(|coord| {
                    let map = (coord.0.div_euclid(size.0), coord.1.div_euclid(size.1));
                    (map, zoom(map))
                })
                .filter(|(map, states)| !stable_maps.contains_key(map))
                .flat_map(|(map, states)| {
                    stable_states.get(&states).map(|stable_39| (map, stable_39))
                })
                .collect_vec();
            if !new_stable_maps.is_empty() {
                new_stable_maps.into_iter().for_each(|(map, stable_39)| {
                    let entrypoint = entrypoints.get(&map).unwrap().iter().next().unwrap();
                    if stable_maps
                        .entry(map)
                        .or_insert(HashSet::new())
                        .insert((i - entrypoint.1, entrypoint.0))
                    {
                        println!("new stable map {:?} at {} {}", map, i, stable_39);
                        // Please find out how the maps outside this one looks at this point.
                        // NW
                        if map.0 < 0 && map.1 < 0 {
                            println!("WNW");
                            let zoomed = zoom((map.0 - 1, map.1));
                            Self::print(tiles, size, &zoomed, (map.0 - 1, map.1));
                            println!("NNW");
                            let zoomed = zoom((map.0, map.1 - 1));
                            Self::print(tiles, size, &zoomed, (map.0, map.1 - 1));
                        }
                    }
                });
            }

            states
        });
        states.len()
    }

    fn print(tiles: &HashMap<Coord, Tile>, size: Coord, states: &Vec<Coord>, map: (isize, isize)) {
        for y in 0..size.1 {
            let s = (0..size.0).fold(String::new(), |mut s, x| {
                let coord = Coord(map.0 * size.0 + x, map.1 * size.1 + y);
                s.push_str(match tiles.get(&coord) {
                    None => "",
                    Some(Tile::Plot) => {
                        if states.contains(&coord) {
                            "O"
                        } else {
                            "."
                        }
                    }
                    Some(Tile::Rock) => "#",
                });
                s
            });
            println!("{}", s);
        }
    }

    fn part1_impl(&self, input: &mut dyn io::Read, steps: usize) -> BoxResult<Output> {
        let (tiles, size, start) = Self::parse(input)?;
        Ok(Self::track(&tiles, size, start, steps, None))
    }

    fn part2_impl(&self, input: &mut dyn io::Read, steps: usize) -> BoxResult<Output> {
        let (tiles, size, start) = Self::parse(input)?;
        let steps = steps as isize;

        // XXX This solution is for an expected symmetrical configuration.  Assert the input
        // XXX is of the expected kind.
        assert!(size.0 % 2 == 1);
        assert!(start.0 == size.0 / 2);
        assert!(size.0 == size.1);
        assert!(start.0 == start.1);
        //assert!((steps - start.0) % size.0 == 0);
        let width = (steps - start.0) / size.0;
        assert!(width % 2 == 0);
        // There are a number of different masks to apply to the map.
        // One each for the N, E, S and W corners
        // One each for the HE, SE, SW and NW edges
        // Two each for the internal maps (offset at even or odd offsets.
        let internal_cnt_odd = (width - 1) * (width - 1);
        let internal_cnt_even = width * width;
        let edge_cnt = width - 1;

        let (even_rocks, odd_rocks, se, nw, sw, ne, s, n, w, e) = tiles.iter().fold(
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            |(
                mut even_rocks,
                mut odd_rocks,
                mut se,
                mut nw,
                mut sw,
                mut ne,
                mut s,
                mut n,
                mut w,
                mut e,
            ),
             (Coord(x, y), tile)| {
                match tile {
                    Tile::Rock => {
                        if (x + y) % 2 == 0 {
                            even_rocks += 1
                        } else {
                            odd_rocks += 1
                        }
                        if x + y < size.0 * 3 / 2 {
                            se += 1;
                            if x + (size.0 - 1 - y) >= size.0 / 2 {
                                s += 1;
                            }
                        }
                        if x + y >= size.0 / 2 {
                            nw += 1;
                            if x + (size.0 - 1 - y) < size.0 * 3 / 2 {
                                n += 1;
                            }
                        }
                        if x + (size.0 - 1 - y) >= size.0 / 2 {
                            sw += 1;
                            if x + y >= size.0 / 2 {
                                w += 1;
                            }
                        }
                        if x + (size.0 - 1 - y) < size.1 * 3 / 2 {
                            ne += 1;
                            if x + y < size.0 * 3 / 2 {
                                e += 1;
                            }
                        }
                    }
                    _ => {}
                }
                (even_rocks, odd_rocks, se, nw, sw, ne, s, n, w, e)
            },
        );

        let cnt_if_no_rocks = (steps + 1) * (steps + 1);

        Ok((cnt_if_no_rocks
            - internal_cnt_odd * odd_rocks
            - internal_cnt_even * even_rocks
            - s
            - n
            - w
            - e
            - edge_cnt * (ne + nw + se + sw)) as usize)
    }

    fn part2_impl_x(&self, input: &mut dyn io::Read, steps: usize) -> BoxResult<Output> {
        let (tiles, size, start) = Self::parse(input)?;

        let stable_size = Self::get_cycle(&tiles, size, start);
        println!("cycle: {:?}", stable_size);

        let hendecades = (steps - 4) / 11;
        fn area(radius: usize) -> usize {
            2 * radius * radius - 2 * radius + 1
        }
        let cnt = if hendecades > 1 {
            // Compute stable maps' contribution (manual inspection shows all are in phase)
            let stable_cnt = area(hendecades)
                + match (steps - 5) % 11 {
                    // No extra stable maps
                    0 | 1 => 0,
                    // NW, NE and SW gets stable
                    3 | 4 | 5 => 3 * hendecades - 3,
                    // SE gets stable
                    6 => 4 * hendecades - 4,
                    // N gets stable
                    7 => 4 * hendecades - 3,
                    // W gets stable
                    8 | 9 => 4 * hendecades - 2,
                    // S & E gets stable,
                    10 => 4 * hendecades,
                    _ => unreachable!(),
                };
            println!(
                "stable plots {} {} {}",
                hendecades,
                stable_cnt,
                stable_cnt
                    * if hendecades % 2 != steps % 2 {
                        stable_size.0
                    } else {
                        stable_size.1
                    }
            );

            // Compute the yet border maps' contribution
            // 1) build the stable border (they are either of the 39 or 42 plot variants)
            // 2) build the unstable border
            // 3) run the remaining 4 steps
            //Self::track(&tiles, size, start_states, 4, true)

            // XXX remove
            Self::track(&tiles, size, start, steps, Some(stable_size));

            stable_cnt
        } else {
            Self::track(&tiles, size, start, steps, Some(stable_size))
        };

        Ok(cnt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, steps: usize, f: Output) {
        assert_eq!(Day21 {}.part1_impl(&mut s.as_bytes(), steps).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            6,
            16,
        );
    }

    fn test2(s: &str, steps: usize, f: Output) {
        assert_eq!(Day21 {}.part2_impl(&mut s.as_bytes(), steps).ok(), Some(f));
    }

    #[test]
    fn part2() {
//         test2(
//             "...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........",
//             6,
//             16,
//         );
//         test2(
//             "...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........",
//             10,
//             50,
//         );
        test2(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            50,
            1594,
        );
        test2(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            100,
            6536,
        );
        test2(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            1000,
            668697,
        );
        test2(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            5000,
            16733044,
        );
    }
}
