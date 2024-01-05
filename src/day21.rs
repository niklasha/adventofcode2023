use crate::day::*;
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
        let mut memo_states = HashMap::new();

        let states = (1..=steps).try_fold(vec![start], |states, step| {
            //println!("step {}", step);
            let states = states
                .into_iter()
                .flat_map(|coord| {
                    coord
                        .moves(tiles, size, stable_size.is_some())
                        .into_iter()
                        .map(|(_, coord)| coord)
                        .filter(|coord| {
                            !stable_maps.contains_key(&(
                                coord.0.div_euclid(size.0),
                                coord.1.div_euclid(size.1),
                            ))
                        })
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
                        .insert((coord, step));
                });
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
                let zoomed = zoom((0, 0));
                let cnt = zoomed.len();
                if cnt == stable_size.0 || cnt == stable_size.1 {
                    if stable_states
                        .insert(zoomed.len(), cnt == stable_size.0)
                        .is_none()
                    {
                        //println!("step {} cnt {}", step, cnt);
                        Self::print(tiles, size, &zoomed, (0, 0));
                    }
                }
            }

            for map in [
                (0, -1),
                (1, 0),
                (0, 1),
                (-1, 0),
                (1, -1),
                (1, 1),
                (-1, 1),
                (-1, -1),
            ] {
                let zoomed = zoom(map);
                if !zoomed.is_empty() {
                    let entry = entrypoints
                        .get(&map)
                        .unwrap()
                        .iter()
                        .map(|(_, entry)| entry)
                        .min()
                        .unwrap();
                    memo_states.insert((map, step - *entry), zoom(map).len());
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
                    stable_states.get(&states.len()).map(|lower| (map, lower))
                })
                .collect_vec();
            if !new_stable_maps.is_empty() {
                new_stable_maps.into_iter().for_each(|(map, lower)| {
                    let entrypoint = entrypoints.get(&map).unwrap().iter().next().unwrap();
                    if stable_maps
                        .entry(map)
                        .or_insert(HashSet::new())
                        .insert((step - entrypoint.1, entrypoint.0))
                    {
                        //println!("new stable map {:?} at {} {}", map, step, lower);
                    }
                });
            }
            // We have all info we need when the inner 3x3 box is stable.
            if (0..9)
                .map(|i| (i % 3 - 1, i / 3 - 1))
                .all(|map| stable_maps.contains_key(&map))
            {
                Err(())
            } else {
                Ok(states)
            }
        });
        if let Ok(states) = states {
            states.len()
        } else {
            // Assume step count 26501365 then the number of stable maps will be
            // 202299 ^ 2 * odd (low) stability
            // + 202300 ^ 2 * even (high) stability
            // + 202299 * sum of the four corner maps' memoized state count for step 130
            // + 202300 * sum of the four corner maps' memoized state count for step 64
            // + sum of the four non-corner maps' memoized state count for step 65
            assert_eq!(size.0, size.1);
            let stable_maps_width = steps / size.0 as Output;
            stable_maps_width * stable_maps_width * stable_size.unwrap().0
                + (stable_maps_width + 1) * (stable_maps_width + 1) * stable_size.unwrap().1
                + stable_maps_width
                    * ([(-1, -1), (-1, 1), (1, -1), (1, 1)]
                        .iter()
                        .map(|map| memo_states.get(&(*map, 130)).unwrap())
                        .sum::<Output>())
                + (stable_maps_width + 1)
                    * ([(-1, -1), (-1, 1), (1, -1), (1, 1)]
                        .iter()
                        .map(|map| memo_states.get(&(*map, 64)).unwrap())
                        .sum::<Output>())
                + ([(0, -1), (-1, 0), (1, 0), (0, 1)]
                    .iter()
                    .map(|map| memo_states.get(&(*map, 65)).unwrap())
                    .sum::<Output>())
        }
    }

    fn print(tiles: &HashMap<Coord, Tile>, size: Coord, states: &[Coord], map: (isize, isize)) {
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
            //println!("{}", s);
        }
    }

    fn part1_impl(&self, input: &mut dyn io::Read, steps: usize) -> BoxResult<Output> {
        let (tiles, size, start) = Self::parse(input)?;
        Ok(Self::track(&tiles, size, start, steps, None))
    }

    // fn time_to_stable(
    //     tiles: &HashMap<Coord, Tile>,
    //     size: Coord,
    //     start: Coord,
    //     stable_sizes: (usize, usize),
    // ) -> usize {
    //     (1..)
    //         .try_fold(vec![start], |states, i| {
    //             let states = states
    //                 .into_iter()
    //                 .flat_map(|coord| {
    //                     coord
    //                         .moves(tiles, size, true)
    //                         .into_iter()
    //                         .map(|(_, coord)| coord)
    //                         .collect_vec()
    //                 })
    //                 .unique()
    //                 .sorted()
    //                 .collect_vec();
    //
    //             // zoom in on a specified map
    //             let zoom = |map: (isize, isize)| {
    //                 states
    //                     .iter()
    //                     .copied()
    //                     .filter(|&Coord(x, y)| {
    //                         x.div_euclid(size.0) == map.0 && y.div_euclid(size.1) == map.1
    //                     })
    //                     .map(|Coord(x, y)| Coord(x.rem_euclid(size.0), y.rem_euclid(size.1)))
    //                     .sorted()
    //                     .collect_vec()
    //             };
    //
    //             let cnt = zoom((0, 0)).len();
    //             if cnt == stable_sizes.0 || cnt == stable_sizes.1 {
    //                 Err(i)
    //             } else {
    //                 Ok(states)
    //             }
    //         })
    //         .unwrap_err()
    // }
    //
    // fn time_to_corners(
    //     tiles: &HashMap<Coord, Tile>,
    //     size: Coord,
    //     start: Coord,
    // ) -> HashMap<Coord, usize> {
    //     (1..)
    //         .try_fold(
    //             (
    //                 vec![start],
    //                 HashSet::from([
    //                     Coord(0, 0),
    //                     Coord(0, size.1 - 1),
    //                     Coord(size.0 - 1, 0),
    //                     Coord(size.0 - 1, size.1 - 1),
    //                 ]),
    //                 HashMap::new(),
    //             ),
    //             |(states, mut corners, mut map), i| {
    //                 let states = states
    //                     .into_iter()
    //                     .flat_map(|coord| {
    //                         coord
    //                             .moves(tiles, size, true)
    //                             .into_iter()
    //                             .map(|(_, coord)| coord)
    //                             .collect_vec()
    //                     })
    //                     .unique()
    //                     .sorted()
    //                     .map(|coord| {
    //                         if corners.contains(&coord) {
    //                             corners.remove(&coord);
    //                             map.insert(coord, i);
    //                         }
    //                         coord
    //                     })
    //                     .collect_vec();
    //                 if corners.is_empty() {
    //                     Err(map)
    //                 } else {
    //                     Ok((states, corners, map))
    //                 }
    //             },
    //         )
    //         .unwrap_err()
    // }
    //
    // fn time_to_stable_map(
    //     tiles: &HashMap<Coord, Tile>,
    //     size: Coord,
    //     stable_sizes: (usize, usize),
    // ) -> HashMap<Coord, usize> {
    //     [
    //         Coord(0, 0),
    //         Coord(0, size.1 - 1),
    //         Coord(size.0 - 1, 0),
    //         Coord(size.0 - 1, size.1 - 1),
    //     ]
    //     .into_iter()
    //     .map(|coord| {
    //         (
    //             coord,
    //             Self::time_to_stable(tiles, size, coord, stable_sizes),
    //         )
    //     })
    //     .collect()
    // }

    fn part2_impl(&self, input: &mut dyn io::Read, steps: usize) -> BoxResult<Output> {
        let (tiles, size, start) = Self::parse(input)?;

        let stable_sizes = Self::get_cycle(&tiles, size, start);
        //println!("cycle: {:?}", stable_sizes);
        // let time_to_stable_map = Self::time_to_stable_map(&tiles, size, stable_sizes);
        // println!("time_to_stable_map: {:?}", time_to_stable_map);
        // let time_to_corners = Self::time_to_corners(&tiles, size, start);
        // println!("time_to_corners: {:?}", time_to_corners);

        Ok(Self::track(&tiles, size, start, steps, Some(stable_sizes)))
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
            6,
            16,
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
            10,
            50,
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
