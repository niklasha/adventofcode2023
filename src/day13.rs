use crate::day::*;

pub struct Day13 {}

type Output = usize;

impl Day for Day13 {
    fn tag(&self) -> &str {
        "13"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day13 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Vec<Vec<Vec<u8>>>> {
        let binding = io::BufReader::new(input)
            .lines()
            .group_by(|r| r.as_ref().map_or(false, |s| s.is_empty()));
        binding
            .into_iter()
            .filter(|&(is_blank, _)| !is_blank)
            .map(|(_, pattern)| {
                pattern
                    .map(|rs| {
                        rs.map_err(|_| AocError.into())
                            .map(|s| s.bytes().collect_vec())
                    })
                    .collect::<BoxResult<Vec<_>>>()
            })
            .collect::<BoxResult<Vec<_>>>()
    }

    fn find_mirror<F>(pattern: &Vec<Vec<u8>>, get_coord: &F, skip: Option<Output>) -> Output
        where
            F: Fn((Output, Output)) -> (Output, Output),
    {
        let (x_max, y_max) = get_coord((pattern[0].len(), pattern.len()));
        let rv = (1..y_max)
            .position(|y0| {
                skip.map_or(true, |skip| y0 != skip) && (0..y0).all(|y1| {
                    let y2 = y0 + (y0 - y1) - 1;
                    if y2 < y_max {
                        (0..x_max)
                            .map(|x| (get_coord((x, y1)), get_coord((x, y2))))
                            .all(|((x1, y1), (x2, y2))| {
                                pattern[y1][x1] == pattern[y2][x2]
                            })
                    } else {
                        true
                    }
                })
            }).map(|y| y + 1).unwrap_or(0);
        // unsafe {
        //     for y in 0..pattern.len() {
        //         println!("{}", String::from_utf8_unchecked(pattern[y].clone()))
        //     }
        //     println!("{}", rv);
        // }
        rv
    }

    fn find_mirror_2<F>(pattern: &Vec<Vec<u8>>, get_coord: &F, skip: Option<Output>) -> Output
        where
            F: Fn((Output, Output)) -> (Output, Output),
    {
        let (x_max, y_max) = (pattern[0].len(), pattern.len());
        (0..y_max).flat_map(move |y| (0..x_max).map(move |x| {
            let mut pattern = pattern.clone();
            pattern[y][x] = if pattern[y][x] == b'.' { b'#' } else { b'.' };
            Self::find_mirror(&pattern, get_coord, skip)
        }).find(|&n| n > 0)).find(|&n| n > 0).unwrap_or(0)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let patterns = Self::parse(input)?;
        Ok(patterns
            .iter()
            .map(|pattern| {
                100 * Self::find_mirror(pattern, &|c| c, None)
                    + Self::find_mirror(pattern, &|(x, y)| (y, x), None)
            })
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let patterns = Self::parse(input)?;
        Ok(patterns
            .iter()
            .map(|pattern| {
                let a = Self::find_mirror(pattern, &|c| c, None);
                let b = Self::find_mirror(pattern, &|(x, y)| (y, x), None);
                100 * Self::find_mirror_2(pattern, &|c| c, Some(a))
                    + Self::find_mirror_2(pattern, &|(x, y)| (y, x), Some(b))
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day13 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
            405,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day13 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
            400,
        );
    }
}
