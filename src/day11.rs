use crate::day::*;

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
        let &x_min = space.iter().map(|Coord(x, _)| x).min().ok_or(AocError)?;
        let &x_max = space.iter().map(|Coord(x, _)| x).max().ok_or(AocError)?;
        let &y_min = space.iter().map(|Coord(_, y)| y).min().ok_or(AocError)?;
        let &y_max = space.iter().map(|Coord(_, y)| y).max().ok_or(AocError)?;
        let mut x_empty = ((x_min + 1)..x_max)
            .filter(|&i| !space.iter().any(|Coord(x, _)| *x == i))
            .sorted()
            .collect_vec();
        x_empty.reverse();
        for i in x_empty {
            space
                .iter_mut()
                .filter(|Coord(x, _)| *x > i)
                .for_each(|Coord(x, _)| *x += n);
        }
        let mut y_empty = ((y_min + 1)..y_max)
            .filter(|&i| !space.iter().any(|Coord(_, y)| *y == i))
            .sorted()
            .collect_vec();
        y_empty.reverse();
        for i in y_empty {
            space
                .iter_mut()
                .filter(|Coord(_, y)| *y > i)
                .for_each(|Coord(_, y)| *y += n);
        }
        Ok(())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut space = Self::parse(input)?;
        Self::expand(&mut space, 1)?;
        Ok(space
            .iter()
            .combinations(2)
            .map(|p| p[0].distance(*p[1]))
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read, n: Output) -> BoxResult<Output> {
        let mut space = Self::parse(input)?;
        Self::expand(&mut space, n)?;
        Ok(space
            .iter()
            .combinations(2)
            .map(|p| p[0].distance(*p[1]))
            .sum())
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
