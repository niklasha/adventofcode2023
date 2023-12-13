use crate::day::*;
use std::io::Read;

pub struct Day11 {}

type Output = usize;

impl Day for Day11 {
    fn tag(&self) -> &str {
        "11"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 999999));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

impl Coord {
    fn distance(self, other: Coord) -> Output {
        (i64::abs(self.0 as i64 - other.0 as i64) + i64::abs(self.1 as i64 - other.1 as i64))
            as Output
    }
}
impl Day11 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Vec<Coord>> {
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .try_fold(Vec::new(), |mut space, (y, rs)| {
                let _ = rs.map_err(|_| AocError).map(|s| {
                    s.bytes()
                        .enumerate()
                        .filter(|(_, b)| *b == b'#')
                        .for_each(|(x, _)| {
                            space.push(Coord(x, y));
                        })
                });
                Ok(space)
            })
    }

    fn expand(space: &mut [Coord], n: Output) -> BoxResult<()> {
        Self::expand_axis(space, n, |&Coord(x, _)| x, |Coord(x, _)| x)?;
        Self::expand_axis(space, n, |&Coord(_, y)| y, |Coord(_, y)| y)
    }

    fn expand_axis<F1, F2>(space: &mut [Coord], n: Output, axis: F1, axis_mut: F2) -> BoxResult<()>
    where
        F1: Fn(&Coord) -> Output,
        F2: Fn(&mut Coord) -> &mut Output,
    {
        let min = space.iter().map(&axis).min().ok_or(AocError)?;
        let max = space.iter().map(&axis).max().ok_or(AocError)?;
        let empties = ((min + 1)..max)
            .filter(|&i| !space.iter().any(|c| axis(c) == i))
            .collect::<Vec<_>>();
        for i in empties.into_iter().rev() {
            space
                .iter_mut()
                .filter(|c| axis(c) > i)
                .for_each(|c| *axis_mut(c) += n);
        }
        Ok(())
    }

    fn distance(space: &[Coord]) -> BoxResult<Output> {
        Ok(space
            .iter()
            .combinations(2)
            .map(|p| p[0].distance(*p[1]))
            .sum())
    }

    fn process(input: &mut dyn Read, n: Output) -> BoxResult<Output> {
        let mut space = Self::parse(input)?;
        Self::expand(&mut space, n)?;
        Self::distance(&space)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, 1)
    }

    fn part2_impl(&self, input: &mut dyn io::Read, n: Output) -> BoxResult<Output> {
        Self::process(input, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day11 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            374,
        );
    }

    fn test2(s: &str, n: Output, f: Output) {
        assert_eq!(Day11 {}.part2_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            9,
            1030,
        );
        test2(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            99,
            8410,
        );
    }
}
