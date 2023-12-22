use crate::day::*;
use regex::Regex;
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    // Generate all legal moves when coming from origin, and having a given straight move count.
    fn moves_from(
        self,
        origin: Option<Dir>,
        size: Coord,
        straight: usize,
        min_len: usize,
        max_len: usize,
    ) -> Vec<(Dir, Self, usize)> {
        self.moves(size)
            .into_iter()
            .filter(|(dir, _)| {
                origin.map_or(true, |origin| {
                    *dir != origin.opposite()
                        && if *dir == origin {
                            straight < max_len
                        } else {
                            straight + 1 >= min_len
                                && match *dir {
                                    Dir::East => size.0 - self.0,
                                    Dir::South => size.1 - self.1,
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
            .collect_vec()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    states: HashSet<State>,
}

impl StateStore {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, state: State) -> bool {
        self.states.insert(state)
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
    straight: HashMap<Dir, usize>,
    track: Vec<Coord>,
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

    fn compute(
        tiles: &HashMap<Coord, Output>,
        size: Coord,
        start: Coord,
        min_len: usize,
        max_len: usize,
    ) -> BoxResult<Output> {
        let finish = Coord(size.0 - 1, size.1 - 1);
        let mut seen = HashMap::<Coord, SeenState>::new();

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
                .collect::<HashMap<_, _>>();
            if let Some(seen_state) = seen.get_mut(&state.coord) {
                if state.heat_loss < seen_state.heat_loss {
                    seen_state.heat_loss = state.heat_loss;
                    true
                } else {
                    [Dir::East, Dir::South, Dir::West, Dir::North]
                        .iter()
                        .any(|dir| {
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
                        })
                }
            } else {
                seen.insert(
                    state.coord,
                    SeenState {
                        heat_loss: state.heat_loss,
                        straight,
                        track: state.track,
                    },
                );
                true
            }
        };

        let mut state_store = StateStore::new();
        state_store.insert(State::from(start, None, 0, 0, vec![]));
        (0..)
            .try_fold(state_store, |state_store, i| {
                println!("loop {}: {:?} ", i, state_store.states.len());
                let state_store =
                    state_store
                        .states()
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
                    Ok(state_store)
                }
            })
            .unwrap_err();
        let finish_state = seen.get(&finish);
        println!("{:?}", finish_state.unwrap().track);
        finish_state
            .map(|state| state.heat_loss)
            .ok_or(AocError.into())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute(&tiles, size, Coord(0, 0), 1, 3)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (tiles, size) = Self::parse(input)?;
        Self::compute(&tiles, size, Coord(0, 0), 4, 10)
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
