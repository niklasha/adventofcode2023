use petgraph::algo::all_simple_paths;
use petgraph::dot::Dot;
use petgraph::graph::UnGraph;
use petgraph::prelude::UnGraphMap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use crate::day::*;

pub struct Day23 {}

type Output = usize;

impl Day for Day23 {
    fn tag(&self) -> &str {
        "23"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
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
    fn walk(self, dir: Dir, size: Coord) -> Option<Self> {
        if self.0 == 0 && dir == Dir::West
            || self.1 == 0 && dir == Dir::North
            || self.0 == size.0 - 1 && dir == Dir::East
            || self.1 == size.1 - 1 && dir == Dir::South
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

    fn moves(
        self,
        tiles: &HashMap<Coord, Tile>,
        size: Coord,
        is_slippery: Option<bool>,
        stop: Coord,
    ) -> Vec<(Dir, Coord)> {
        [Dir::East, Dir::South, Dir::West, Dir::North]
            .into_iter()
            .flat_map(|dir| {
                self.walk(dir, size)
                    .filter(|coord| match tiles.get(&coord) {
                        Some(Tile::Path) => true,
                        Some(Tile::Slope(slope)) => is_slippery
                            .map_or(*coord == stop, |is_slippery| !is_slippery || dir == *slope),
                        _ => false,
                    })
                    .map(|coord| (dir, coord))
            })
            .collect_vec()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

fn build_graph(
    tiles: &HashMap<Coord, Tile>,
    size: Coord,
    start: Coord,
    stop: Coord,
) -> UnGraphMap<Coord, Output> {
    (0..)
        .try_fold(
            (UnGraphMap::new(), VecDeque::from([(start, None, start, 0)])),
            |(mut graph, mut queue), _| {
                if let Some((coord, previous, mut last_node, mut weight)) = queue.pop_front() {
                    if coord == stop {
                        graph.add_edge(last_node, stop, weight);
                        Ok((graph, queue))
                    } else {
                        let mut neighbors = coord.moves(tiles, size, Some(false), stop);
                        neighbors.retain(|(_, coord)| Some(*coord) != previous);
                        if neighbors.len() > 1 {
                            graph.add_edge(last_node, coord, weight);
                            last_node = coord;
                            weight = 0;
                        }
                        Ok(neighbors.iter().fold(
                            (graph, queue),
                            |(mut graph, mut queue), (_, next)| {
                                if graph.contains_node(*next) {
                                    graph.add_edge(last_node, *next, weight + 1);
                                    queue.remove(
                                        queue
                                            .iter()
                                            .position(|(c, _, _, _)| *c == coord)
                                            .expect(&format!("missing {:?} in queue", coord)),
                                    );
                                } else {
                                    queue.push_front((*next, Some(coord), last_node, weight + 1));
                                }
                                (graph, queue)
                            },
                        ))
                    }
                } else {
                    Err(graph)
                }
            },
        )
        .unwrap_err()
}

impl Day23 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<Coord, Tile>, Coord)> {
        io::BufReader::new(input).lines().enumerate().try_fold(
            (HashMap::new(), Coord(0, 0)),
            |(mut map, mut size), (y, rs)| {
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
                                b'.' => Tile::Path,
                                b'#' => Tile::Forest,
                                b'<' => Tile::Slope(Dir::West),
                                b'>' => Tile::Slope(Dir::East),
                                b'^' => Tile::Slope(Dir::North),
                                b'v' => Tile::Slope(Dir::South),
                                _ => panic!(),
                            },
                        );
                    })
                });
                size.1 = y as isize + 1;
                Ok((map, size))
            },
        )
    }

    fn track(
        tiles: &HashMap<Coord, Tile>,
        size: Coord,
        start: Coord,
        stop: Coord,
        is_slippery: Option<bool>,
        mut seen: HashSet<Coord>,
    ) -> Vec<HashSet<Coord>> {
        seen.insert(start);
        (1..)
            .try_fold((vec![(start, seen)], vec![]), |(states, mut seens), i| {
                let states = states
                    .into_iter()
                    .flat_map(|(coord, mut seen)| {
                        seen.insert(coord);
                        coord
                            .moves(tiles, size, is_slippery, stop)
                            .into_iter()
                            .map(|(_, coord)| {
                                if coord == stop {
                                    seens.push(seen.clone());
                                }
                                coord
                            })
                            .filter(|coord| !seen.contains(coord))
                            .map(|coord| (coord, seen.clone()))
                            .collect_vec()
                    })
                    .collect_vec();
                if states.is_empty() {
                    Err(seens)
                } else {
                    Ok((states, seens))
                }
            })
            .unwrap_err()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::track(
            &tiles,
            size,
            Coord(1, 0),
            Coord(size.0 - 2, size.1 - 1),
            Some(true),
            HashSet::new(),
        )
        .into_iter()
        .map(|seen| seen.len())
        .max()
        .ok_or(AocError.into())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        let start = Coord(1, 0);
        let stop = Coord(size.1 - 2, size.1 - 1);
        let graph = build_graph(&tiles, size, start, stop);
        //println!("{:?}", Dot::new(&graph));
        all_simple_paths::<Vec<_>, _>(&graph, start, stop, 0, None)
            .map(|path| {
                path.into_iter().tuple_windows()
                    .flat_map(|(a, b)| graph.edge_weight(a, b))
                    .sum()
            })
            .max()
            .ok_or(AocError.into())
    }

    fn part2_impl_x(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        let walkable = tiles
            .iter()
            .filter(|&(_, tile)| *tile != Tile::Forest)
            .count();

        // Find all paths between slopes in shortest first order
        // Find all slopes, do a BFS until another slope is found, emit path as an iterator
        // build a new iterator, producing path sets, sorted in total length order
        // build new map from standard map reduced by the current path set
        // Find shortest path in that map
        //

        // Find all shortcuts, and sum their lengthes up.  Remove that from the tile total.
        // First all slope combinations
        let targets = tiles
            .iter()
            .filter(|&(coord, tile)| match tile {
                Tile::Slope(_) => true,
                Tile::Path if *coord == Coord(size.0 - 2, size.1 - 1) => true,
                _ => false,
            })
            .map(|(coord, _)| *coord)
            .collect_vec();

        return (1..)
            .try_fold(
                (vec![(vec![], Coord(1, 0), vec![HashSet::new()])], vec![]),
                |(states, mut reaches), _| {
                    let new_states = states
                        .into_iter()
                        .flat_map(
                            |(filter, start, seens): (Vec<Coord>, _, Vec<HashSet<Coord>>)| {
                                let new_targets = targets
                                    .iter()
                                    .filter(|&target| !filter.contains(target))
                                    .map(|slope| {
                                        (
                                            *slope,
                                            Self::track(
                                                &tiles,
                                                size,
                                                start,
                                                *slope,
                                                None,
                                                seens[0].clone(),
                                            ),
                                        )
                                    })
                                    .filter(|(_, seens)| !seens.is_empty())
                                    .collect_vec();
                                new_targets.iter().for_each(|slope| {
                                    println!("{}: {:?} {}", filter.len(), slope.0, slope.1.len())
                                });
                                if let Some((_, seens)) = new_targets
                                    .iter()
                                    .find(|(coord, seens)| *coord == Coord(size.0 - 2, size.1 - 1))
                                {
                                    let len =
                                        seens.iter().max_by_key(|seen| seen.len()).unwrap().len();
                                    println!("reached in {}", len);
                                    reaches.push(len);
                                }
                                let mut filter = filter.clone();
                                filter.push(start);
                                new_targets.into_iter().map(move |(new_start, new_seens)| {
                                    (filter.clone(), new_start, new_seens.clone())
                                })
                            },
                        )
                        .collect_vec();
                    if new_states.is_empty() {
                        Err(reaches)
                    } else {
                        Ok((new_states, reaches))
                    }
                },
            )
            .unwrap_err()
            .into_iter()
            .max()
            .ok_or(AocError.into());

        let new_slopes = targets
            .iter()
            .map(|slope| {
                (
                    *slope,
                    Self::track(&tiles, size, Coord(1, 0), *slope, None, HashSet::new()),
                )
            })
            .filter(|(_, seens)| !seens.is_empty())
            .collect_vec();
        new_slopes
            .iter()
            .for_each(|slope| println!("1: {:?} {}", slope.0, slope.1.len()));
        new_slopes.into_iter().for_each(|(new_slope, seens)| {
            let new_slopes_2 = targets
                .iter()
                .filter(|&slope| *slope != new_slope)
                .map(|slope| {
                    (
                        *slope,
                        Self::track(&tiles, size, new_slope, *slope, None, seens[0].clone()),
                    )
                })
                .filter(|(_, seens)| !seens.is_empty())
                .collect_vec();
            new_slopes_2
                .iter()
                .for_each(|slope| println!("2: {:?} {}", slope.0, slope.1.len()));
            new_slopes_2.into_iter().for_each(|(new_slope_2, seens)| {
                let new_slopes_3 = targets
                    .iter()
                    .filter(|&slope| *slope != new_slope && *slope != new_slope_2)
                    .map(|slope| {
                        (
                            *slope,
                            Self::track(&tiles, size, new_slope_2, *slope, None, seens[0].clone()),
                        )
                    })
                    .filter(|(_, seens)| !seens.is_empty())
                    .collect_vec();
                new_slopes_3
                    .iter()
                    .for_each(|slope| println!("3: {:?} {}", slope.0, slope.1.len()));
                new_slopes_3.into_iter().for_each(|(new_slope_3, seens)| {
                    targets
                        .iter()
                        .filter(|&slope| {
                            *slope != new_slope && *slope != new_slope_2 && *slope != new_slope_3
                        })
                        .map(|slope| {
                            (
                                *slope,
                                Self::track(
                                    &tiles,
                                    size,
                                    new_slope_3,
                                    *slope,
                                    None,
                                    seens[0].clone(),
                                ),
                            )
                        })
                        .filter(|(_, seens)| !seens.is_empty())
                        .for_each(|(slope, seens)| {
                            println!("{:?} {:?}", slope, seens);
                        });
                });
            });
        });
        let paths = targets
            .into_iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| {
                let seens = Self::track(&tiles, size, a, b, None, HashSet::new());
                if seens.len() > 0 {
                    println!("{:?} -> {:?}: {:?}", a, b, seens);
                }
                (a, b)
            })
            .collect_vec();
        // for each, find all paths, there can be at most
        Ok(walkable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day23 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
            94,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day23 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
            154,
        );
    }
}
