use crate::day::*;
use petgraph::algo::{dijkstra, Measure};
use petgraph::prelude::*;
use petgraph::visit::{IntoEdges, VisitMap, Visitable};
use regex::Regex;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub struct Day17 {}

type Output = usize;

impl Day for Day17 {
    fn tag(&self) -> &str {
        "17"
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord(usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn opposite(self) -> Dir {
        match self {
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
        }
    }
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

    fn moves(self, size: Coord) -> Vec<(Dir, Coord)> {
        [Dir::East, Dir::South, Dir::West, Dir::North]
            .into_iter()
            .flat_map(|dir| self.walk(dir, size).map(|coord| (dir, coord)))
            .collect_vec()
    }

    fn distance(self, other: Self) -> usize {
        self.0.max(other.0) - self.0.min(other.0) + self.1.max(other.1) - self.1.min(other.1)
    }
}

// based on petgraph 0.6.4
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

// based on petgraph 0.6.4
pub fn custom_dijkstra<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    mut edge_cost: F,
) -> HashMap<G::NodeId, K>
where
    G: IntoEdges + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
{
    let mut visited = graph.visit_map();
    let mut scores = HashMap::new();
    //let mut predecessor = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    let zero_score = K::default();
    scores.insert(start, zero_score);
    visit_next.push(MinScored(zero_score, start));
    while let Some(MinScored(node_score, node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        if goal.as_ref() == Some(&node) {
            break;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }
            let next_score = node_score + edge_cost(edge);
            match scores.entry(next) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        *ent.into_mut() = next_score;
                        visit_next.push(MinScored(next_score, next));
                        //predecessor.insert(next.clone(), node.clone());
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    visit_next.push(MinScored(next_score, next));
                    //predecessor.insert(next.clone(), node.clone());
                }
            }
        }
        visited.visit(node);
    }
    scores
}

impl Day17 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<Coord, Output>, Coord)> {
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
                        map.insert(Coord(x, y), (b - b'0') as Output);
                    })
                });
                size.1 = y + 1;
                Ok((map, size))
            },
        )
    }

    fn compute_dijkstra(
        tiles: &HashMap<Coord, Output>,
        size: Coord,
        start: Coord,
        min_len: usize,
        max_len: usize,
    ) -> BoxResult<Output> {
        let finish = Coord(size.0 - 1, size.1 - 1);

        let graph = tiles
            .iter()
            .fold(DiGraphMap::new(), |mut graph, (coord, heat_loss)| {
                for (dir, neighbor) in &coord.moves(size) {
                    let neighbor_heat_loss = *tiles.get(neighbor).unwrap();
                    let opposite_dir = dir.opposite();
                    for origin in [Dir::North, Dir::East, Dir::South, Dir::West] {
                        for straight in 0..max_len {
                            let node = graph.add_node((*coord, origin, straight));
                            let neighbor_node = graph.add_node((*neighbor, origin, straight));
                            if straight < max_len - 1 {
                                if origin == *dir {
                                    graph.add_edge(
                                        node,
                                        (*neighbor, *dir, straight + 1),
                                        neighbor_heat_loss,
                                    );
                                } else if origin == opposite_dir {
                                    graph.add_edge(
                                        neighbor_node,
                                        (*coord, opposite_dir, straight + 1),
                                        *heat_loss,
                                    );
                                }
                            }
                            if origin != *dir && origin != opposite_dir && straight + 1 >= min_len {
                                graph.add_edge(node, (*neighbor, *dir, 0), neighbor_heat_loss);
                                graph.add_edge(neighbor_node, (*coord, *dir, 0), *heat_loss);
                            }
                        }
                    }
                }
                graph
            });

        ((min_len - 1)..max_len)
            .flat_map(|straight| {
                [Dir::South, Dir::East]
                    .into_iter()
                    .cartesian_product([Dir::South, Dir::East])
                    .map(move |(origin_start, origin_finish)| {
                        (
                            (start, origin_start, max_len - 1),
                            (finish, origin_finish, straight),
                        )
                    })
            })
            .map(|(start, finish)| {
                dijkstra(&graph, start, Some(finish), |(_, _, edge)| *edge)
                    .get(&finish)
                    .copied()
                    .unwrap_or(usize::MAX)
            })
            .min()
            .ok_or(AocError.into())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute_dijkstra(&tiles, size, Coord(0, 0), 1, 3)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute_dijkstra(&tiles, size, Coord(0, 0), 4, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day17 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
            102,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day17 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
            94,
        );
        test2(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
            71,
        );
    }
}
