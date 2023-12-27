use crate::day::*;
use regex::Regex;
use std::collections::btree_set::Iter;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::iter;

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

    // // Return a vector of long moves (a straight line for min_len to max_len
    // // steps and then a turn, or reaching a stop) and a corresponding track..
    // fn ultra_moves(self, origin: Option<Dir>, size: Coord, min_len: usize, max_len: usize, stop: Coord, seen: &BTreeSet<Coord>) -> Vec<(Coord, Vec<Coord>)> {
    //     // If we are called when already there, just return no moves.
    //     if self == stop {
    //         vec![]
    //     } else {
    //         // Otherwise try walking in each direction, ending with a turn.
    //         [Dir::East, Dir::South, Dir::West, Dir::North].into_iter()
    //             .filter(|dir| origin.map_or(true, |origin| dir != origin))
    //             .flat_map(|dir|
    //                 (min_len..=max_len).map(|len|
    //                         iter::repeat(dir).take(len).chain(
    //                         if dir == Dir::North || dir == Dir::South {
    //                             [Dir::East, Dir::West]
    //                         } else {
    //                             [Dir::North, Dir::South]
    //                         })))
    //             .try_fold((), |state, mut plan| {
    //                 plan.try_fold((self), |(coord), dir| {
    //                     if let Some(next) = self.walk(dir, size) {
    //                         if seen.contains(&next) {
    //                             Err(())
    //                         } else if next == stop {
    //                             Err(())
    //                         } else {
    //                         }
    //                     } else {
    //                         Err(())
    //                     }
    //                 });
    //                 Ok(state)
    //             })
    //     }
    // }

    // Generate all legal moves when coming from origin, and having a given straight move count.
    fn moves_from(
        self,
        origin: Option<Dir>,
        size: Coord,
        straight: usize,
        min_len: usize,
        max_len: usize,
    ) -> Vec<(Dir, Self, usize)> {
        let x = self.moves(size)
            .into_iter()
            .filter(|(dir, _)| {
                origin.map_or(true, |origin| {
                    *dir != origin.opposite()
                        && if *dir == origin {
                            straight < max_len - 1
                        } else {
                            straight + 1 >= min_len
                                && match *dir {
                                    Dir::East => size.0 - 1 - self.0,
                                    Dir::South => size.1 - 1 - self.1,
                                    Dir::West => self.0,
                                    Dir::North => self.1,
                                } >= min_len
                        }
                })
            })
            .map(|(dir, coord)| {
                (
                    dir,
                    coord,
                    if origin.map_or(true, |origin| dir != origin) {
                        0usize
                    } else {
                        straight + 1
                    },
                )
            })
            .collect_vec();
        //println!("x = {:?}", x);
        x
    }

    fn distance(self, other: Self) -> usize {
        self.0.max(other.0) - self.0.min(other.0) + self.1.max(other.1) - self.1.min(other.1)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    coord: Coord,
    dir: Option<Dir>,
    straight: usize,
    heat_loss: Output,
    track: Vec<Coord>,
}

impl State {
    fn from(
        coord: Coord,
        dir: Option<Dir>,
        straight: usize,
        heat_loss: Output,
        track: Vec<Coord>,
    ) -> State {
        State {
            coord,
            dir,
            straight,
            heat_loss,
            track,
        }
    }
}

#[derive(Debug, Default)]
struct StateStore {
    states: BTreeSet<State>,
}

impl StateStore {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, state: State) -> bool {
        let x = self.states.insert(state);
        if !x {
            //println!("xxx");
        }
        x
    }

    fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    fn states(&self) -> Iter<'_, State> {
        self.states.iter()
    }
}

#[derive(Clone, Debug)]
struct SeenState {
    heat_loss: Output,
    straight: BTreeMap<Dir, usize>,
    track: Vec<Coord>,
}

impl Day17 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(BTreeMap<Coord, Output>, Coord)> {
        io::BufReader::new(input).lines().enumerate().try_fold(
            (BTreeMap::new(), Coord(0, 0)),
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

    fn compute_dfs(
        tiles: &BTreeMap<Coord, Output>,
        size: Coord,
        start: Coord,
        min_len: usize,
        max_len: usize,
    ) -> BoxResult<Output> {
        let finish = Coord(size.0 - 1, size.1 - 1);
        (0..).try_fold((BTreeSet::from([(0, 0, start, None, 0)]), BTreeMap::new()), |(mut queue, mut seen), _| {
            println!("{:?}", queue.iter().take(3).collect_vec());

            let is_valid = |coord| {

            };

            if let Some((_, heat_loss_actual, coord, dir, straight)) = queue.pop_first() {
                if coord == finish && straight + 1 >= min_len {
                    Err(Ok(heat_loss_actual))
                } else {
                    if is_valid(coord) {
                        for (next_dir, next_coord, next_straight) in coord.moves_from(dir, size, straight, min_len, max_len) {
                            let heat_loss = heat_loss_actual + tiles.get(&next_coord).unwrap();
                            queue.insert((heat_loss + next_coord.distance(finish), heat_loss, next_coord, Some(next_dir), next_straight));
                        }
                    }
                    Ok((queue, seen))
                }
            } else {
                Err(Err(AocError.into()))
            }
        }).unwrap_err()
    }

    fn compute(
        tiles: &BTreeMap<Coord, Output>,
        size: Coord,
        start: Coord,
        min_len: usize,
        max_len: usize,
    ) -> BoxResult<Output> {
        let finish = Coord(size.0 - 1, size.1 - 1);
        let mut seen = BTreeMap::<Coord, SeenState>::new();

        let mut is_valid = |state: State| -> bool {
            if seen.get(&finish).map_or(false, |finish_state| {
                state.heat_loss >= finish_state.heat_loss
            }) {
                return false;
            }
            let straight = state
                .coord
                .moves_from(state.dir, size, state.straight, min_len, max_len)
                .iter()
                .map(|&(dir, coord, straight)| (dir, straight))
                .collect::<Vec<(_, _)>>();
            for dup in straight.iter().duplicates_by(|(k, v)| k) {
                //println!("dups {:?}", dup);
            }
            let straight = straight.into_iter().collect::<BTreeMap<_, _>>();
            if let Some(seen_state) = seen.get_mut(&state.coord) {
                if state.heat_loss < seen_state.heat_loss {
                    if state.coord != finish || (state.straight >= min_len - 1 && state.straight < max_len) {
                        seen_state.heat_loss = state.heat_loss;
                        if state.coord == finish {
                            //println!("finish state {:?}", seen_state);
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    let x = [Dir::East, Dir::South, Dir::West, Dir::North]
                        .iter()
                        .map(|dir| {
                            if let Some(straight) = straight.get(dir) {
                                seen_state
                                    .straight
                                    .get_mut(dir)
                                    .map_or(true, |seen_straight| {
                                        if *straight < *seen_straight {
                                            // XXX maybe should peek the far cells if heat_loss is already computed there, and low?
                                            *seen_straight = *straight;
                                            true
                                        } else {
                                            false
                                        }
                                    })
                            } else {
                                false
                            }
                        }).collect_vec();
                    x.into_iter().any(|x| x)
                }
            } else {
                let seen_state = SeenState {
                    heat_loss: state.heat_loss,
                    straight,
                    track: state.track,
                };
                if state.coord != finish || (state.straight >= min_len - 1 && state.straight < max_len) {
                    if state.coord == finish {
                        //println!("finish state {:?}", seen_state);
                    }
                    if let Some(old) = seen.insert(
                        state.coord,
                        seen_state,
                    ) {
                        //println!("argh {:?}", old);
                    };
                    true
                } else {
                    false
                }
            }
        };

        let mut debug = false;
        let mut state_store = StateStore::new();
        state_store.insert(State::from(start, None, 0, 0, vec![]));
        (0..)
            .try_fold(state_store, |state_store, i| {
                println!("loop {}: {:?} ", i, state_store.states.len());
                let state_store =
                    state_store
                        .states()
//                        .inspect(|x| if [Coord(1,0), Coord(2,0), Coord(3,0), Coord(4,0), Coord(5,0), Coord(6,0), Coord(7,0),Coord (7,1),Coord (7,2), Coord(7,3),Coord (7,4), Coord(8,4), Coord(9,4), Coord(10,4),Coord (11,4)].contains(&x.coord) { println!("state {:?}", x); })
                        .fold(StateStore::new(), |mut state_store, state| {
                            let steps = state.coord.moves_from(
                                state.dir,
                                size,
                                state.straight,
                                min_len,
                                max_len,
                            );
                            steps
                                .into_iter()
                                .map(|(dir, coord, straight)| {
                                    let heat_loss = state.heat_loss + tiles.get(&coord).unwrap();
                                    let mut track = state.track.clone();
                                    track.push(state.coord);
                                    State::from(coord, Some(dir), straight, heat_loss, track)
                                })
                                .filter(|state| is_valid(state.clone()))
                                .for_each(|state| {
                                    state_store.insert(state);
                                });
                            state_store
                        });
                if state_store.is_empty() {
                    Err(())
                } else {
                    // if debug {
                    //     println!("{:?}", state_store);
                    // }
                    Ok(state_store)
                }
            })
            .unwrap_err();
        let finish_state = seen.get(&finish);
        println!("finish state at end {:?}", finish_state.unwrap());
        finish_state
            .map(|state| state.heat_loss)
            .ok_or(AocError.into())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute_dfs(&tiles, size, Coord(0, 0), 1, 3)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute_dfs(&tiles, size, Coord(0, 0), 4, 10)
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
