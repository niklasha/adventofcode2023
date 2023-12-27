use num::integer::{gcd, lcm};
use regex::Regex;
use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;

use crate::day::*;

pub struct Day24 {}

type Output = usize;

impl Day for Day24 {
    fn tag(&self) -> &str {
        "24"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!(
            "{:?}",
            self.part1_impl(&mut *input(), 200000000000000.0, 400000000000000.0)
        );
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex =
        Regex::new("^(\\d+), (\\d+), (\\d+) @ +(-?\\d+), +(-?\\d+), +(-?\\d+)$").unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Hailstone<T: Clone + Copy + Debug> {
    pos: (T, T, T),
    speed: (T, T, T),
}

impl<T: Clone + Copy + Debug + FromStr> FromStr for Hailstone<T> {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, [px, py, pz, vx, vy, vz]) = PATTERN.captures(&s).ok_or(AocError)?.extract(); // XXX
        Ok(Hailstone {
            pos: (
                px.parse().map_err(|_| AocError)?,
                py.parse().map_err(|_| AocError)?,
                pz.parse().map_err(|_| AocError)?,
            ),
            speed: (
                vx.parse().map_err(|_| AocError)?,
                vy.parse().map_err(|_| AocError)?,
                vz.parse().map_err(|_| AocError)?,
            ),
        })
    }
}

impl Day24 {
    fn parse_hailstone<T>(spec: BoxResult<String>) -> BoxResult<Hailstone<T>>
    where
        T: Clone + Copy + Debug + FromStr,
        <T as FromStr>::Err: error::Error + 'static,
    {
        spec.and_then(|s| s.parse().map_err(|e: AocError| e.into()))
    }

    fn parse<T>(input: &mut dyn io::Read) -> BoxResult<Vec<Hailstone<T>>>
    where
        T: Clone + Copy + Debug + FromStr,
        <T as FromStr>::Err: error::Error + 'static,
    {
        io::BufReader::new(input)
            .lines()
            .map(|rs| Self::parse_hailstone::<T>(rs.map_err(|e| AocError.into())))
            .collect::<BoxResult<Vec<_>>>()
    }

    fn part1_impl(&self, input: &mut dyn io::Read, min: f32, max: f32) -> BoxResult<Output> {
        fn intersection(a: &Range<f32>, b: &Range<f32>) -> Range<f32> {
            a.start.max(b.start)..a.end.max(b.end)
        }

        let hailstones = Self::parse::<f32>(input)?;
        let hailstones = hailstones
            .into_iter()
            .map(|hailstone| {
                let x_time_range = ((min - hailstone.pos.0) / hailstone.speed.0)
                    ..((max - hailstone.pos.0) / hailstone.speed.0);
                let y_time_range = ((min - hailstone.pos.1) / hailstone.speed.1)
                    ..((max - hailstone.pos.1) / hailstone.speed.1);
                let x_time_range = x_time_range.start.min(x_time_range.end)
                    ..x_time_range.start.max(x_time_range.end);
                let y_time_range = y_time_range.start.min(y_time_range.end)
                    ..y_time_range.start.max(y_time_range.end);
                (hailstone, intersection(&x_time_range, &y_time_range))
            })
            //.filter(|(_, time_range)| *time_range.end() >= 0.0 && !time_range.is_empty())
            .collect_vec();
        Ok(hailstones
            .into_iter()
            .tuple_combinations()
            .filter(|((a, _), (b, _))| {
                if a.speed.0 == 0.0 || b.speed.0 == 0.0 {
                    todo!()
                } else {
                    let ka = a.speed.1 / a.speed.0;
                    let ma = a.pos.1 - a.speed.1 / a.speed.0 * a.pos.0;
                    let kb = b.speed.1 / b.speed.0;
                    let mb = b.pos.1 - b.speed.1 / b.speed.0 * b.pos.0;
                    if ka == kb {
                        //                        println!("never intersects: {:?} {:?}", a, b);
                        ma == mb
                    } else {
                        let x = (ma - mb) / (kb - ka);
                        let y = ka * x + ma;
                        let ta = (x - a.pos.0) / a.speed.0;
                        let tb = (x - b.pos.0) / b.speed.0;
                        if !(min..max).contains(&x) || (min..max).contains(&y) {
                            //                            println!("does not intersect in test area: {:?} {:?}", a, b);
                        }
                        if ta < 0.0 {
                            //                            println!("intersects in A's past: {:?} {:?}", a, b);
                        }
                        if tb < 0.0 {
                            //                            println!("intersects in B's past: {:?} {:?}", a, b);
                        }
                        ta >= 0.0 && tb >= 0.0 && (min..max).contains(&x) && (min..max).contains(&y)
                    }
                }
            })
            //.filter(|((a, _), (b, _))| !intersection(a, b).is_empty())
            // .inspect(|x| {
            //     println!("{:?}", x);
            // })
            .count())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let hailstones = Self::parse::<isize>(input)?;
        // solve the equation system
        // x+n*u=251454256616722+43*n
        // y+n*v=382438634889004-207*n
        // z+n*w=18645302082228+371*n
        // x+m*u=289124150762025-73*m
        // y+m*v=364325878532733-158*m
        // z+m*w=278169080781801-13*m
        // x+o*u=268852221227649+41*o
        // y+o*v=10710819924145+192*o
        // z+o*w=258969710792682+62*o
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, min: f32, max: f32, f: Output) {
        assert_eq!(
            Day24 {}.part1_impl(&mut s.as_bytes(), min, max).ok(),
            Some(f)
        );
    }

    #[test]
    fn part1() {
        test1(
            "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
            7.0,
            27.0,
            2,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day24 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
            47,
        );
    }
}
