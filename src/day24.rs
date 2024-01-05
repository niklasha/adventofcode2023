use num::integer::{gcd, lcm};
use num_bigint::BigInt;
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
        let (_, [px, py, pz, vx, vy, vz]) = PATTERN.captures(s).ok_or(AocError)?.extract(); // XXX
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
        // x+n*u=A+a*n
        // y+n*v=D+d*n
        // z+n*w=G+g*n
        // x+m*u=B+b*m
        // y+m*v=E+e*m
        // z+m*w=H+h*m
        // x+o*u=C+c*o
        // y+o*v=F+f*o
        // z+o*w=I+i*o
        // and add x + y + z
        let A = &BigInt::from(hailstones[0].pos.0);
        let B = &BigInt::from(hailstones[1].pos.0);
        let C = &BigInt::from(hailstones[2].pos.0);
        let D = &BigInt::from(hailstones[0].pos.1);
        let E = &BigInt::from(hailstones[1].pos.1);
        let F = &BigInt::from(hailstones[2].pos.1);
        let G = &BigInt::from(hailstones[0].pos.2);
        let H = &BigInt::from(hailstones[1].pos.2);
        let I = &BigInt::from(hailstones[2].pos.2);
        let a = &BigInt::from(hailstones[0].speed.0);
        let b = &BigInt::from(hailstones[1].speed.0);
        let c = &BigInt::from(hailstones[2].speed.0);
        let d = &BigInt::from(hailstones[0].speed.1);
        let e = &BigInt::from(hailstones[1].speed.1);
        let f = &BigInt::from(hailstones[2].speed.1);
        let g = &BigInt::from(hailstones[0].speed.2);
        let h = &BigInt::from(hailstones[1].speed.2);
        let i = &BigInt::from(hailstones[2].speed.2);
        let A2 = &A.pow(2);
        let B2 = &B.pow(2);
        let C2 = &C.pow(2);
        let D2 = &D.pow(2);
        let E2 = &E.pow(2);
        let F2 = &F.pow(2);
        let G2 = &G.pow(2);
        let H2 = &H.pow(2);
        let I2 = &I.pow(2);
        let a2 = &a.pow(2);
        let b2 = &b.pow(2);
        let c2 = &c.pow(2);
        let d2 = &d.pow(2);
        let e2 = &e.pow(2);
        let f2 = &f.pow(2);
        let g2 = &g.pow(2);
        let h2 = &h.pow(2);
        let i2 = &i.pow(2);
        let sum: BigInt = ((((B2 - 2 * A * B + A2) * F2
            + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D)
                * F
            + (C2 - 2 * A * C + A2) * E2
            + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D * E
            + (C2 - 2 * B * C + B2) * D2)
            * h
            + ((-B2 + 2 * A * B - A2) * F2
                + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D)
                    * F
                + (-C2 + 2 * A * C - A2) * E2
                + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D * E
                + (-C2 + 2 * B * C - B2) * D2)
                * g
            + ((((B - A) * C - A * B + A2) * F
                + (-C2 + 2 * A * C - A2) * E
                + (C2 + (-B - A) * C + A * B) * D)
                * H
                + (((A - B) * C + B2 - A * B) * F
                    + (C2 + (-B - A) * C + A * B) * E
                    + (-C2 + 2 * B * C - B2) * D)
                    * G
                + (B2 - 2 * A * B + A2) * F2
                + (((A - B) * C + A * B - A2) * E
                    + ((B - A) * C - B2 + A * B) * D
                    + (B2 - 2 * A * B + A2) * C)
                    * F
                + ((A - B) * C2 + (A * B - A2) * C) * E
                + ((B - A) * C2 + (A * B - B2) * C) * D)
                * e
            + ((((A - B) * C + A * B - A2) * F
                + (C2 - 2 * A * C + A2) * E
                + (-C2 + (B + A) * C - A * B) * D)
                * H
                + (((B - A) * C - B2 + A * B) * F
                    + (-C2 + (B + A) * C - A * B) * E
                    + (C2 - 2 * B * C + B2) * D)
                    * G
                + (-B2 + 2 * A * B - A2) * F2
                + (((B - A) * C - A * B + A2) * E
                    + ((A - B) * C + B2 - A * B) * D
                    + (-B2 + 2 * A * B - A2) * C)
                    * F
                + ((B - A) * C2 + (A2 - A * B) * C) * E
                + ((A - B) * C2 + (B2 - A * B) * C) * D)
                * d
            + (((A - B) * F2
                + ((C - A) * E + (-C + 2 * B - A) * D) * F
                + (A - C) * D * E
                + (C - B) * D2)
                * H
                + ((B - A) * F2
                    + ((-C - B + 2 * A) * E + (C - B) * D) * F
                    + (C - A) * E2
                    + (B - C) * D * E)
                    * G
                + ((A - B) * E + (B - A) * D) * F2
                + ((C - A) * E2
                    + ((-(BigInt::from(2) * C) + B + A) * D + (A - B) * C) * E
                    + (C - B) * D2
                    + (B - A) * C * D)
                    * F
                + (C2 - A * C) * E2
                + ((B + A) * C - 2 * C2) * D * E
                + (C2 - B * C) * D2)
                * b
            + (((B - A) * F2
                + ((A - C) * E + (C - 2 * B + A) * D) * F
                + (C - A) * D * E
                + (B - C) * D2)
                * H
                + ((A - B) * F2
                    + ((C + B - 2 * A) * E + (B - C) * D) * F
                    + (A - C) * E2
                    + (C - B) * D * E)
                    * G
                + ((B - A) * E + (A - B) * D) * F2
                + ((A - C) * E2
                    + ((2 * C - B - A) * D + (B - A) * C) * E
                    + (B - C) * D2
                    + (A - B) * C * D)
                    * F
                + (A * C - C2) * E2
                + (2 * C2 + (-B - A) * C) * D * E
                + (B * C - C2) * D2)
                * a)
            * i2
            + (((-B2 + 2 * A * B - A2) * F2
                + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D)
                    * F
                + (-C2 + 2 * A * C - A2) * E2
                + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D * E
                + (-C2 + 2 * B * C - B2) * D2)
                * h2
                + ((((-(BigInt::from(2) * B2) + 4 * A * B - 2 * A2) * F
                    + ((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D)
                    * I
                    + (((B - A) * C - A * B + A2) * F
                        + (-C2 + 2 * A * C - A2) * E
                        + (C2 + (-B - A) * C + A * B) * D)
                        * H
                    + (((A - B) * C + B2 - A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D)
                        * G
                    + (((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D
                        + (-B2 + 2 * A * B - A2) * C)
                        * F
                    + (C2 - 2 * A * C + A2) * E2
                    + ((-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D
                        + (B - A) * C2
                        + (A2 - A * B) * C)
                        * E
                    + (C2 - 2 * B * C + B2) * D2
                    + ((A - B) * C2 + (B2 - A * B) * C) * D)
                    * f
                    + (((B2 - 2 * A * B + A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D)
                        * I
                        + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * F
                            + (2 * C2 - 4 * A * C + 2 * A2) * E
                            + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D)
                            * H
                        + (((B - A) * C - B2 + A * B) * F
                            + (-C2 + (B + A) * C - A * B) * E
                            + (C2 - 2 * B * C + B2) * D)
                            * G
                        + (-B2 + 2 * A * B - A2) * F2
                        + (((B - A) * C - A * B + A2) * E
                            + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D
                            + (A * B - B2) * C
                            + A * B2
                            - A2 * B)
                            * F
                        + ((C2 + (-B - A) * C + A * B) * D + B * C2 - 2 * A * B * C + A2 * B)
                            * E
                        + (-C2 + 2 * B * C - B2) * D2
                        + (-(B * C2) + (B2 + A * B) * C - A * B2) * D)
                        * e
                    + (((B2 - 2 * A * B + A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D)
                        * I
                        + (((B - A) * C - A * B + A2) * F
                            + (-C2 + 2 * A * C - A2) * E
                            + (C2 + (-B - A) * C + A * B) * D)
                            * H
                        + (B2 - 2 * A * B + A2) * F2
                        + (((B - A) * C - B2 + A * B) * D + (2 * B2 - 3 * A * B + A2) * C
                            - A * B2
                            + A2 * B)
                            * F
                        + (-C2 + 2 * A * C - A2) * E2
                        + ((C2 + (-B - A) * C + A * B) * D
                            + (A - 2 * B) * C2
                            + (3 * A * B - A2) * C
                            - A2 * B)
                            * E
                        + ((2 * B - A) * C2 - 2 * B2 * C + A * B2) * D)
                        * d
                    + ((((2 * B - 2 * A) * E + (2 * A - 2 * B) * D) * F
                        + (2 * A - 2 * C) * E2
                        + (4 * C - 2 * B - 2 * A) * D * E
                        + (2 * B - 2 * C) * D2)
                        * I
                        + ((A - B) * F2
                            + ((C - A) * E + (-C + 2 * B - A) * D) * F
                            + (A - C) * D * E
                            + (C - B) * D2)
                            * H
                        + ((B - A) * F2
                            + ((-C - B + 2 * A) * E + (C - B) * D) * F
                            + (C - A) * E2
                            + (B - C) * D * E)
                            * G
                        + ((B - A) * E + (A - B) * D + B2 - 2 * A * B + A2) * F2
                        + ((A - C) * E2
                            + ((2 * C - B - A) * D + (A - B) * C + 2 * A * B - 2 * A2) * E
                            + (B - C) * D2
                            + ((B - A) * C - 2 * B2 + 2 * A * B) * D)
                            * F
                        + (A2 - A * C) * E2
                        + ((B + A) * C - 2 * A * B) * D * E
                        + (B2 - B * C) * D2)
                        * c
                    + ((((A - B) * E + (B - A) * D) * F
                        + (C - A) * E2
                        + (-(BigInt::from(2) * C) + B + A) * D * E
                        + (C - B) * D2)
                        * I
                        + ((2 * B - 2 * A) * F2
                            + ((2 * A - 2 * C) * E + (2 * C - 4 * B + 2 * A) * D) * F
                            + (2 * C - 2 * A) * D * E
                            + (2 * B - 2 * C) * D2)
                            * H
                        + ((A - B) * F2
                            + ((C + B - 2 * A) * E + (B - C) * D) * F
                            + (A - C) * E2
                            + (C - B) * D * E)
                            * G
                        + ((B - A) * E + A * B - A2) * F2
                        + ((A - C) * E2
                            + ((C - 2 * B + A) * D + (B - 2 * A) * C - A * B + 2 * A2) * E
                            + ((2 * A - B) * C - A * B) * D)
                            * F
                        + ((C - A) * D - C2 + 2 * A * C - A2) * E2
                        + ((B - C) * D2 + (2 * C2 + (-B - 2 * A) * C + A * B) * D) * E
                        + (B * C - C2) * D2)
                        * b
                    + ((((A - B) * E + (B - A) * D) * F
                        + (C - A) * E2
                        + (-(BigInt::from(2) * C) + B + A) * D * E
                        + (C - B) * D2)
                        * I
                        + ((A - B) * F2
                            + ((C - A) * E + (-C + 2 * B - A) * D) * F
                            + (A - C) * D * E
                            + (C - B) * D2)
                            * H
                        + ((2 * A - 2 * B) * E + (B - A) * D - B2 + A * B) * F2
                        + ((2 * C - 2 * A) * E2
                            + ((3 * B - 3 * C) * D + A * C - A * B) * E
                            + (C - B) * D2
                            + (-(A * C) + 2 * B2 - A * B) * D)
                            * F
                        + ((A - C) * D + C2 - A * C) * E2
                        + ((C - B) * D2 + (-(BigInt::from(2) * C2) + A * C + A * B) * D) * E
                        + (C2 - B2) * D2)
                        * a)
                    * h
                + ((B2 - 2 * A * B + A2) * F2
                    + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                        + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D)
                        * F
                    + (C2 - 2 * A * C + A2) * E2
                    + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D * E
                    + (C2 - 2 * B * C + B2) * D2)
                    * g.pow(2)
                + ((((2 * B2 - 4 * A * B + 2 * A2) * F
                    + ((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                    + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D)
                    * I
                    + (((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + (-C2 + (B + A) * C - A * B) * D)
                        * H
                    + (((B - A) * C - B2 + A * B) * F
                        + (-C2 + (B + A) * C - A * B) * E
                        + (C2 - 2 * B * C + B2) * D)
                        * G
                    + (((B - A) * C - A * B + A2) * E
                        + ((A - B) * C + B2 - A * B) * D
                        + (B2 - 2 * A * B + A2) * C)
                        * F
                    + (-C2 + 2 * A * C - A2) * E2
                    + ((2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D
                        + (A - B) * C2
                        + (A * B - A2) * C)
                        * E
                    + (-C2 + 2 * B * C - B2) * D2
                    + ((B - A) * C2 + (A * B - B2) * C) * D)
                    * f
                    + (((-B2 + 2 * A * B - A2) * F
                        + ((B - A) * C - A * B + A2) * E
                        + ((A - B) * C + B2 - A * B) * D)
                        * I
                        + (((B - A) * C - B2 + A * B) * F
                            + (-C2 + (B + A) * C - A * B) * E
                            + (C2 - 2 * B * C + B2) * D)
                            * G
                        + (-B2 + 2 * A * B - A2) * F2
                        + (((B - A) * C - A * B + A2) * E + (-B2 + 3 * A * B - 2 * A2) * C
                            - A * B2
                            + A2 * B)
                            * F
                        + ((-C2 + (B + A) * C - A * B) * D + (B - 2 * A) * C2 + 2 * A2 * C
                            - A2 * B)
                            * E
                        + (C2 - 2 * B * C + B2) * D2
                        + ((2 * A - B) * C2 + (B2 - 3 * A * B) * C + A * B2) * D)
                        * e
                    + (((-B2 + 2 * A * B - A2) * F
                        + ((B - A) * C - A * B + A2) * E
                        + ((A - B) * C + B2 - A * B) * D)
                        * I
                        + (((B - A) * C - A * B + A2) * F
                            + (-C2 + 2 * A * C - A2) * E
                            + (C2 + (-B - A) * C + A * B) * D)
                            * H
                        + (((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * F
                            + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * E
                            + (-(BigInt::from(2) * C2) + 4 * B * C - 2 * B2) * D)
                            * G
                        + (B2 - 2 * A * B + A2) * F2
                        + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                            + ((B - A) * C - B2 + A * B) * D
                            + (A2 - A * B) * C
                            + A * B2
                            - A2 * B)
                            * F
                        + (C2 - 2 * A * C + A2) * E2
                        + ((-C2 + (B + A) * C - A * B) * D
                            + A * C2
                            + (-(A * B) - A2) * C
                            + A2 * B)
                            * E
                        + (-(A * C2) + 2 * A * B * C - A * B2) * D)
                        * d
                    + ((((2 * A - 2 * B) * E + (2 * B - 2 * A) * D) * F
                        + (2 * C - 2 * A) * E2
                        + (-(BigInt::from(4) * C) + 2 * B + 2 * A) * D * E
                        + (2 * C - 2 * B) * D2)
                        * I
                        + ((B - A) * F2
                            + ((A - C) * E + (C - 2 * B + A) * D) * F
                            + (C - A) * D * E
                            + (B - C) * D2)
                            * H
                        + ((A - B) * F2
                            + ((C + B - 2 * A) * E + (B - C) * D) * F
                            + (A - C) * E2
                            + (C - B) * D * E)
                            * G
                        + ((A - B) * E + (B - A) * D - B2 + 2 * A * B - A2) * F2
                        + ((C - A) * E2
                            + ((-(BigInt::from(2) * C) + B + A) * D + (B - A) * C - 2 * A * B
                                + 2 * A2)
                                * E
                            + (C - B) * D2
                            + ((A - B) * C + 2 * B2 - 2 * A * B) * D)
                            * F
                        + (A * C - A2) * E2
                        + ((-B - A) * C + 2 * A * B) * D * E
                        + (B * C - B2) * D2)
                        * c
                    + ((((B - A) * E + (A - B) * D) * F
                        + (A - C) * E2
                        + (2 * C - B - A) * D * E
                        + (B - C) * D2)
                        * I
                        + ((A - B) * F2
                            + ((C + B - 2 * A) * E + (B - C) * D) * F
                            + (A - C) * E2
                            + (C - B) * D * E)
                            * G
                        + ((B - A) * E + (2 * A - 2 * B) * D - A * B + A2) * F2
                        + ((A - C) * E2
                            + ((3 * C - 3 * A) * D + B * C + A * B - 2 * A2) * E
                            + (2 * B - 2 * C) * D2
                            + (A * B - B * C) * D)
                            * F
                        + ((A - C) * D - C2 + A2) * E2
                        + ((C - B) * D2 + (2 * C2 - B * C - A * B) * D) * E
                        + (B * C - C2) * D2)
                        * b
                    + ((((B - A) * E + (A - B) * D) * F
                        + (A - C) * E2
                        + (2 * C - B - A) * D * E
                        + (B - C) * D2)
                        * I
                        + ((A - B) * F2
                            + ((C - A) * E + (-C + 2 * B - A) * D) * F
                            + (A - C) * D * E
                            + (C - B) * D2)
                            * H
                        + ((2 * B - 2 * A) * F2
                            + ((-(BigInt::from(2) * C) - 2 * B + 4 * A) * E
                                + (2 * C - 2 * B) * D)
                                * F
                            + (2 * C - 2 * A) * E2
                            + (2 * B - 2 * C) * D * E)
                            * G
                        + ((B - A) * D + B2 - A * B) * F2
                        + (((-C - B + 2 * A) * D + (A - 2 * B) * C + A * B) * E
                            + (C - B) * D2
                            + ((2 * B - A) * C - 2 * B2 + A * B) * D)
                            * F
                        + ((C - A) * D + C2 - A * C) * E2
                        + ((B - C) * D2
                            + (-(BigInt::from(2) * C2) + (2 * B + A) * C - A * B) * D)
                            * E
                        + (C2 - 2 * B * C + B2) * D2)
                        * a)
                    * g
                + (((((A - B) * C + A * B - A2) * H
                    + ((B - A) * C - B2 + A * B) * G
                    + (-(BigInt::from(2) * B2) + 4 * A * B - 2 * A2) * F
                    + ((B - A) * C - A * B + A2) * E
                    + ((A - B) * C + B2 - A * B) * D
                    + (-B2 + 2 * A * B - A2) * C)
                    * I
                    + (C2 - 2 * A * C + A2) * H2
                    + ((-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * G
                        + ((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * F
                        + (-C2 + 2 * A * C - A2) * E
                        + (C2 + (-B - A) * C + A * B) * D
                        + (B - A) * C2
                        + (A2 - A * B) * C)
                        * H
                    + (C2 - 2 * B * C + B2) * G2
                    + (((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D
                        + (A - B) * C2
                        + (B2 - A * B) * C)
                        * G)
                    * e
                    + ((((B - A) * C - A * B + A2) * H
                        + ((A - B) * C + B2 - A * B) * G
                        + (2 * B2 - 4 * A * B + 2 * A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D
                        + (B2 - 2 * A * B + A2) * C)
                        * I
                        + (-C2 + 2 * A * C - A2) * H2
                        + ((2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * G
                            + ((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * F
                            + (C2 - 2 * A * C + A2) * E
                            + (-C2 + (B + A) * C - A * B) * D
                            + (A - B) * C2
                            + (A * B - A2) * C)
                            * H
                        + (-C2 + 2 * B * C - B2) * G2
                        + (((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * F
                            + (-C2 + (B + A) * C - A * B) * E
                            + (C2 - 2 * B * C + B2) * D
                            + (B - A) * C2
                            + (A * B - B2) * C)
                            * G)
                        * d
                    + ((((2 * B - 2 * A) * F + (A - C) * E + (C - 2 * B + A) * D) * H
                        + ((2 * A - 2 * B) * F + (C + B - 2 * A) * E + (B - C) * D) * G
                        + ((2 * B - 2 * A) * E + (2 * A - 2 * B) * D) * F
                        + (A - C) * E2
                        + ((2 * C - B - A) * D + (B - A) * C) * E
                        + (B - C) * D2
                        + (A - B) * C * D)
                        * I
                        + ((A - C) * F + (C - A) * D) * H2
                        + (((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * G
                            + ((A - C) * E + (C + B - 2 * A) * D + (B - A) * C) * F
                            + ((A - C) * D - 2 * C2 + 2 * A * C) * E
                            + (C - B) * D2
                            + (2 * C2 + (-B - A) * C) * D)
                            * H
                        + ((B - C) * F + (C - B) * E) * G2
                        + (((C - 2 * B + A) * E + (B - C) * D + (A - B) * C) * F
                            + (C - A) * E2
                            + ((B - C) * D + 2 * C2 + (-B - A) * C) * E
                            + (2 * B * C - 2 * C2) * D)
                            * G)
                        * b
                    + ((((2 * A - 2 * B) * F + (C - A) * E + (-C + 2 * B - A) * D) * H
                        + ((2 * B - 2 * A) * F + (-C - B + 2 * A) * E + (C - B) * D) * G
                        + ((2 * A - 2 * B) * E + (2 * B - 2 * A) * D) * F
                        + (C - A) * E2
                        + ((-(BigInt::from(2) * C) + B + A) * D + (A - B) * C) * E
                        + (C - B) * D2
                        + (B - A) * C * D)
                        * I
                        + ((C - A) * F + (A - C) * D) * H2
                        + (((-(BigInt::from(2) * C) + B + A) * F + (C - A) * E + (C - B) * D)
                            * G
                            + ((C - A) * E + (-C - B + 2 * A) * D + (A - B) * C) * F
                            + ((C - A) * D + 2 * C2 - 2 * A * C) * E
                            + (B - C) * D2
                            + ((B + A) * C - 2 * C2) * D)
                            * H
                        + ((C - B) * F + (B - C) * E) * G2
                        + (((-C + 2 * B - A) * E + (C - B) * D + (B - A) * C) * F
                            + (A - C) * E2
                            + ((C - B) * D - 2 * C2 + (B + A) * C) * E
                            + (2 * C2 - 2 * B * C) * D)
                            * G)
                        * a)
                    * f
                + ((((B - A) * C - A * B + A2) * H
                    + (B2 - 2 * A * B + A2) * F
                    + ((B - A) * C - B2 + A * B) * D
                    + (B2 - A * B) * C
                    - A * B2
                    + A2 * B)
                    * I
                    + (-C2 + 2 * A * C - A2) * H2
                    + ((C2 + (-B - A) * C + A * B) * G
                        + ((A - B) * C + A * B - A2) * F
                        + (-C2 + (B + A) * C - A * B) * D
                        - B * C2
                        + 2 * A * B * C
                        - A2 * B)
                        * H
                    + (((B - A) * C - B2 + A * B) * F
                        + (C2 - 2 * B * C + B2) * D
                        + B * C2
                        + (-B2 - A * B) * C
                        + A * B2)
                        * G)
                    * e2
                + (((((A - B) * C + A * B - A2) * H
                    + ((A - B) * C + B2 - A * B) * G
                    + ((A - B) * C + A * B - A2) * E
                    + ((A - B) * C + B2 - A * B) * D
                    + (A2 - B2) * C
                    + 2 * A * B2
                    - 2 * A2 * B)
                    * I
                    + (C2 - 2 * A * C + A2) * H2
                    + ((C2 - 2 * A * C + A2) * E
                        + (C2 + (-B - A) * C + A * B) * D
                        + (B + A) * C2
                        + (-(BigInt::from(3) * A * B) - A2) * C
                        + 2 * A2 * B)
                        * H
                    + (-C2 + 2 * B * C - B2) * G2
                    + ((-C2 + (B + A) * C - A * B) * E
                        + (-C2 + 2 * B * C - B2) * D
                        + (-B - A) * C2
                        + (B2 + 3 * A * B) * C
                        - 2 * A * B2)
                        * G)
                    * d
                    + ((((A - B) * F
                        + (2 * C - 2 * A) * E
                        + (-(BigInt::from(2) * C) + B + A) * D)
                        * H
                        + ((B - A) * F
                            + (-(BigInt::from(2) * C) + B + A) * E
                            + (2 * C - 2 * B) * D)
                            * G
                        + ((B - A) * E + (A - B) * D - B2 + 2 * A * B - A2) * F
                        + ((2 * B - 2 * A) * C - A * B + A2) * E
                        + ((2 * A - 2 * B) * C + B2 - A * B) * D)
                        * I
                        + ((A - C) * F + (C - A) * D) * H2
                        + (((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * G
                            + (2 * A - 2 * B) * F2
                            + ((C - A) * E + (-C + 2 * B - A) * D + (A - B) * C - A * B + A2)
                                * F
                            + (A * C - A2) * E
                            + ((B - 2 * A) * C + A * B) * D)
                            * H
                        + ((B - C) * F + (C - B) * E) * G2
                        + ((2 * B - 2 * A) * F2
                            + ((-C - B + 2 * A) * E + (C - B) * D + (B - A) * C + B2 - A * B)
                                * F
                            + ((A - 2 * B) * C + A * B) * E
                            + (B * C - B2) * D)
                            * G)
                        * c
                    + ((((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * H
                        + ((2 * A - 2 * B) * E + (B - A) * D - A * B + A2) * F
                        + ((-C + 2 * B - A) * D + (A - 2 * B) * C + 2 * A * B - A2) * E
                        + (C - B) * D2
                        + ((2 * B - A) * C - A * B) * D)
                        * I
                        + ((2 * C - 2 * A) * F + (2 * A - 2 * C) * D) * H2
                        + (((-(BigInt::from(2) * C) + B + A) * F + (C - A) * E + (C - B) * D)
                            * G
                            + (B - A) * F2
                            + ((C - A) * E + (C - 2 * B + A) * D + (B + A) * C - A * B - A2)
                                * F
                            + ((A - C) * D + C2 - 2 * A * C + A2) * E
                            + (B - C) * D2
                            + (-C2 + (A - B) * C + A * B) * D)
                            * H
                        + ((A - B) * F2
                            + ((-C + 2 * B - A) * E + (B - C) * D + (-B - A) * C + 2 * A * B)
                                * F
                            + ((2 * C - 2 * B) * D - C2 + (2 * B + A) * C - 2 * A * B) * E
                            + (C2 - B * C) * D)
                            * G)
                        * b
                    + ((((2 * B - 2 * A) * F + (A - C) * E + (C - 2 * B + A) * D) * H
                        + ((A - B) * F + (2 * C - B - A) * E + (2 * B - 2 * C) * D) * G
                        + ((B - A) * E + B2 - A * B) * F
                        + ((C - 2 * B + A) * D + A * C - A * B) * E
                        + (B - C) * D2
                        + (-(A * C) - B2 + 2 * A * B) * D)
                        * I
                        + ((A - C) * F + (C - A) * D) * H2
                        + ((B - A) * F2
                            + ((2 * A - 2 * C) * E - 2 * A * C + 2 * A * B) * F
                            + ((C - A) * D - C2 + A * C) * E
                            + (C - B) * D2
                            + (C2 + A * C - 2 * A * B) * D)
                            * H
                        + ((C - B) * F + (B - C) * E) * G2
                        + ((A - B) * F2
                            + ((2 * C - B - A) * E + 2 * A * C - B2 - A * B) * F
                            + ((2 * B - 2 * C) * D + C2 - 2 * A * C + A * B) * E
                            + (B2 - C2) * D)
                            * G)
                        * a)
                    * e
                + ((((B - A) * C - B2 + A * B) * G
                    + (-B2 + 2 * A * B - A2) * F
                    + ((B - A) * C - A * B + A2) * E
                    + (A * B - A2) * C
                    - A * B2
                    + A2 * B)
                    * I
                    + ((-C2 + (B + A) * C - A * B) * G
                        + ((B - A) * C - A * B + A2) * F
                        + (-C2 + 2 * A * C - A2) * E
                        - A * C2
                        + (A * B + A2) * C
                        - A2 * B)
                        * H
                    + (C2 - 2 * B * C + B2) * G2
                    + (((A - B) * C + B2 - A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + A * C2
                        - 2 * A * B * C
                        + A * B2)
                        * G)
                    * d2
                + (((((B - A) * F + (2 * A - 2 * C) * E + (2 * C - B - A) * D) * H
                    + ((A - B) * F + (2 * C - B - A) * E + (2 * B - 2 * C) * D) * G
                    + ((A - B) * E + (B - A) * D + B2 - 2 * A * B + A2) * F
                    + ((2 * A - 2 * B) * C + A * B - A2) * E
                    + ((2 * B - 2 * A) * C - B2 + A * B) * D)
                    * I
                    + ((C - A) * F + (A - C) * D) * H2
                    + (((-(BigInt::from(2) * C) + B + A) * F + (C - A) * E + (C - B) * D) * G
                        + (2 * B - 2 * A) * F2
                        + ((A - C) * E + (C - 2 * B + A) * D + (B - A) * C + A * B - A2) * F
                        + (A2 - A * C) * E
                        + ((2 * A - B) * C - A * B) * D)
                        * H
                    + ((C - B) * F + (B - C) * E) * G2
                    + ((2 * A - 2 * B) * F2
                        + ((C + B - 2 * A) * E + (B - C) * D + (A - B) * C - B2 + A * B) * F
                        + ((2 * B - A) * C - A * B) * E
                        + (B2 - B * C) * D)
                        * G)
                    * c
                    + ((((A - B) * F
                        + (2 * C - 2 * A) * E
                        + (-(BigInt::from(2) * C) + B + A) * D)
                        * H
                        + ((2 * B - 2 * A) * F + (-C - B + 2 * A) * E + (C - B) * D) * G
                        + ((B - A) * D + A * B - A2) * F
                        + (C - A) * E2
                        + ((-C - B + 2 * A) * D + B * C - 2 * A * B + A2) * E
                        + (A * B - B * C) * D)
                        * I
                        + ((A - C) * F + (C - A) * D) * H2
                        + ((A - B) * F2
                            + ((-(BigInt::from(2) * C) + B + A) * D - 2 * B * C + A * B + A2)
                                * F
                            + ((2 * C - 2 * A) * D + C2 - A2) * E
                            + (-C2 + 2 * B * C - A * B) * D)
                            * H
                        + ((C - B) * F + (B - C) * E) * G2
                        + ((B - A) * F2
                            + ((2 * C - 2 * B) * D + 2 * B * C - 2 * A * B) * F
                            + (A - C) * E2
                            + ((B - C) * D - C2 - B * C + 2 * A * B) * E
                            + (C2 - B * C) * D)
                            * G)
                        * b
                    + ((((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * G
                        + ((B - A) * E + (2 * A - 2 * B) * D - B2 + A * B) * F
                        + (A - C) * E2
                        + ((C + B - 2 * A) * D + (B - 2 * A) * C + A * B) * E
                        + ((2 * A - B) * C + B2 - 2 * A * B) * D)
                        * I
                        + (((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * G
                            + (A - B) * F2
                            + ((C - A) * E + (C + B - 2 * A) * D + (B + A) * C - 2 * A * B)
                                * F
                            + ((2 * A - 2 * C) * D - C2 + A * C) * E
                            + (C2 + (-B - 2 * A) * C + 2 * A * B) * D)
                            * H
                        + ((2 * B - 2 * C) * F + (2 * C - 2 * B) * E) * G2
                        + ((B - A) * F2
                            + ((-C - B + 2 * A) * E + (B - C) * D + (-B - A) * C + B2 + A * B)
                                * F
                            + (C - A) * E2
                            + ((C - B) * D + C2 + (A - B) * C - A * B) * E
                            + (-C2 + 2 * B * C - B2) * D)
                            * G)
                        * a)
                    * d
                + (((((D - E) * F + D * E - D2) * H
                    + ((E - D) * F - E2 + D * E) * G
                    + (-E2 + (2 * D + B - A) * E - D2 + (A - B) * D) * F
                    + (A - 2 * C) * E2
                    + (4 * C - B - A) * D * E
                    + (B - 2 * C) * D2)
                    * I
                    + (F2 - 2 * D * F + D2) * H2
                    + ((-(BigInt::from(2) * F2) + (2 * E + 2 * D) * F - 2 * D * E) * G
                        + (E - D - B + A) * F2
                        + ((-D + 2 * C - A) * E
                            + D2
                            + (-(BigInt::from(2) * C) + 2 * B - A) * D)
                            * F
                        + (A - 2 * C) * D * E
                        + (2 * C - B) * D2)
                        * H
                    + (F2 - 2 * E * F + E2) * G2
                    + ((-E + D + B - A) * F2
                        + (E2 + (-D - 2 * C - B + 2 * A) * E + (2 * C - B) * D) * F
                        + (2 * C - A) * E2
                        + (B - 2 * C) * D * E)
                        * G)
                    * b
                    + ((((E - D) * F - D * E + D2) * H
                        + ((D - E) * F + E2 - D * E) * G
                        + (E2 + (-(BigInt::from(2) * D) - B + A) * E + D2 + (B - A) * D) * F
                        + (2 * C - A) * E2
                        + (-(BigInt::from(4) * C) + B + A) * D * E
                        + (2 * C - B) * D2)
                        * I
                        + (-F2 + 2 * D * F - D2) * H2
                        + ((2 * F2 + (-(BigInt::from(2) * E) - 2 * D) * F + 2 * D * E) * G
                            + (-E + D + B - A) * F2
                            + ((D - 2 * C + A) * E - D2 + (2 * C - 2 * B + A) * D) * F
                            + (2 * C - A) * D * E
                            + (B - 2 * C) * D2)
                            * H
                        + (-F2 + 2 * E * F - E2) * G2
                        + ((E - D - B + A) * F2
                            + (-E2 + (D + 2 * C + B - 2 * A) * E + (B - 2 * C) * D) * F
                            + (A - 2 * C) * E2
                            + (2 * C - B) * D * E)
                            * G)
                        * a)
                    * c
                + ((((E - D) * F - D * E + D2) * H
                    + (E2 + (A - D) * E - A * D) * F
                    + (-D + C - A) * E2
                    + (D2 + (A - 2 * C) * D) * E
                    + C * D2)
                    * I
                    + (-F2 + 2 * D * F - D2) * H2
                    + ((F2 + (-E - D) * F + D * E) * G
                        + (-E - A) * F2
                        + ((2 * D - C + A) * E + (C + A) * D) * F
                        + ((C - A) * D - D2) * E
                        - C * D2)
                        * H
                    + ((E + A) * F2
                        + (-E2 + (-D + C - 2 * A) * E - C * D) * F
                        + (D - C + A) * E2
                        + C * D * E)
                        * G)
                    * b2
                + ((((D - E) * F + D * E - D2) * H
                    + ((D - E) * F + E2 - D * E) * G
                    + (-E2 + (-B - A) * E + D2 + (B + A) * D) * F
                    + (2 * D + A) * E2
                    + ((B - A) * D - 2 * D2) * E
                    - B * D2)
                    * I
                    + (F2 - 2 * D * F + D2) * H2
                    + ((E + D + B + A) * F2
                        + ((-(BigInt::from(3) * D) - A) * E - D2
                            + (-(BigInt::from(2) * B) - A) * D)
                            * F
                        + (2 * D2 + A * D) * E
                        + B * D2)
                        * H
                    + (-F2 + 2 * E * F - E2) * G2
                    + ((-E - D - B - A) * F2
                        + (E2 + (3 * D + B + 2 * A) * E + B * D) * F
                        + (-(BigInt::from(2) * D) - A) * E2
                        - B * D * E)
                        * G)
                    * a
                    * b
                + ((((E - D) * F - E2 + D * E) * G
                    + ((D + B) * E - D2 - B * D) * F
                    + (-D - C) * E2
                    + (D2 + (2 * C - B) * D) * E
                    + (B - C) * D2)
                    * I
                    + ((-F2 + (E + D) * F - D * E) * G
                        + (-D - B) * F2
                        + ((D + C) * E + D2 + (2 * B - C) * D) * F
                        + (-D2 - C * D) * E
                        + (C - B) * D2)
                        * H
                    + (F2 - 2 * E * F + E2) * G2
                    + ((D + B) * F2
                        + ((-(BigInt::from(2) * D) - C - B) * E + (C - B) * D) * F
                        + (D + C) * E2
                        + (B - C) * D * E)
                        * G)
                    * a2)
                * i
            + (((B2 - 2 * A * B + A2) * F2
                + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                    + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D)
                    * F
                + (C2 - 2 * A * C + A2) * E2
                + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D * E
                + (C2 - 2 * B * C + B2) * D2)
                * g
                + (((B2 - 2 * A * B + A2) * F
                    + ((A - B) * C + A * B - A2) * E
                    + ((B - A) * C - B2 + A * B) * D)
                    * I
                    + (((B - A) * C - B2 + A * B) * F
                        + (-C2 + (B + A) * C - A * B) * E
                        + (C2 - 2 * B * C + B2) * D)
                        * G
                    + (((B - A) * C - A * B + A2) * E + (B2 - A * B) * C - A * B2 + A2 * B) * F
                    + (-C2 + 2 * A * C - A2) * E2
                    + ((C2 + (-B - A) * C + A * B) * D - B * C2 + 2 * A * B * C - A2 * B) * E
                    + (B * C2 + (-B2 - A * B) * C + A * B2) * D)
                    * f
                + (((-B2 + 2 * A * B - A2) * F
                    + ((B - A) * C - A * B + A2) * E
                    + ((A - B) * C + B2 - A * B) * D)
                    * I
                    + (((A - B) * C + B2 - A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D)
                        * G
                    + (((A - B) * C + A * B - A2) * E + (A * B - B2) * C + A * B2 - A2 * B) * F
                    + (C2 - 2 * A * C + A2) * E2
                    + ((-C2 + (B + A) * C - A * B) * D + B * C2 - 2 * A * B * C + A2 * B) * E
                    + (-(B * C2) + (B2 + A * B) * C - A * B2) * D)
                    * d
                + ((((A - B) * E + (B - A) * D) * F
                    + (C - A) * E2
                    + (-(BigInt::from(2) * C) + B + A) * D * E
                    + (C - B) * D2)
                    * I
                    + ((A - B) * F2
                        + ((C + B - 2 * A) * E + (B - C) * D) * F
                        + (A - C) * E2
                        + (C - B) * D * E)
                        * G
                    + ((A - B) * E - B2 + A * B) * F2
                    + ((C - A) * E2
                        + ((-C + 2 * B - A) * D + B * C - A * B) * E
                        + (-(B * C) + 2 * B2 - A * B) * D)
                        * F
                    + (A - C) * D * E2
                    + ((C - B) * D2 + (A * B - B * C) * D) * E
                    + (B * C - B2) * D2)
                    * c
                + ((((B - A) * E + (A - B) * D) * F
                    + (A - C) * E2
                    + (2 * C - B - A) * D * E
                    + (B - C) * D2)
                    * I
                    + ((B - A) * F2
                        + ((-C - B + 2 * A) * E + (C - B) * D) * F
                        + (C - A) * E2
                        + (B - C) * D * E)
                        * G
                    + ((B - A) * E + B2 - A * B) * F2
                    + ((A - C) * E2
                        + ((C - 2 * B + A) * D - B * C + A * B) * E
                        + (B * C - 2 * B2 + A * B) * D)
                        * F
                    + (C - A) * D * E2
                    + ((B - C) * D2 + (B * C - A * B) * D) * E
                    + (B2 - B * C) * D2)
                    * a)
                * h2
            + (((-B2 + 2 * A * B - A2) * F2
                + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D)
                    * F
                + (-C2 + 2 * A * C - A2) * E2
                + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D * E
                + (-C2 + 2 * B * C - B2) * D2)
                * g.pow(2)
                + (((((A - B) * C + A * B - A2) * F
                    + (C2 - 2 * A * C + A2) * E
                    + (-C2 + (B + A) * C - A * B) * D)
                    * H
                    + (((A - B) * C + B2 - A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D)
                        * G
                    + (((A - B) * C + A * B - A2) * E
                        + ((A - B) * C + B2 - A * B) * D
                        + (A2 - B2) * C
                        + 2 * A * B2
                        - 2 * A2 * B)
                        * F
                    + (C2 - 2 * A * C + A2) * E2
                    + ((B + A) * C2 + (-(BigInt::from(3) * A * B) - A2) * C + 2 * A2 * B) * E
                    + (-C2 + 2 * B * C - B2) * D2
                    + ((-B - A) * C2 + (B2 + 3 * A * B) * C - 2 * A * B2) * D)
                    * f
                    + (((-B2 + 2 * A * B - A2) * F
                        + ((B - A) * C - A * B + A2) * E
                        + ((A - B) * C + B2 - A * B) * D)
                        * I
                        + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * F
                            + (-(BigInt::from(2) * C2) + 4 * A * C - 2 * A2) * E
                            + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D)
                            * H
                        + (((A - B) * C + B2 - A * B) * F
                            + (C2 + (-B - A) * C + A * B) * E
                            + (-C2 + 2 * B * C - B2) * D)
                            * G
                        + (B2 - 2 * A * B + A2) * F2
                        + (((A - B) * C + A * B - A2) * E
                            + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D
                            + (B2 - A * B) * C
                            - A * B2
                            + A2 * B)
                            * F
                        + ((-C2 + (B + A) * C - A * B) * D - B * C2 + 2 * A * B * C - A2 * B)
                            * E
                        + (C2 - 2 * B * C + B2) * D2
                        + (B * C2 + (-B2 - A * B) * C + A * B2) * D)
                        * e
                    + (((B2 - 2 * A * B + A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D)
                        * I
                        + (((A - B) * C + A * B - A2) * F
                            + (C2 - 2 * A * C + A2) * E
                            + (-C2 + (B + A) * C - A * B) * D)
                            * H
                        + (((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * F
                            + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * E
                            + (2 * C2 - 4 * B * C + 2 * B2) * D)
                            * G
                        + (-B2 + 2 * A * B - A2) * F2
                        + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                            + ((A - B) * C + B2 - A * B) * D
                            + (A * B - A2) * C
                            - A * B2
                            + A2 * B)
                            * F
                        + (-C2 + 2 * A * C - A2) * E2
                        + ((C2 + (-B - A) * C + A * B) * D - A * C2 + (A * B + A2) * C
                            - A2 * B)
                            * E
                        + (A * C2 - 2 * A * B * C + A * B2) * D)
                        * d
                    + (((B - A) * F2
                        + ((A - C) * E + (C - 2 * B + A) * D) * F
                        + (C - A) * D * E
                        + (B - C) * D2)
                        * H
                        + ((B - A) * F2
                            + ((-C - B + 2 * A) * E + (C - B) * D) * F
                            + (C - A) * E2
                            + (B - C) * D * E)
                            * G
                        + ((B - A) * E + (B - A) * D + B2 - A2) * F2
                        + ((A - C) * E2
                            + ((3 * A - 3 * B) * D + (-B - A) * C + 2 * A2) * E
                            + (C - B) * D2
                            + ((B + A) * C - 2 * B2) * D)
                            * F
                        + ((2 * C - 2 * A) * D + A * C - A2) * E2
                        + ((2 * B - 2 * C) * D2 + (B - A) * C * D) * E
                        + (B2 - B * C) * D2)
                        * c
                    + ((((B - A) * E + (A - B) * D) * F
                        + (A - C) * E2
                        + (2 * C - B - A) * D * E
                        + (B - C) * D2)
                        * I
                        + ((2 * A - 2 * B) * F2
                            + ((2 * C - 2 * A) * E
                                + (-(BigInt::from(2) * C) + 4 * B - 2 * A) * D)
                                * F
                            + (2 * A - 2 * C) * D * E
                            + (2 * C - 2 * B) * D2)
                            * H
                        + ((B - A) * F2
                            + ((-C - B + 2 * A) * E + (C - B) * D) * F
                            + (C - A) * E2
                            + (B - C) * D * E)
                            * G
                        + ((A - B) * E - A * B + A2) * F2
                        + ((C - A) * E2
                            + ((-C + 2 * B - A) * D + (2 * A - B) * C + A * B - 2 * A2) * E
                            + ((B - 2 * A) * C + A * B) * D)
                            * F
                        + ((A - C) * D + C2 - 2 * A * C + A2) * E2
                        + ((C - B) * D2
                            + (-(BigInt::from(2) * C2) + (B + 2 * A) * C - A * B) * D)
                            * E
                        + (C2 - B * C) * D2)
                        * b
                    + ((((A - B) * E + (B - A) * D) * F
                        + (C - A) * E2
                        + (-(BigInt::from(2) * C) + B + A) * D * E
                        + (C - B) * D2)
                        * I
                        + ((B - A) * F2
                            + ((A - C) * E + (C - 2 * B + A) * D) * F
                            + (C - A) * D * E
                            + (B - C) * D2)
                            * H
                        + ((2 * A - 2 * B) * F2
                            + ((2 * C + 2 * B - 4 * A) * E + (2 * B - 2 * C) * D) * F
                            + (2 * A - 2 * C) * E2
                            + (2 * C - 2 * B) * D * E)
                            * G
                        + ((A - B) * D - B2 + A * B) * F2
                        + (((C + B - 2 * A) * D + (2 * B - A) * C - A * B) * E
                            + (B - C) * D2
                            + ((A - 2 * B) * C + 2 * B2 - A * B) * D)
                            * F
                        + ((A - C) * D - C2 + A * C) * E2
                        + ((C - B) * D2
                            + (2 * C2 + (-(BigInt::from(2) * B) - A) * C + A * B) * D)
                            * E
                        + (-C2 + 2 * B * C - B2) * D2)
                        * a)
                    * g
                + ((B2 - 2 * A * B + A2) * I2
                    + (((A - B) * C + A * B - A2) * H
                        + ((B - A) * C - B2 + A * B) * G
                        + ((B - A) * C - A * B + A2) * E
                        + ((A - B) * C + B2 - A * B) * D
                        + (B2 - 2 * A * B + A2) * C)
                        * I
                    + ((-C2 + 2 * A * C - A2) * E
                        + (C2 + (-B - A) * C + A * B) * D
                        + (A - B) * C2
                        + (A * B - A2) * C)
                        * H
                    + ((C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D
                        + (B - A) * C2
                        + (A * B - B2) * C)
                        * G)
                    * f2
                + (((-B2 + 2 * A * B - A2) * I2
                    + (((B - A) * C - A * B + A2) * H
                        + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * G
                        + (B2 - 2 * A * B + A2) * F
                        + ((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * E
                        + ((B - A) * C - B2 + A * B) * D
                        + (A * B - B2) * C
                        + A * B2
                        - A2 * B)
                        * I
                    + ((C2 + (-B - A) * C + A * B) * G
                        + ((A - B) * C + A * B - A2) * F
                        + (2 * C2 - 4 * A * C + 2 * A2) * E
                        + (-C2 + (B + A) * C - A * B) * D
                        + B * C2
                        - 2 * A * B * C
                        + A2 * B)
                        * H
                    + (-C2 + 2 * B * C - B2) * G2
                    + (((B - A) * C - B2 + A * B) * F
                        + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * E
                        + (C2 - 2 * B * C + B2) * D
                        - B * C2
                        + (B2 + A * B) * C
                        - A * B2)
                        * G)
                    * e
                    + ((-B2 + 2 * A * B - A2) * I2
                        + (((B - A) * C - A * B + A2) * H
                            + (-B2 + 2 * A * B - A2) * F
                            + ((B - A) * C - B2 + A * B) * D
                            + (-B2 + 3 * A * B - 2 * A2) * C
                            - A * B2
                            + A2 * B)
                            * I
                        + ((-C2 + (B + A) * C - A * B) * G
                            + ((B - A) * C - A * B + A2) * F
                            + (-C2 + (B + A) * C - A * B) * D
                            + (B - 2 * A) * C2
                            + 2 * A2 * C
                            - A2 * B)
                            * H
                        + (C2 - 2 * B * C + B2) * G2
                        + (((A - B) * C + B2 - A * B) * F
                            + (C2 - 2 * B * C + B2) * D
                            + (2 * A - B) * C2
                            + (B2 - 3 * A * B) * C
                            + A * B2)
                            * G)
                        * d
                    + (((2 * A - 2 * B) * E + (2 * B - 2 * A) * D) * I2
                        + (((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * H
                            + ((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * G
                            + ((A - B) * E + (B - A) * D - B2 + 2 * A * B - A2) * F
                            + (A - C) * E2
                            + ((2 * C - B - A) * D + (A - B) * C - A * B + A2) * E
                            + (B - C) * D2
                            + ((B - A) * C + B2 - A * B) * D)
                            * I
                        + (((2 * C - 2 * A) * E
                            + (-(BigInt::from(2) * C) + B + A) * D
                            + (2 * B - 2 * A) * C
                            - A * B
                            + A2)
                            * F
                            + ((A - C) * D + A * C - A2) * E
                            + (C - B) * D2
                            + ((A - 2 * B) * C + A * B) * D)
                            * H
                        + (((-(BigInt::from(2) * C) + B + A) * E
                            + (2 * C - 2 * B) * D
                            + (2 * A - 2 * B) * C
                            + B2
                            - A * B)
                            * F
                            + (C - A) * E2
                            + ((B - C) * D + (B - 2 * A) * C + A * B) * E
                            + (B * C - B2) * D)
                            * G)
                        * c
                    + (((B - A) * E + (A - B) * D) * I2
                        + (((2 * A - 2 * B) * F + (C - A) * E + (-C + 2 * B - A) * D) * H
                            + ((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * G
                            + ((A - B) * E - A * B + A2) * F
                            + (2 * C - 2 * A) * E2
                            + ((-(BigInt::from(2) * C) + B + A) * D + (B + A) * C
                                - A * B
                                - A2)
                                * E
                            + ((-B - A) * C + 2 * A * B) * D)
                            * I
                        + (((-C + 2 * B - A) * F + (A - C) * E + (2 * C - 2 * B) * D) * G
                            + ((A - C) * E + (A - 2 * B) * C + 2 * A * B - A2) * F
                            + ((C - A) * D + C2 - 2 * A * C + A2) * E
                            + (-C2 + (2 * B + A) * C - 2 * A * B) * D)
                            * H
                        + ((C - B) * F + (B - C) * E) * G2
                        + (((C + B - 2 * A) * E + (2 * B - A) * C - A * B) * F
                            + (2 * A - 2 * C) * E2
                            + ((C - B) * D - C2 + (A - B) * C + A * B) * E
                            + (C2 - B * C) * D)
                            * G)
                        * b
                    + (((B - A) * E + (A - B) * D) * I2
                        + (((B - A) * F + (2 * A - 2 * C) * E + (2 * C - B - A) * D) * H
                            + ((2 * B - 2 * A) * E + (A - B) * D + B2 - A * B) * F
                            + (A - C) * E2
                            + (2 * A * B - 2 * A * C) * E
                            + (C - B) * D2
                            + (2 * A * C - B2 - A * B) * D)
                            * I
                        + (((C - 2 * B + A) * F + (C - A) * E + (2 * B - 2 * C) * D) * G
                            + ((A - C) * E + (2 * C - B - A) * D + A * C - A * B) * F
                            + (A * C - C2) * E
                            + (B - C) * D2
                            + (C2 - 2 * A * C + A * B) * D)
                            * H
                        + ((B - C) * F + (C - B) * E) * G2
                        + (((C - 2 * B + A) * E + (2 * B - 2 * C) * D - A * C - B2
                            + 2 * A * B)
                            * F
                            + (C - A) * E2
                            + (C2 + A * C - 2 * A * B) * E
                            + (B2 - C2) * D)
                            * G)
                        * a)
                    * f
                + (((B2 - 2 * A * B + A2) * I2
                    + (((A - B) * C + A * B - A2) * H
                        + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * G
                        + (-B2 + 2 * A * B - A2) * F
                        + ((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * E
                        + ((A - B) * C + B2 - A * B) * D
                        + (B2 - A * B) * C
                        - A * B2
                        + A2 * B)
                        * I
                    + ((-C2 + (B + A) * C - A * B) * G
                        + ((B - A) * C - A * B + A2) * F
                        + (-(BigInt::from(2) * C2) + 4 * A * C - 2 * A2) * E
                        + (C2 + (-B - A) * C + A * B) * D
                        - B * C2
                        + 2 * A * B * C
                        - A2 * B)
                        * H
                    + (C2 - 2 * B * C + B2) * G2
                    + (((A - B) * C + B2 - A * B) * F
                        + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * E
                        + (-C2 + 2 * B * C - B2) * D
                        + B * C2
                        + (-B2 - A * B) * C
                        + A * B2)
                        * G)
                    * d
                    + (((B - A) * E + (A - B) * D) * I2
                        + (((B - A) * F + (2 * A - 2 * C) * E + (2 * C - B - A) * D) * H
                            + ((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * G
                            + ((B - A) * E + (B - A) * D + 2 * B2 - 2 * A * B) * F
                            + ((-C - B + 2 * A) * D - B * C + A * B) * E
                            + (C - B) * D2
                            + (B * C - 2 * B2 + A * B) * D)
                            * I
                        + (((-C - B + 2 * A) * F + (2 * C - 2 * A) * E + (B - C) * D) * G
                            + (B - A) * F2
                            + ((2 * A - 2 * C) * E + (C - 2 * B + A) * D - B * C + A * B) * F
                            + (2 * C - 2 * A) * D * E
                            + (B - C) * D2
                            + (B * C - A * B) * D)
                            * H
                        + ((C - B) * F + (B - C) * E) * G2
                        + ((A - B) * F2
                            + ((2 * C - B - A) * E + (B - C) * D + B * C - 2 * B2 + A * B) * F
                            + ((B - C) * D + B * C - A * B) * E
                            + (2 * B2 - 2 * B * C) * D)
                            * G)
                        * c
                    + (((A - B) * E + (B - A) * D) * I2
                        + (((A - B) * F
                            + (2 * C - 2 * A) * E
                            + (-(BigInt::from(2) * C) + B + A) * D)
                            * H
                            + ((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * G
                            + ((A - B) * E + (A - B) * D - 2 * B2 + 2 * A * B) * F
                            + ((C + B - 2 * A) * D + B * C - A * B) * E
                            + (B - C) * D2
                            + (-(B * C) + 2 * B2 - A * B) * D)
                            * I
                        + (((C + B - 2 * A) * F + (2 * A - 2 * C) * E + (C - B) * D) * G
                            + (A - B) * F2
                            + ((2 * C - 2 * A) * E + (-C + 2 * B - A) * D + B * C - A * B) * F
                            + (2 * A - 2 * C) * D * E
                            + (C - B) * D2
                            + (A * B - B * C) * D)
                            * H
                        + ((B - C) * F + (C - B) * E) * G2
                        + ((B - A) * F2
                            + ((-(BigInt::from(2) * C) + B + A) * E + (C - B) * D - B * C
                                + 2 * B2
                                - A * B)
                                * F
                            + ((C - B) * D - B * C + A * B) * E
                            + (2 * B * C - 2 * B2) * D)
                            * G)
                        * a)
                    * e
                + ((((A - B) * C + B2 - A * B) * G
                    + (B2 - 2 * A * B + A2) * F
                    + ((A - B) * C + A * B - A2) * E
                    + (A2 - A * B) * C
                    + A * B2
                    - A2 * B)
                    * I
                    + ((C2 + (-B - A) * C + A * B) * G
                        + ((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + A * C2
                        + (-(A * B) - A2) * C
                        + A2 * B)
                        * H
                    + (-C2 + 2 * B * C - B2) * G2
                    + (((B - A) * C - B2 + A * B) * F + (-C2 + (B + A) * C - A * B) * E
                        - A * C2
                        + 2 * A * B * C
                        - A * B2)
                        * G)
                    * d2
                + ((((B - A) * E + (A - B) * D) * I2
                    + (((2 * A - 2 * B) * F + (C - A) * E + (-C + 2 * B - A) * D) * H
                        + ((2 * A - 2 * B) * D - B2 + A2) * F
                        + (C - A) * E2
                        + ((-C + 2 * B - A) * D + (2 * B - A) * C - A2) * E
                        + ((A - 2 * B) * C + B2) * D)
                        * I
                    + (((C + B - 2 * A) * F + (2 * A - 2 * C) * E + (C - B) * D) * G
                        + (A - B) * F2
                        + ((C + B - 2 * A) * D + (2 * A - B) * C - A2) * F
                        + ((A - C) * D - A * C + A2) * E
                        + (B - A) * C * D)
                        * H
                    + ((B - C) * F + (C - B) * E) * G2
                    + ((B - A) * F2
                        + ((B - C) * D + (B - 2 * A) * C + B2) * F
                        + (A - C) * E2
                        + ((2 * C - 2 * B) * D + (2 * A - 2 * B) * C) * E
                        + (B * C - B2) * D)
                        * G)
                    * c
                    + (((A - B) * E + (B - A) * D) * I2
                        + (((2 * B - 2 * A) * F + (A - C) * E + (C - 2 * B + A) * D) * H
                            + ((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * G
                            + ((B - A) * E + A * B - A2) * F
                            + (2 * A - 2 * C) * E2
                            + ((2 * C - B - A) * D + (-B - A) * C + A * B + A2) * E
                            + ((B + A) * C - 2 * A * B) * D)
                            * I
                        + (((C - 2 * B + A) * F + (C - A) * E + (2 * B - 2 * C) * D) * G
                            + ((C - A) * E + (2 * B - A) * C - 2 * A * B + A2) * F
                            + ((A - C) * D - C2 + 2 * A * C - A2) * E
                            + (C2 + (-(BigInt::from(2) * B) - A) * C + 2 * A * B) * D)
                            * H
                        + ((B - C) * F + (C - B) * E) * G2
                        + (((-C - B + 2 * A) * E + (A - 2 * B) * C + A * B) * F
                            + (2 * C - 2 * A) * E2
                            + ((B - C) * D + C2 + (B - A) * C - A * B) * E
                            + (B * C - C2) * D)
                            * G)
                        * b
                    + ((((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * G
                        + ((A - B) * E + (2 * B - 2 * A) * D + B2 - A * B) * F
                        + (C - A) * E2
                        + ((-C - B + 2 * A) * D + (2 * A - B) * C - A * B) * E
                        + ((B - 2 * A) * C - B2 + 2 * A * B) * D)
                        * I
                        + (((-(BigInt::from(2) * C) + B + A) * F + (C - A) * E + (C - B) * D)
                            * G
                            + (B - A) * F2
                            + ((A - C) * E + (-C - B + 2 * A) * D + (-B - A) * C + 2 * A * B)
                                * F
                            + ((2 * C - 2 * A) * D + C2 - A * C) * E
                            + (-C2 + (B + 2 * A) * C - 2 * A * B) * D)
                            * H
                        + ((2 * C - 2 * B) * F + (2 * B - 2 * C) * E) * G2
                        + ((A - B) * F2
                            + ((C + B - 2 * A) * E + (C - B) * D + (B + A) * C - B2 - A * B)
                                * F
                            + (A - C) * E2
                            + ((B - C) * D - C2 + (B - A) * C + A * B) * E
                            + (C2 - 2 * B * C + B2) * D)
                            * G)
                        * a)
                    * d
                + ((E2 - 2 * D * E + D2) * I2
                    + (((D - E) * F + D * E - D2) * H
                        + ((E - D) * F - E2 + D * E) * G
                        + (E2 + (-(BigInt::from(2) * D) + B - A) * E + D2 + (A - B) * D) * F
                        + A * E2
                        + (-B - A) * D * E
                        + B * D2)
                        * I
                    + ((-E + D - B + A) * F2
                        + ((D - A) * E - D2 + (2 * B - A) * D) * F
                        + A * D * E
                        - B * D2)
                        * H
                    + ((E - D + B - A) * F2 + (-E2 + (D - B + 2 * A) * E - B * D) * F - A * E2
                        + B * D * E)
                        * G)
                    * c2
                + (((-E2 + 2 * D * E - D2) * I2
                    + (((E - D) * F - D * E + D2) * H
                        + ((2 * D - 2 * E) * F + 2 * E2 - 2 * D * E) * G
                        + (-E2 + (D - 2 * B + A) * E + (2 * B - A) * D) * F
                        + (D + C - A) * E2
                        + ((-(BigInt::from(2) * C) + 2 * B + A) * D - D2) * E
                        + (C - 2 * B) * D2)
                        * I
                    + ((F2 + (-E - D) * F + D * E) * G
                        + (E + 2 * B - A) * F2
                        + ((-(BigInt::from(2) * D) - C + A) * E + (C - 4 * B + A) * D) * F
                        + (D2 + (C - A) * D) * E
                        + (2 * B - C) * D2)
                        * H
                    + (-F2 + 2 * E * F - E2) * G2
                    + ((-E - 2 * B + A) * F2
                        + (E2 + (D + C + 2 * B - 2 * A) * E + (2 * B - C) * D) * F
                        + (-D - C + A) * E2
                        + (C - 2 * B) * D * E)
                        * G)
                    * b
                    + ((-E2 + 2 * D * E - D2) * I2
                        + (((E - D) * F - D * E + D2) * H
                            + (-E2 + (3 * D + A) * E - 2 * D2 - A * D) * F
                            + (-D - C - A) * E2
                            + (D2 + (2 * C + A) * D) * E
                            - C * D2)
                            * I
                        + ((-F2 + (E + D) * F - D * E) * G
                            + (E - 2 * D - A) * F2
                            + ((C + A) * E + 2 * D2 + (A - C) * D) * F
                            + ((-C - A) * D - D2) * E
                            + C * D2)
                            * H
                        + (F2 - 2 * E * F + E2) * G2
                        + ((-E + 2 * D + A) * F2
                            + (E2 + (-(BigInt::from(3) * D) - C - 2 * A) * E + C * D) * F
                            + (D + C + A) * E2
                            - C * D * E)
                            * G)
                        * a)
                    * c
                + ((E2 - 2 * D * E + D2) * I2
                    + (((D - E) * F + D * E - D2) * H
                        + ((2 * E - 2 * D) * F - 2 * E2 + 2 * D * E) * G
                        + (E2 + (-D + 2 * B - A) * E + (A - 2 * B) * D) * F
                        + (-D - C + A) * E2
                        + (D2 + (2 * C - 2 * B - A) * D) * E
                        + (2 * B - C) * D2)
                        * I
                    + ((-F2 + (E + D) * F - D * E) * G
                        + (-E - 2 * B + A) * F2
                        + ((2 * D + C - A) * E + (-C + 4 * B - A) * D) * F
                        + ((A - C) * D - D2) * E
                        + (C - 2 * B) * D2)
                        * H
                    + (F2 - 2 * E * F + E2) * G2
                    + ((E + 2 * B - A) * F2
                        + (-E2 + (-D - C - 2 * B + 2 * A) * E + (C - 2 * B) * D) * F
                        + (D + C - A) * E2
                        + (2 * B - C) * D * E)
                        * G)
                    * a
                    * b
                + ((((D - E) * F + E2 - D * E) * G
                    + ((-D - B) * E + D2 + B * D) * F
                    + (D + C) * E2
                    + ((B - 2 * C) * D - D2) * E
                    + (C - B) * D2)
                    * I
                    + ((F2 + (-E - D) * F + D * E) * G
                        + (D + B) * F2
                        + ((-D - C) * E - D2 + (C - 2 * B) * D) * F
                        + (D2 + C * D) * E
                        + (B - C) * D2)
                        * H
                    + (-F2 + 2 * E * F - E2) * G2
                    + ((-D - B) * F2
                        + ((2 * D + C + B) * E + (B - C) * D) * F
                        + (-D - C) * E2
                        + (C - B) * D * E)
                        * G)
                    * a2)
                * h
            + ((((-B2 + 2 * A * B - A2) * F
                + ((B - A) * C - A * B + A2) * E
                + ((A - B) * C + B2 - A * B) * D)
                * I
                + (((B - A) * C - A * B + A2) * F
                    + (-C2 + 2 * A * C - A2) * E
                    + (C2 + (-B - A) * C + A * B) * D)
                    * H
                + (((B - A) * C - B2 + A * B) * D + (A * B - A2) * C - A * B2 + A2 * B) * F
                + ((-C2 + (B + A) * C - A * B) * D - A * C2 + (A * B + A2) * C - A2 * B) * E
                + (C2 - 2 * B * C + B2) * D2
                + (A * C2 - 2 * A * B * C + A * B2) * D)
                * f
                + (((B2 - 2 * A * B + A2) * F
                    + ((A - B) * C + A * B - A2) * E
                    + ((B - A) * C - B2 + A * B) * D)
                    * I
                    + (((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + (-C2 + (B + A) * C - A * B) * D)
                        * H
                    + (((A - B) * C + B2 - A * B) * D + (A2 - A * B) * C + A * B2 - A2 * B) * F
                    + ((C2 + (-B - A) * C + A * B) * D + A * C2 + (-(A * B) - A2) * C + A2 * B)
                        * E
                    + (-C2 + 2 * B * C - B2) * D2
                    + (-(A * C2) + 2 * A * B * C - A * B2) * D)
                    * e
                + ((((B - A) * E + (A - B) * D) * F
                    + (A - C) * E2
                    + (2 * C - B - A) * D * E
                    + (B - C) * D2)
                    * I
                    + ((A - B) * F2
                        + ((C - A) * E + (-C + 2 * B - A) * D) * F
                        + (A - C) * D * E
                        + (C - B) * D2)
                        * H
                    + ((A - B) * D - A * B + A2) * F2
                    + (((C + B - 2 * A) * D + A * C + A * B - 2 * A2) * E
                        + (B - C) * D2
                        + (A * B - A * C) * D)
                        * F
                    + ((A - C) * D - A * C + A2) * E2
                    + ((C - B) * D2 + (A * C - A * B) * D) * E)
                    * c
                + ((((A - B) * E + (B - A) * D) * F
                    + (C - A) * E2
                    + (-(BigInt::from(2) * C) + B + A) * D * E
                    + (C - B) * D2)
                    * I
                    + ((B - A) * F2
                        + ((A - C) * E + (C - 2 * B + A) * D) * F
                        + (C - A) * D * E
                        + (B - C) * D2)
                        * H
                    + ((B - A) * D + A * B - A2) * F2
                    + (((-C - B + 2 * A) * D - A * C - A * B + 2 * A2) * E
                        + (C - B) * D2
                        + (A * C - A * B) * D)
                        * F
                    + ((C - A) * D + A * C - A2) * E2
                    + ((B - C) * D2 + (A * B - A * C) * D) * E)
                    * b)
                * g.pow(2)
            + (((-B2 + 2 * A * B - A2) * I2
                + (((B - A) * C - A * B + A2) * H
                    + ((A - B) * C + B2 - A * B) * G
                    + ((A - B) * C + A * B - A2) * E
                    + ((B - A) * C - B2 + A * B) * D
                    + (-B2 + 2 * A * B - A2) * C)
                    * I
                + ((C2 - 2 * A * C + A2) * E
                    + (-C2 + (B + A) * C - A * B) * D
                    + (B - A) * C2
                    + (A2 - A * B) * C)
                    * H
                + ((-C2 + (B + A) * C - A * B) * E
                    + (C2 - 2 * B * C + B2) * D
                    + (A - B) * C2
                    + (B2 - A * B) * C)
                    * G)
                * f2
                + (((B2 - 2 * A * B + A2) * I2
                    + (((B - A) * C - B2 + A * B) * G
                        + (B2 - 2 * A * B + A2) * F
                        + ((B - A) * C - A * B + A2) * E
                        + (2 * B2 - 3 * A * B + A2) * C
                        - A * B2
                        + A2 * B)
                        * I
                    + (-C2 + 2 * A * C - A2) * H2
                    + ((C2 + (-B - A) * C + A * B) * G
                        + ((A - B) * C + A * B - A2) * F
                        + (-C2 + 2 * A * C - A2) * E
                        + (A - 2 * B) * C2
                        + (3 * A * B - A2) * C
                        - A2 * B)
                        * H
                    + (((B - A) * C - B2 + A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (2 * B - A) * C2
                        - 2 * B2 * C
                        + A * B2)
                        * G)
                    * e
                    + ((B2 - 2 * A * B + A2) * I2
                        + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * H
                            + ((B - A) * C - B2 + A * B) * G
                            + (-B2 + 2 * A * B - A2) * F
                            + ((B - A) * C - A * B + A2) * E
                            + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * D
                            + (A2 - A * B) * C
                            + A * B2
                            - A2 * B)
                            * I
                        + (C2 - 2 * A * C + A2) * H2
                        + ((-C2 + (B + A) * C - A * B) * G
                            + ((B - A) * C - A * B + A2) * F
                            + (-C2 + 2 * A * C - A2) * E
                            + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * D
                            + A * C2
                            + (-(A * B) - A2) * C
                            + A2 * B)
                            * H
                        + (((A - B) * C + B2 - A * B) * F
                            + (C2 + (-B - A) * C + A * B) * E
                            + (-(BigInt::from(2) * C2) + 4 * B * C - 2 * B2) * D
                            - A * C2
                            + 2 * A * B * C
                            - A * B2)
                            * G)
                        * d
                    + (((2 * B - 2 * A) * E + (2 * A - 2 * B) * D) * I2
                        + (((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * H
                            + ((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * G
                            + ((B - A) * E + (A - B) * D + B2 - 2 * A * B + A2) * F
                            + (C - A) * E2
                            + ((-(BigInt::from(2) * C) + B + A) * D + (B - A) * C + A * B
                                - A2)
                                * E
                            + (C - B) * D2
                            + ((A - B) * C - B2 + A * B) * D)
                            * I
                        + (((2 * A - 2 * C) * E
                            + (2 * C - B - A) * D
                            + (2 * A - 2 * B) * C
                            + A * B
                            - A2)
                            * F
                            + ((C - A) * D - A * C + A2) * E
                            + (B - C) * D2
                            + ((2 * B - A) * C - A * B) * D)
                            * H
                        + (((2 * C - B - A) * E + (2 * B - 2 * C) * D + (2 * B - 2 * A) * C
                            - B2
                            + A * B)
                            * F
                            + (A - C) * E2
                            + ((C - B) * D + (2 * A - B) * C - A * B) * E
                            + (B2 - B * C) * D)
                            * G)
                        * c
                    + (((A - B) * E + (B - A) * D) * I2
                        + (((B - A) * F
                            + (-(BigInt::from(2) * C) + B + A) * E
                            + (2 * C - 2 * B) * D)
                            * G
                            + ((A - B) * E + (2 * B - 2 * A) * D + A * B - A2) * F
                            + (A - C) * E2
                            + (-(BigInt::from(2) * B * C) + A * B + A2) * E
                            + (C - B) * D2
                            + (2 * B * C - 2 * A * B) * D)
                            * I
                        + ((C - A) * F + (A - C) * D) * H2
                        + (((-C - B + 2 * A) * F + (2 * C - 2 * A) * E + (B - C) * D) * G
                            + ((2 * C - 2 * A) * E + (-C - B + 2 * A) * D + B * C - 2 * A * B
                                + A2)
                                * F
                            + (C2 - A2) * E
                            + (B - C) * D2
                            + (-C2 - B * C + 2 * A * B) * D)
                            * H
                        + (((-(BigInt::from(2) * C) + B + A) * E + (C - B) * D - B * C
                            + A * B)
                            * F
                            + (C - A) * E2
                            + (-C2 + 2 * B * C - A * B) * E
                            + (C2 - B * C) * D)
                            * G)
                        * b
                    + (((A - B) * E + (B - A) * D) * I2
                        + (((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * H
                            + ((2 * A - 2 * B) * F + (C + B - 2 * A) * E + (B - C) * D) * G
                            + ((A - B) * D - B2 + A * B) * F
                            + ((2 * C - B - A) * D + (B + A) * C - 2 * A * B) * E
                            + (2 * B - 2 * C) * D2
                            + ((-B - A) * C + B2 + A * B) * D)
                            * I
                        + ((A - C) * F + (C - A) * D) * H2
                        + (((C + B - 2 * A) * F + (2 * A - 2 * C) * E + (C - B) * D) * G
                            + ((-C + 2 * B - A) * D + (B - 2 * A) * C + A * B) * F
                            + ((A - C) * D - C2 + A * C) * E
                            + (2 * C - 2 * B) * D2
                            + (C2 + (A - B) * C - A * B) * D)
                            * H
                        + (((C - B) * D + (2 * A - B) * C + B2 - 2 * A * B) * F
                            + ((B - C) * D + C2 + (-B - 2 * A) * C + 2 * A * B) * E
                            + (-C2 + 2 * B * C - B2) * D)
                            * G)
                        * a)
                    * f
                + ((((A - B) * C + A * B - A2) * H
                    + (-B2 + 2 * A * B - A2) * F
                    + ((A - B) * C + B2 - A * B) * D
                    + (A * B - B2) * C
                    + A * B2
                    - A2 * B)
                    * I
                    + (C2 - 2 * A * C + A2) * H2
                    + ((-C2 + (B + A) * C - A * B) * G
                        + ((B - A) * C - A * B + A2) * F
                        + (C2 + (-B - A) * C + A * B) * D
                        + B * C2
                        - 2 * A * B * C
                        + A2 * B)
                        * H
                    + (((A - B) * C + B2 - A * B) * F + (-C2 + 2 * B * C - B2) * D - B * C2
                        + (B2 + A * B) * C
                        - A * B2)
                        * G)
                    * e2
                + (((-B2 + 2 * A * B - A2) * I2
                    + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * H
                        + ((A - B) * C + B2 - A * B) * G
                        + (B2 - 2 * A * B + A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * D
                        + (A * B - A2) * C
                        - A * B2
                        + A2 * B)
                        * I
                    + (-C2 + 2 * A * C - A2) * H2
                    + ((C2 + (-B - A) * C + A * B) * G
                        + ((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * D
                        - A * C2
                        + (A * B + A2) * C
                        - A2 * B)
                        * H
                    + (((B - A) * C - B2 + A * B) * F
                        + (-C2 + (B + A) * C - A * B) * E
                        + (2 * C2 - 4 * B * C + 2 * B2) * D
                        + A * C2
                        - 2 * A * B * C
                        + A * B2)
                        * G)
                    * d
                    + (((A - B) * E + (B - A) * D) * I2
                        + (((2 * A - 2 * B) * F + (C + B - 2 * A) * E + (B - C) * D) * G
                            + ((2 * A - 2 * B) * E - B2 + A2) * F
                            + ((C + B - 2 * A) * D + (2 * A - B) * C - A2) * E
                            + (B - C) * D2
                            + ((B - 2 * A) * C + B2) * D)
                            * I
                        + ((C - A) * F + (A - C) * D) * H2
                        + (((-C + 2 * B - A) * F + (A - C) * E + (2 * C - 2 * B) * D) * G
                            + (B - A) * F2
                            + ((C - A) * E + (2 * B - A) * C - A2) * F
                            + ((2 * A - 2 * C) * D - A * C + A2) * E
                            + (C - B) * D2
                            + (2 * A - 2 * B) * C * D)
                            * H
                        + ((A - B) * F2
                            + ((-C + 2 * B - A) * E + (A - 2 * B) * C + B2) * F
                            + ((C - B) * D + (B - A) * C) * E
                            + (B * C - B2) * D)
                            * G)
                        * c
                    + ((((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * H
                        + ((2 * B - 2 * A) * E + (A - B) * D + A * B - A2) * F
                        + ((C - 2 * B + A) * D + (2 * B - A) * C - 2 * A * B + A2) * E
                        + (B - C) * D2
                        + ((A - 2 * B) * C + A * B) * D)
                        * I
                        + ((2 * A - 2 * C) * F + (2 * C - 2 * A) * D) * H2
                        + (((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * G
                            + (A - B) * F2
                            + ((A - C) * E + (-C + 2 * B - A) * D + (-B - A) * C + A * B + A2)
                                * F
                            + ((C - A) * D - C2 + 2 * A * C - A2) * E
                            + (C - B) * D2
                            + (C2 + (B - A) * C - A * B) * D)
                            * H
                        + ((B - A) * F2
                            + ((C - 2 * B + A) * E + (C - B) * D + (B + A) * C - 2 * A * B)
                                * F
                            + ((2 * B - 2 * C) * D
                                + C2
                                + (-(BigInt::from(2) * B) - A) * C
                                + 2 * A * B)
                                * E
                            + (B * C - C2) * D)
                            * G)
                        * b
                    + (((B - A) * E + (A - B) * D) * I2
                        + (((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * H
                            + ((2 * B - 2 * A) * F + (-C - B + 2 * A) * E + (C - B) * D) * G
                            + ((B - A) * D + B2 - A * B) * F
                            + ((-(BigInt::from(2) * C) + B + A) * D
                                + (-B - A) * C
                                + 2 * A * B)
                                * E
                            + (2 * C - 2 * B) * D2
                            + ((B + A) * C - B2 - A * B) * D)
                            * I
                        + ((C - A) * F + (A - C) * D) * H2
                        + (((-C - B + 2 * A) * F + (2 * C - 2 * A) * E + (B - C) * D) * G
                            + ((C - 2 * B + A) * D + (2 * A - B) * C - A * B) * F
                            + ((C - A) * D + C2 - A * C) * E
                            + (2 * B - 2 * C) * D2
                            + (-C2 + (B - A) * C + A * B) * D)
                            * H
                        + (((B - C) * D + (B - 2 * A) * C - B2 + 2 * A * B) * F
                            + ((C - B) * D - C2 + (B + 2 * A) * C - 2 * A * B) * E
                            + (C2 - 2 * B * C + B2) * D)
                            * G)
                        * a)
                    * e
                + ((((A - B) * E + (B - A) * D) * I2
                    + (((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * H
                        + ((B - A) * F
                            + (-(BigInt::from(2) * C) + B + A) * E
                            + (2 * C - 2 * B) * D)
                            * G
                        + ((B - A) * E + (B - A) * D + 2 * A * B - 2 * A2) * F
                        + (A - C) * E2
                        + ((C - 2 * B + A) * D - A * C - A * B + 2 * A2) * E
                        + (A * C - A * B) * D)
                        * I
                    + ((A - C) * F + (C - A) * D) * H2
                    + (((C - 2 * B + A) * F + (C - A) * E + (2 * B - 2 * C) * D) * G
                        + (A - B) * F2
                        + ((C - A) * E + (-(BigInt::from(2) * C) + B + A) * D - A * C - A * B
                            + 2 * A2)
                            * F
                        + ((C - A) * D + 2 * A * C - 2 * A2) * E
                        + (A * B - A * C) * D)
                        * H
                    + ((B - A) * F2
                        + ((-C - B + 2 * A) * E + (2 * C - 2 * B) * D + A * C - A * B) * F
                        + (C - A) * E2
                        + ((2 * B - 2 * C) * D - A * C + A * B) * E)
                        * G)
                    * c
                    + (((B - A) * E + (A - B) * D) * I2
                        + (((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * H
                            + ((A - B) * F + (2 * C - B - A) * E + (2 * B - 2 * C) * D) * G
                            + ((A - B) * E + (A - B) * D - 2 * A * B + 2 * A2) * F
                            + (C - A) * E2
                            + ((-C + 2 * B - A) * D + A * C + A * B - 2 * A2) * E
                            + (A * B - A * C) * D)
                            * I
                        + ((C - A) * F + (A - C) * D) * H2
                        + (((-C + 2 * B - A) * F + (A - C) * E + (2 * C - 2 * B) * D) * G
                            + (B - A) * F2
                            + ((A - C) * E + (2 * C - B - A) * D + A * C + A * B - 2 * A2) * F
                            + ((A - C) * D - 2 * A * C + 2 * A2) * E
                            + (A * C - A * B) * D)
                            * H
                        + ((A - B) * F2
                            + ((C + B - 2 * A) * E + (2 * B - 2 * C) * D - A * C + A * B) * F
                            + (A - C) * E2
                            + ((2 * C - 2 * B) * D + A * C - A * B) * E)
                            * G)
                        * b)
                    * d
                + ((-E2 + 2 * D * E - D2) * I2
                    + (((E - D) * F - D * E + D2) * H
                        + ((D - E) * F + E2 - D * E) * G
                        + (-E2 + (2 * D - B + A) * E - D2 + (B - A) * D) * F
                        - A * E2
                        + (B + A) * D * E
                        - B * D2)
                        * I
                    + ((E - D + B - A) * F2 + ((A - D) * E + D2 + (A - 2 * B) * D) * F
                        - A * D * E
                        + B * D2)
                        * H
                    + ((-E + D - B + A) * F2 + (E2 + (-D + B - 2 * A) * E + B * D) * F + A * E2
                        - B * D * E)
                        * G)
                    * c2
                + (((E2 - 2 * D * E + D2) * I2
                    + (((E - D) * F - E2 + D * E) * G
                        + (2 * E2 + (B - 3 * D) * E + D2 - B * D) * F
                        + (C - D) * E2
                        + (D2 + (-(BigInt::from(2) * C) - B) * D) * E
                        + (C + B) * D2)
                        * I
                    + (-F2 + 2 * D * F - D2) * H2
                    + ((F2 + (-E - D) * F + D * E) * G
                        + (-(BigInt::from(2) * E) + D - B) * F2
                        + ((3 * D - C) * E - D2 + (C + 2 * B) * D) * F
                        + (C * D - D2) * E
                        + (-C - B) * D2)
                        * H
                    + ((2 * E - D + B) * F2
                        + (-(BigInt::from(2) * E2) + (C - B) * E + (-C - B) * D) * F
                        + (D - C) * E2
                        + (C + B) * D * E)
                        * G)
                    * b
                    + ((E2 - 2 * D * E + D2) * I2
                        + (((2 * D - 2 * E) * F + 2 * D * E - 2 * D2) * H
                            + ((E - D) * F - E2 + D * E) * G
                            + ((-D + B - 2 * A) * E + D2 + (2 * A - B) * D) * F
                            + (D - C + 2 * A) * E2
                            + ((2 * C - B - 2 * A) * D - D2) * E
                            + (B - C) * D2)
                            * I
                        + (F2 - 2 * D * F + D2) * H2
                        + ((-F2 + (E + D) * F - D * E) * G
                            + (D - B + 2 * A) * F2
                            + ((-D + C - 2 * A) * E - D2 + (-C + 2 * B - 2 * A) * D) * F
                            + (D2 + (2 * A - C) * D) * E
                            + (C - B) * D2)
                            * H
                        + ((-D + B - 2 * A) * F2
                            + ((2 * D - C - B + 4 * A) * E + (C - B) * D) * F
                            + (-D + C - 2 * A) * E2
                            + (B - C) * D * E)
                            * G)
                        * a)
                    * c
                + ((((D - E) * F + D * E - D2) * H
                    + (-E2 + (D - A) * E + A * D) * F
                    + (D - C + A) * E2
                    + ((2 * C - A) * D - D2) * E
                    - C * D2)
                    * I
                    + (F2 - 2 * D * F + D2) * H2
                    + ((-F2 + (E + D) * F - D * E) * G
                        + (E + A) * F2
                        + ((-(BigInt::from(2) * D) + C - A) * E + (-C - A) * D) * F
                        + (D2 + (A - C) * D) * E
                        + C * D2)
                        * H
                    + ((-E - A) * F2
                        + (E2 + (D - C + 2 * A) * E + C * D) * F
                        + (-D + C - A) * E2
                        - C * D * E)
                        * G)
                    * b2
                + ((-E2 + 2 * D * E - D2) * I2
                    + (((2 * E - 2 * D) * F - 2 * D * E + 2 * D2) * H
                        + ((D - E) * F + E2 - D * E) * G
                        + ((D - B + 2 * A) * E - D2 + (B - 2 * A) * D) * F
                        + (-D + C - 2 * A) * E2
                        + (D2 + (-(BigInt::from(2) * C) + B + 2 * A) * D) * E
                        + (C - B) * D2)
                        * I
                    + (-F2 + 2 * D * F - D2) * H2
                    + ((F2 + (-E - D) * F + D * E) * G
                        + (-D + B - 2 * A) * F2
                        + ((D - C + 2 * A) * E + D2 + (C - 2 * B + 2 * A) * D) * F
                        + ((C - 2 * A) * D - D2) * E
                        + (B - C) * D2)
                        * H
                    + ((D - B + 2 * A) * F2
                        + ((-(BigInt::from(2) * D) + C + B - 4 * A) * E + (B - C) * D) * F
                        + (D - C + 2 * A) * E2
                        + (C - B) * D * E)
                        * G)
                    * a
                    * b)
                * g
            + (((B2 - 2 * A * B + A2) * I2
                + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * H
                    + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * G)
                    * I
                + (C2 - 2 * A * C + A2) * H2
                + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * G * H
                + (C2 - 2 * B * C + B2) * G2)
                * e
                + ((-B2 + 2 * A * B - A2) * I2
                    + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * H
                        + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * G)
                        * I
                    + (-C2 + 2 * A * C - A2) * H2
                    + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * G * H
                    + (-C2 + 2 * B * C - B2) * G2)
                    * d
                + (((A - B) * H + (B - A) * G + (A - B) * E + (B - A) * D) * I2
                    + ((C - A) * H2
                        + ((-(BigInt::from(2) * C) + B + A) * G
                            + (C - A) * E
                            + (-C - B + 2 * A) * D
                            + (A - B) * C)
                            * H
                        + (C - B) * G2
                        + ((-C + 2 * B - A) * E + (C - B) * D + (B - A) * C) * G)
                        * I
                    + ((C - A) * D + C2 - A * C) * H2
                    + ((A - C) * E + (B - C) * D - 2 * C2 + (B + A) * C) * G * H
                    + ((C - B) * E + C2 - B * C) * G2)
                    * b
                + (((B - A) * H + (A - B) * G + (B - A) * E + (A - B) * D) * I2
                    + ((A - C) * H2
                        + ((2 * C - B - A) * G
                            + (A - C) * E
                            + (C + B - 2 * A) * D
                            + (B - A) * C)
                            * H
                        + (B - C) * G2
                        + ((C - 2 * B + A) * E + (B - C) * D + (A - B) * C) * G)
                        * I
                    + ((A - C) * D - C2 + A * C) * H2
                    + ((C - A) * E + (C - B) * D + 2 * C2 + (-B - A) * C) * G * H
                    + ((B - C) * E - C2 + B * C) * G2)
                    * a)
                * f2
            + (((-B2 + 2 * A * B - A2) * I2
                + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * H
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * G)
                    * I
                + (-C2 + 2 * A * C - A2) * H2
                + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * G * H
                + (-C2 + 2 * B * C - B2) * G2)
                * e2
                + ((((B - A) * H + (A - B) * G + (A - B) * E + (B - A) * D + B2 - 2 * A * B
                    + A2)
                    * I2
                    + ((A - C) * H2
                        + ((2 * C - B - A) * G
                            + (2 * B - 2 * A) * F
                            + (C - A) * E
                            + (-C - B + 2 * A) * D
                            + (A - B) * C
                            + 2 * A * B
                            - 2 * A2)
                            * H
                        + (B - C) * G2
                        + ((2 * A - 2 * B) * F
                            + (-C + 2 * B - A) * E
                            + (C - B) * D
                            + (B - A) * C
                            - 2 * B2
                            + 2 * A * B)
                            * G)
                        * I
                    + ((2 * A - 2 * C) * F + (C - A) * D - A * C + A2) * H2
                    + ((4 * C - 2 * B - 2 * A) * F + (A - C) * E + (B - C) * D + (B + A) * C
                        - 2 * A * B)
                        * G
                        * H
                    + ((2 * B - 2 * C) * F + (C - B) * E - B * C + B2) * G2)
                    * c
                    + (((B - A) * H + (2 * B - 2 * A) * E + (A - B) * D + A * B - A2) * I2
                        + ((A - C) * H2
                            + ((C - 2 * B + A) * G
                                + (A - B) * F
                                + (2 * A - 2 * C) * E
                                + (C + B - 2 * A) * D
                                + (B - 2 * A) * C
                                - A * B
                                + 2 * A2)
                                * H
                            + ((B - A) * F
                                + (2 * C - 4 * B + 2 * A) * E
                                + (B - C) * D
                                + (2 * A - B) * C
                                - A * B)
                                * G)
                            * I
                        + ((C - A) * G + (C - A) * F + (A - C) * D - C2 + 2 * A * C - A2) * H2
                        + ((B - C) * G2
                            + ((-(BigInt::from(2) * C) + B + A) * F
                                + (2 * C - 2 * A) * E
                                + (C - B) * D
                                + 2 * C2
                                + (-B - 2 * A) * C
                                + A * B)
                                * G)
                            * H
                        + ((C - B) * F + (2 * B - 2 * C) * E - C2 + B * C) * G2)
                        * b
                    + (((2 * A - 2 * B) * H + (B - A) * G + (A - B) * E - B2 + A * B) * I2
                        + ((2 * C - 2 * A) * H2
                            + ((3 * B - 3 * C) * G + (A - B) * F + (C - A) * E + A * C
                                - A * B)
                                * H
                            + (C - B) * G2
                            + ((B - A) * F + (-C + 2 * B - A) * E - A * C + 2 * B2 - A * B)
                                * G)
                            * I
                        + ((A - C) * G + (C - A) * F + C2 - A * C) * H2
                        + ((C - B) * G2
                            + ((-(BigInt::from(2) * C) + B + A) * F + (A - C) * E - 2 * C2
                                + A * C
                                + A * B)
                                * G)
                            * H
                        + ((C - B) * F + (C - B) * E + C2 - B2) * G2)
                        * a)
                    * e
                + ((B2 - 2 * A * B + A2) * I2
                    + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * H
                        + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * G)
                        * I
                    + (C2 - 2 * A * C + A2) * H2
                    + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * G * H
                    + (C2 - 2 * B * C + B2) * G2)
                    * d2
                + ((((A - B) * H + (B - A) * G + (B - A) * E + (A - B) * D - B2 + 2 * A * B
                    - A2)
                    * I2
                    + ((C - A) * H2
                        + ((-(BigInt::from(2) * C) + B + A) * G
                            + (2 * A - 2 * B) * F
                            + (A - C) * E
                            + (C + B - 2 * A) * D
                            + (B - A) * C
                            - 2 * A * B
                            + 2 * A2)
                            * H
                        + (C - B) * G2
                        + ((2 * B - 2 * A) * F
                            + (C - 2 * B + A) * E
                            + (B - C) * D
                            + (A - B) * C
                            + 2 * B2
                            - 2 * A * B)
                            * G)
                        * I
                    + ((2 * C - 2 * A) * F + (A - C) * D + A * C - A2) * H2
                    + ((-(BigInt::from(4) * C) + 2 * B + 2 * A) * F
                        + (C - A) * E
                        + (C - B) * D
                        + (-B - A) * C
                        + 2 * A * B)
                        * G
                        * H
                    + ((2 * C - 2 * B) * F + (B - C) * E + B * C - B2) * G2)
                    * c
                    + (((B - A) * H + (2 * A - 2 * B) * G + (A - B) * D - A * B + A2) * I2
                        + ((A - C) * H2
                            + ((3 * C - 3 * A) * G
                                + (B - A) * F
                                + (C + B - 2 * A) * D
                                + B * C
                                + A * B
                                - 2 * A2)
                                * H
                            + (2 * B - 2 * C) * G2
                            + ((A - B) * F + (B - C) * D - B * C + A * B) * G)
                            * I
                        + ((A - C) * G + (A - C) * F + (A - C) * D - C2 + A2) * H2
                        + ((C - B) * G2
                            + ((2 * C - B - A) * F + (C - B) * D + 2 * C2 - B * C - A * B) * G)
                            * H
                        + ((B - C) * F - C2 + B * C) * G2)
                        * b
                    + (((B - A) * G + (A - B) * E + (2 * B - 2 * A) * D + B2 - A * B) * I2
                        + (((-C - B + 2 * A) * G
                            + (B - A) * F
                            + (C - A) * E
                            + (-(BigInt::from(2) * C) - 2 * B + 4 * A) * D
                            + (A - 2 * B) * C
                            + A * B)
                            * H
                            + (C - B) * G2
                            + ((A - B) * F
                                + (-C + 2 * B - A) * E
                                + (2 * C - 2 * B) * D
                                + (2 * B - A) * C
                                - 2 * B2
                                + A * B)
                                * G)
                            * I
                        + ((C - A) * G + (A - C) * F + (2 * C - 2 * A) * D + C2 - A * C) * H2
                        + ((B - C) * G2
                            + ((2 * C - B - A) * F + (A - C) * E + (2 * B - 2 * C) * D
                                - 2 * C2
                                + (2 * B + A) * C
                                - A * B)
                                * G)
                            * H
                        + ((B - C) * F + (C - B) * E + C2 - 2 * B * C + B2) * G2)
                        * a)
                    * d
                + ((((E - D) * H
                    + (D - E) * G
                    + E2
                    + (-(BigInt::from(2) * D) - B + A) * E
                    + D2
                    + (B - A) * D)
                    * I2
                    + ((D - F) * H2
                        + ((2 * F - E - D) * G
                            + (-E + D + B - A) * F
                            + (2 * D + 2 * C - A) * E
                            - 2 * D2
                            + (-(BigInt::from(2) * C) - B + 2 * A) * D)
                            * H
                        + (E - F) * G2
                        + ((E - D - B + A) * F - 2 * E2
                            + (2 * D - 2 * C + 2 * B - A) * E
                            + (2 * C - B) * D)
                            * G)
                        * I
                    + ((-D - 2 * C + A) * F + D2 + (2 * C - A) * D) * H2
                    + ((E + D + 4 * C - B - A) * F
                        + (-(BigInt::from(2) * D) - 2 * C + A) * E
                        + (B - 2 * C) * D)
                        * G
                        * H
                    + ((-E - 2 * C + B) * F + E2 + (2 * C - B) * E) * G2)
                    * b
                    + (((D - E) * H + (E - D) * G - E2 + (2 * D + B - A) * E - D2
                        + (A - B) * D)
                        * I2
                        + ((F - D) * H2
                            + ((-(BigInt::from(2) * F) + E + D) * G
                                + (E - D - B + A) * F
                                + (-(BigInt::from(2) * D) - 2 * C + A) * E
                                + 2 * D2
                                + (2 * C + B - 2 * A) * D)
                                * H
                            + (F - E) * G2
                            + ((-E + D + B - A) * F
                                + 2 * E2
                                + (-(BigInt::from(2) * D) + 2 * C - 2 * B + A) * E
                                + (B - 2 * C) * D)
                                * G)
                            * I
                        + ((D + 2 * C - A) * F - D2 + (A - 2 * C) * D) * H2
                        + ((-E - D - 4 * C + B + A) * F
                            + (2 * D + 2 * C - A) * E
                            + (2 * C - B) * D)
                            * G
                            * H
                        + ((E + 2 * C - B) * F - E2 + (B - 2 * C) * E) * G2)
                        * a)
                    * c
                + (((D - E) * H - E2 + (D - A) * E + A * D) * I2
                    + ((F - D) * H2
                        + ((-F + 2 * E - D) * G
                            + (E + A) * F
                            + (-D - C + A) * E
                            + (C - 2 * A) * D)
                            * H
                        + ((-E - A) * F + 2 * E2 + (-D + C + A) * E - C * D) * G)
                        * I
                    + ((D - F) * G + (C - A) * F + (A - C) * D) * H2
                    + ((F - E) * G2 + ((-E - 2 * C + A) * F + (D + C - A) * E + C * D) * G) * H
                    + ((E + C) * F - E2 - C * E) * G2)
                    * b2
                + (((E - D) * H + (E - D) * G + E2 + (B + A) * E - D2 + (-B - A) * D) * I2
                    + ((D - F) * H2
                        + ((3 * D - 3 * E) * G + (-E - D - B - A) * F - A * E
                            + 2 * D2
                            + (B + 2 * A) * D)
                            * H
                        + (F - E) * G2
                        + ((E + D + B + A) * F - 2 * E2
                            + (-(BigInt::from(2) * B) - A) * E
                            + B * D)
                            * G)
                        * I
                    + ((2 * F - 2 * D) * G + (D + A) * F - D2 - A * D) * H2
                    + ((2 * E - 2 * F) * G2 + ((E - D + B - A) * F + A * E - B * D) * G) * H
                    + ((-E - B) * F + E2 + B * E) * G2)
                    * a
                    * b
                + (((D - E) * G + (-D - B) * E + D2 + B * D) * I2
                    + (((F + E - 2 * D) * G + (D + B) * F + (D + C) * E - 2 * D2
                        + (-C - B) * D)
                        * H
                        + (E - F) * G2
                        + ((-D - B) * F + (D - C + 2 * B) * E + (C - B) * D) * G)
                        * I
                    + ((D - F) * G + (-D - C) * F + D2 + C * D) * H2
                    + ((F - E) * G2 + ((D + 2 * C - B) * F + (-D - C) * E + (B - C) * D) * G)
                        * H
                    + ((B - C) * F + (C - B) * E) * G2)
                    * a2)
                * f
            + (((B2 - 2 * A * B + A2) * I2
                + (((2 * A - 2 * B) * C + 2 * A * B - 2 * A2) * H
                    + ((2 * B - 2 * A) * C - 2 * B2 + 2 * A * B) * G)
                    * I
                + (C2 - 2 * A * C + A2) * H2
                + (-(BigInt::from(2) * C2) + (2 * B + 2 * A) * C - 2 * A * B) * G * H
                + (C2 - 2 * B * C + B2) * G2)
                * d
                + (((A - B) * H + (A - B) * D - B2 + A * B) * I2
                    + ((C - A) * H2
                        + ((-C + 2 * B - A) * G + (A - B) * F + (C + B - 2 * A) * D + B * C
                            - A * B)
                            * H
                        + ((B - A) * F + (B - C) * D - B * C + 2 * B2 - A * B) * G)
                        * I
                    + ((A - C) * G + (C - A) * F + (A - C) * D) * H2
                    + ((C - B) * G2
                        + ((-(BigInt::from(2) * C) + B + A) * F + (C - B) * D - B * C + A * B)
                            * G)
                        * H
                    + ((C - B) * F + B * C - B2) * G2)
                    * c
                + (((B - A) * H + (B - A) * D + B2 - A * B) * I2
                    + ((A - C) * H2
                        + ((C - 2 * B + A) * G + (B - A) * F + (-C - B + 2 * A) * D - B * C
                            + A * B)
                            * H
                        + ((A - B) * F + (C - B) * D + B * C - 2 * B2 + A * B) * G)
                        * I
                    + ((C - A) * G + (A - C) * F + (C - A) * D) * H2
                    + ((B - C) * G2 + ((2 * C - B - A) * F + (B - C) * D + B * C - A * B) * G)
                        * H
                    + ((B - C) * F - B * C + B2) * G2)
                    * a)
                * e2
            + (((-B2 + 2 * A * B - A2) * I2
                + (((2 * B - 2 * A) * C - 2 * A * B + 2 * A2) * H
                    + ((2 * A - 2 * B) * C + 2 * B2 - 2 * A * B) * G)
                    * I
                + (-C2 + 2 * A * C - A2) * H2
                + (2 * C2 + (-(BigInt::from(2) * B) - 2 * A) * C + 2 * A * B) * G * H
                + (-C2 + 2 * B * C - B2) * G2)
                * d2
                + ((((B - A) * H + (B - A) * G + (B - A) * E + (B - A) * D + B2 - A2) * I2
                    + ((A - C) * H2
                        + ((3 * A - 3 * B) * G
                            + (A - C) * E
                            + (-C - B + 2 * A) * D
                            + (-B - A) * C
                            + 2 * A2)
                            * H
                        + (C - B) * G2
                        + ((C - 2 * B + A) * E + (C - B) * D + (B + A) * C - 2 * B2) * G)
                        * I
                    + ((2 * C - 2 * A) * G + (C - A) * D + A * C - A2) * H2
                    + ((2 * B - 2 * C) * G2 + ((C - A) * E + (B - C) * D + (B - A) * C) * G)
                        * H
                    + ((B - C) * E - B * C + B2) * G2)
                    * c
                    + (((A - B) * H + (2 * A - 2 * B) * E + (B - A) * D - A * B + A2) * I2
                        + ((C - A) * H2
                            + ((-C + 2 * B - A) * G
                                + (B - A) * F
                                + (2 * C - 2 * A) * E
                                + (-C - B + 2 * A) * D
                                + (2 * A - B) * C
                                + A * B
                                - 2 * A2)
                                * H
                            + ((A - B) * F
                                + (-(BigInt::from(2) * C) + 4 * B - 2 * A) * E
                                + (C - B) * D
                                + (B - 2 * A) * C
                                + A * B)
                                * G)
                            * I
                        + ((A - C) * G + (A - C) * F + (C - A) * D + C2 - 2 * A * C + A2) * H2
                        + ((C - B) * G2
                            + ((2 * C - B - A) * F + (2 * A - 2 * C) * E + (B - C) * D
                                - 2 * C2
                                + (B + 2 * A) * C
                                - A * B)
                                * G)
                            * H
                        + ((B - C) * F + (2 * C - 2 * B) * E + C2 - B * C) * G2)
                        * b
                    + (((A - B) * G + (B - A) * E + (2 * A - 2 * B) * D - B2 + A * B) * I2
                        + (((C + B - 2 * A) * G
                            + (A - B) * F
                            + (A - C) * E
                            + (2 * C + 2 * B - 4 * A) * D
                            + (2 * B - A) * C
                            - A * B)
                            * H
                            + (B - C) * G2
                            + ((B - A) * F
                                + (C - 2 * B + A) * E
                                + (2 * B - 2 * C) * D
                                + (A - 2 * B) * C
                                + 2 * B2
                                - A * B)
                                * G)
                            * I
                        + ((A - C) * G + (C - A) * F + (2 * A - 2 * C) * D - C2 + A * C) * H2
                        + ((C - B) * G2
                            + ((-(BigInt::from(2) * C) + B + A) * F
                                + (C - A) * E
                                + (2 * C - 2 * B) * D
                                + 2 * C2
                                + (-(BigInt::from(2) * B) - A) * C
                                + A * B)
                                * G)
                            * H
                        + ((C - B) * F + (B - C) * E - C2 + 2 * B * C - B2) * G2)
                        * a)
                    * d
                + (((D - E) * H + (E - D) * G + (A - B) * E + (B - A) * D) * I2
                    + ((F - D) * H2
                        + ((-(BigInt::from(2) * F) + E + D) * G + (-E + D + B - A) * F - A * E
                            + (2 * A - B) * D)
                            * H
                        + (F - E) * G2
                        + ((E - D - B + A) * F + (2 * B - A) * E - B * D) * G)
                        * I
                    + (F2 + (A - D) * F - A * D) * H2
                    + (-(BigInt::from(2) * F2) + (E + D - B - A) * F + A * E + B * D) * G * H
                    + (F2 + (B - E) * F - B * E) * G2)
                    * c2
                + ((((E - D) * H + (D + 2 * B - A) * E - D2 + (A - 2 * B) * D) * I2
                    + ((D - F) * H2
                        + ((F - 2 * E + D) * G
                            + (E - 2 * D - 2 * B + A) * F
                            + (-D - C + A) * E
                            + 2 * D2
                            + (C + 2 * B - 2 * A) * D)
                            * H
                        + ((-E + 2 * D + 2 * B - A) * F
                            + (-D + C - 4 * B + A) * E
                            + (2 * B - C) * D)
                            * G)
                        * I
                    + ((F - D) * G - F2 + (2 * D + C - A) * F - D2 + (A - C) * D) * H2
                    + ((E - F) * G2
                        + (2 * F2
                            + (-E - 2 * D - 2 * C + 2 * B + A) * F
                            + (D + C - A) * E
                            + (C - 2 * B) * D)
                            * G)
                        * H
                    + (-F2 + (E + C - 2 * B) * F + (2 * B - C) * E) * G2)
                    * b
                    + (((E - D) * H + (2 * D - 2 * E) * G + (-D - A) * E + D2 + A * D) * I2
                        + ((D - F) * H2
                            + ((3 * F - 3 * D) * G + (E + A) * F + (D + C + A) * E - 2 * D2
                                + (-C - 2 * A) * D)
                                * H
                            + (2 * E - 2 * F) * G2
                            + ((-E - A) * F + (D - C + A) * E + C * D) * G)
                            * I
                        + ((D - F) * G - F2 + (-C - A) * F + D2 + (C + A) * D) * H2
                        + ((F - E) * G2
                            + (2 * F2 + (-E + 2 * C + A) * F + (-D - C - A) * E - C * D) * G)
                            * H
                        + (-F2 + (E - C) * F + C * E) * G2)
                        * a)
                    * c
                + (((D - E) * H + (-D - 2 * B + A) * E + D2 + (2 * B - A) * D) * I2
                    + ((F - D) * H2
                        + ((-F + 2 * E - D) * G
                            + (-E + 2 * D + 2 * B - A) * F
                            + (D + C - A) * E
                            - 2 * D2
                            + (-C - 2 * B + 2 * A) * D)
                            * H
                        + ((E - 2 * D - 2 * B + A) * F
                            + (D - C + 4 * B - A) * E
                            + (C - 2 * B) * D)
                            * G)
                        * I
                    + ((D - F) * G
                        + F2
                        + (-(BigInt::from(2) * D) - C + A) * F
                        + D2
                        + (C - A) * D)
                        * H2
                    + ((F - E) * G2
                        + (-(BigInt::from(2) * F2)
                            + (E + 2 * D + 2 * C - 2 * B - A) * F
                            + (-D - C + A) * E
                            + (2 * B - C) * D)
                            * G)
                        * H
                    + (F2 + (-E - C + 2 * B) * F + (C - 2 * B) * E) * G2)
                    * a
                    * b
                + (((E - D) * G + (D + B) * E - D2 - B * D) * I2
                    + (((-F - E + 2 * D) * G
                        + (-D - B) * F
                        + (-D - C) * E
                        + 2 * D2
                        + (C + B) * D)
                        * H
                        + (F - E) * G2
                        + ((D + B) * F + (-D + C - 2 * B) * E + (B - C) * D) * G)
                        * I
                    + ((F - D) * G + (D + C) * F - D2 - C * D) * H2
                    + ((E - F) * G2 + ((-D - 2 * C + B) * F + (D + C) * E + (C - B) * D) * G)
                        * H
                    + ((C - B) * F + (B - C) * E) * G2)
                    * a2)
                * e
            + ((((A - B) * G + (A - B) * E - A * B + A2) * I2
                + (((C + B - 2 * A) * G + (B - A) * F + (C - A) * E + A * C + A * B - 2 * A2)
                    * H
                    + (B - C) * G2
                    + ((A - B) * F + (-C + 2 * B - A) * E - A * C + A * B) * G)
                    * I
                + ((A - C) * G + (A - C) * F - A * C + A2) * H2
                + ((C - B) * G2 + ((2 * C - B - A) * F + (A - C) * E + A * C - A * B) * G) * H
                + ((B - C) * F + (C - B) * E) * G2)
                * c
                + (((B - A) * G + (B - A) * E + A * B - A2) * I2
                    + (((-C - B + 2 * A) * G + (A - B) * F + (A - C) * E - A * C - A * B
                        + 2 * A2)
                        * H
                        + (C - B) * G2
                        + ((B - A) * F + (C - 2 * B + A) * E + A * C - A * B) * G)
                        * I
                    + ((C - A) * G + (C - A) * F + A * C - A2) * H2
                    + ((B - C) * G2
                        + ((-(BigInt::from(2) * C) + B + A) * F + (C - A) * E - A * C + A * B)
                            * G)
                        * H
                    + ((C - B) * F + (B - C) * E) * G2)
                    * b)
                * d2
            + ((((E - D) * H + (D - E) * G + (B - A) * E + (A - B) * D) * I2
                + ((D - F) * H2
                    + ((2 * F - E - D) * G + (E - D - B + A) * F + A * E + (B - 2 * A) * D) * H
                    + (E - F) * G2
                    + ((-E + D + B - A) * F + (A - 2 * B) * E + B * D) * G)
                    * I
                + (-F2 + (D - A) * F + A * D) * H2
                + (2 * F2 + (-E - D + B + A) * F - A * E - B * D) * G * H
                + (-F2 + (E - B) * F + B * E) * G2)
                * c2
                + ((((2 * D - 2 * E) * H + (E - D) * G - E2 + (D - B) * E + B * D) * I2
                    + ((2 * F - 2 * D) * H2
                        + ((3 * E - 3 * F) * G + (D + B) * F + (-D - C) * E + (C - B) * D) * H
                        + (F - E) * G2
                        + ((-D - B) * F + 2 * E2 + (-D + C + 2 * B) * E + (-C - B) * D) * G)
                        * I
                    + ((D - F) * G + F2 + (C - D) * F - C * D) * H2
                    + ((F - E) * G2
                        + (-(BigInt::from(2) * F2)
                            + (D - 2 * C - B) * F
                            + (D + C) * E
                            + (C + B) * D)
                            * G)
                        * H
                    + (F2 + (C + B) * F - E2 + (-C - B) * E) * G2)
                    * b
                    + (((E - D) * G + E2 + (-D - B + 2 * A) * E + (B - 2 * A) * D) * I2
                        + (((-F - E + 2 * D) * G
                            + (-(BigInt::from(2) * E) + D + B - 2 * A) * F
                            + (D + C - 2 * A) * E
                            + (-C - B + 4 * A) * D)
                            * H
                            + (F - E) * G2
                            + ((2 * E - D - B + 2 * A) * F - 2 * E2
                                + (D - C + 2 * B - 2 * A) * E
                                + (C - B) * D)
                                * G)
                            * I
                        + ((F - D) * G + F2 + (-D - C + 2 * A) * F + (C - 2 * A) * D) * H2
                        + ((E - F) * G2
                            + (-(BigInt::from(2) * F2)
                                + (2 * E + D + 2 * C - B - 2 * A) * F
                                + (-D - C + 2 * A) * E
                                + (B - C) * D)
                                * G)
                            * H
                        + (F2 + (-(BigInt::from(2) * E) - C + B) * F + E2 + (C - B) * E) * G2)
                        * a)
                    * c
                + (((E - D) * H + E2 + (A - D) * E - A * D) * I2
                    + ((D - F) * H2
                        + ((F - 2 * E + D) * G
                            + (-E - A) * F
                            + (D + C - A) * E
                            + (2 * A - C) * D)
                            * H
                        + ((E + A) * F - 2 * E2 + (D - C - A) * E + C * D) * G)
                        * I
                    + ((F - D) * G + (A - C) * F + (C - A) * D) * H2
                    + ((E - F) * G2 + ((E + 2 * C - A) * F + (-D - C + A) * E - C * D) * G) * H
                    + ((-E - C) * F + E2 + C * E) * G2)
                    * b2
                + (((D - E) * G - E2 + (D + B - 2 * A) * E + (2 * A - B) * D) * I2
                    + (((F + E - 2 * D) * G
                        + (2 * E - D - B + 2 * A) * F
                        + (-D - C + 2 * A) * E
                        + (C + B - 4 * A) * D)
                        * H
                        + (E - F) * G2
                        + ((-(BigInt::from(2) * E) + D + B - 2 * A) * F
                            + 2 * E2
                            + (-D + C - 2 * B + 2 * A) * E
                            + (B - C) * D)
                            * G)
                        * I
                    + ((D - F) * G - F2 + (D + C - 2 * A) * F + (2 * A - C) * D) * H2
                    + ((F - E) * G2
                        + (2 * F2
                            + (-(BigInt::from(2) * E) - D - 2 * C + B + 2 * A) * F
                            + (D + C - 2 * A) * E
                            + (C - B) * D)
                            * G)
                        * H
                    + (-F2 + (2 * E + C - B) * F - E2 + (B - C) * E) * G2)
                    * a
                    * b)
                * d
            + (((E2 - 2 * D * E + D2) * I2
                + (((2 * D - 2 * E) * F + 2 * D * E - 2 * D2) * H
                    + ((2 * E - 2 * D) * F - 2 * E2 + 2 * D * E) * G)
                    * I
                + (F2 - 2 * D * F + D2) * H2
                + (-(BigInt::from(2) * F2) + (2 * E + 2 * D) * F - 2 * D * E) * G * H
                + (F2 - 2 * E * F + E2) * G2)
                * b
                + ((-E2 + 2 * D * E - D2) * I2
                    + (((2 * E - 2 * D) * F - 2 * D * E + 2 * D2) * H
                        + ((2 * D - 2 * E) * F + 2 * E2 - 2 * D * E) * G)
                        * I
                    + (-F2 + 2 * D * F - D2) * H2
                    + (2 * F2 + (-(BigInt::from(2) * E) - 2 * D) * F + 2 * D * E) * G * H
                    + (-F2 + 2 * E * F - E2) * G2)
                    * a)
                * c2
            + (((-E2 + 2 * D * E - D2) * I2
                + (((2 * E - 2 * D) * F - 2 * D * E + 2 * D2) * H
                    + ((2 * D - 2 * E) * F + 2 * E2 - 2 * D * E) * G)
                    * I
                + (-F2 + 2 * D * F - D2) * H2
                + (2 * F2 + (-(BigInt::from(2) * E) - 2 * D) * F + 2 * D * E) * G * H
                + (-F2 + 2 * E * F - E2) * G2)
                * b2
                + ((E2 - 2 * D * E + D2) * I2
                    + (((2 * D - 2 * E) * F + 2 * D * E - 2 * D2) * H
                        + ((2 * E - 2 * D) * F - 2 * E2 + 2 * D * E) * G)
                        * I
                    + (F2 - 2 * D * F + D2) * H2
                    + (-(BigInt::from(2) * F2) + (2 * E + 2 * D) * F - 2 * D * E) * G * H
                    + (F2 - 2 * E * F + E2) * G2)
                    * a2)
                * c
            + ((E2 - 2 * D * E + D2) * I2
                + (((2 * D - 2 * E) * F + 2 * D * E - 2 * D2) * H
                    + ((2 * E - 2 * D) * F - 2 * E2 + 2 * D * E) * G)
                    * I
                + (F2 - 2 * D * F + D2) * H2
                + (-(BigInt::from(2) * F2) + (2 * E + 2 * D) * F - 2 * D * E) * G * H
                + (F2 - 2 * E * F + E2) * G2)
                * a
                * b2
            + ((-E2 + 2 * D * E - D2) * I2
                + (((2 * E - 2 * D) * F - 2 * D * E + 2 * D2) * H
                    + ((2 * D - 2 * E) * F + 2 * E2 - 2 * D * E) * G)
                    * I
                + (-F2 + 2 * D * F - D2) * H2
                + (2 * F2 + (-(BigInt::from(2) * E) - 2 * D) * F + 2 * D * E) * G * H
                + (-F2 + 2 * E * F - E2) * G2)
                * a2
                * b)
            / ((((B2 - 2 * A * B + A2) * F
                + ((A - B) * C + A * B - A2) * E
                + ((B - A) * C - B2 + A * B) * D)
                * e
                + ((-B2 + 2 * A * B - A2) * F
                    + ((B - A) * C - A * B + A2) * E
                    + ((A - B) * C + B2 - A * B) * D)
                    * d
                + (((A - B) * E + (B - A) * D) * F
                    + (C - A) * E2
                    + (-(BigInt::from(2) * C) + B + A) * D * E
                    + (C - B) * D2)
                    * b
                + (((B - A) * E + (A - B) * D) * F
                    + (A - C) * E2
                    + (2 * C - B - A) * D * E
                    + (B - C) * D2)
                    * a)
                * i2
                + ((((-B2 + 2 * A * B - A2) * F
                    + ((B - A) * C - A * B + A2) * E
                    + ((A - B) * C + B2 - A * B) * D)
                    * f
                    + (((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + (-C2 + (B + A) * C - A * B) * D)
                        * e
                    + (((B - A) * C + B2 - 3 * A * B + 2 * A2) * F
                        + (-C2 + (3 * A - B) * C + A * B - 2 * A2) * E
                        + (C2 - 2 * A * C - B2 + 2 * A * B) * D)
                        * d
                    + (((B - A) * E + (A - B) * D) * F
                        + (A - C) * E2
                        + (2 * C - B - A) * D * E
                        + (B - C) * D2)
                        * c
                    + ((B - A) * F2
                        + ((A - C) * E + (C - 2 * B + A) * D) * F
                        + (C - A) * D * E
                        + (B - C) * D2)
                        * b
                    + ((A - B) * F2
                        + ((C - B) * E + (-C + 3 * B - 2 * A) * D) * F
                        + (C - A) * E2
                        + (-(BigInt::from(3) * C) + B + 2 * A) * D * E
                        + (2 * C - 2 * B) * D2)
                        * a)
                    * h
                    + (((B2 - 2 * A * B + A2) * F
                        + ((A - B) * C + A * B - A2) * E
                        + ((B - A) * C - B2 + A * B) * D)
                        * f
                        + (((B - A) * C - 2 * B2 + 3 * A * B - A2) * F
                            + (-C2 + 2 * B * C - 2 * A * B + A2) * E
                            + (C2 + (A - 3 * B) * C + 2 * B2 - A * B) * D)
                            * e
                        + (((A - B) * C + B2 - A * B) * F
                            + (C2 + (-B - A) * C + A * B) * E
                            + (-C2 + 2 * B * C - B2) * D)
                            * d
                        + (((A - B) * E + (B - A) * D) * F
                            + (C - A) * E2
                            + (-(BigInt::from(2) * C) + B + A) * D * E
                            + (C - B) * D2)
                            * c
                        + ((A - B) * F2
                            + ((C + 2 * B - 3 * A) * E + (A - C) * D) * F
                            + (2 * A - 2 * C) * E2
                            + (3 * C - 2 * B - A) * D * E
                            + (B - C) * D2)
                            * b
                        + ((B - A) * F2
                            + ((-C - B + 2 * A) * E + (C - B) * D) * F
                            + (C - A) * E2
                            + (B - C) * D * E)
                            * a)
                        * g
                    + (((-B2 + 2 * A * B - A2) * I
                        + ((B - A) * C - A * B + A2) * H
                        + ((A - B) * C + B2 - A * B) * G)
                        * e
                        + ((B2 - 2 * A * B + A2) * I
                            + ((A - B) * C + A * B - A2) * H
                            + ((B - A) * C - B2 + A * B) * G)
                            * d
                        + (((B - A) * E + (A - B) * D) * I
                            + ((B - A) * F + (2 * A - 2 * C) * E + (2 * C - B - A) * D) * H
                            + ((A - B) * F + (2 * C - B - A) * E + (2 * B - 2 * C) * D) * G)
                            * b
                        + (((A - B) * E + (B - A) * D) * I
                            + ((A - B) * F
                                + (2 * C - 2 * A) * E
                                + (-(BigInt::from(2) * C) + B + A) * D)
                                * H
                            + ((B - A) * F
                                + (-(BigInt::from(2) * C) + B + A) * E
                                + (2 * C - 2 * B) * D)
                                * G)
                            * a)
                        * f
                    + (((B - A) * C - A * B + A2) * I
                        + (-C2 + 2 * A * C - A2) * H
                        + (C2 + (-B - A) * C + A * B) * G)
                        * e2
                    + ((((2 * A - 2 * B) * C + B2 - A2) * I
                        + (2 * C2 + (-B - 3 * A) * C + A * B + A2) * H
                        + (-(BigInt::from(2) * C2) + (3 * B + A) * C - B2 - A * B) * G)
                        * d
                        + (((B - A) * E + (A - B) * D) * I
                            + ((2 * A - 2 * B) * F + (C - A) * E + (-C + 2 * B - A) * D) * H
                            + ((2 * B - 2 * A) * F + (-C - B + 2 * A) * E + (C - B) * D) * G)
                            * c
                        + (((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * I
                            + ((2 * C - 2 * A) * F + (2 * A - 2 * C) * D) * H
                            + ((-(BigInt::from(2) * C) + B + A) * F
                                + (C - A) * E
                                + (C - B) * D)
                                * G)
                            * b
                        + (((B - A) * F + (C - B) * E + (A - C) * D) * I
                            + ((2 * B - 2 * C) * F + (A - C) * E + (3 * C - 2 * B - A) * D) * H
                            + ((2 * C - 3 * B + A) * F + (B - A) * E + (2 * B - 2 * C) * D) * G)
                            * a)
                        * e
                    + (((B - A) * C - B2 + A * B) * I
                        + (-C2 + (B + A) * C - A * B) * H
                        + (C2 - 2 * B * C + B2) * G)
                        * d2
                    + ((((A - B) * E + (B - A) * D) * I
                        + ((2 * B - 2 * A) * F + (A - C) * E + (C - 2 * B + A) * D) * H
                        + ((2 * A - 2 * B) * F + (C + B - 2 * A) * E + (B - C) * D) * G)
                        * c
                        + (((B - A) * F + (C - B) * E + (A - C) * D) * I
                            + ((-(BigInt::from(2) * C) - B + 3 * A) * F
                                + (2 * C - 2 * A) * E
                                + (B - A) * D)
                                * H
                            + ((2 * C - 2 * A) * F
                                + (-(BigInt::from(3) * C) + B + 2 * A) * E
                                + (C - B) * D)
                                * G)
                            * b
                        + (((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * I
                            + ((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * H
                            + ((2 * B - 2 * C) * F + (2 * C - 2 * B) * E) * G)
                            * a)
                        * d
                    + (((-E2 + 2 * D * E - D2) * I
                        + ((E - D) * F - D * E + D2) * H
                        + ((D - E) * F + E2 - D * E) * G)
                        * b
                        + ((E2 - 2 * D * E + D2) * I
                            + ((D - E) * F + D * E - D2) * H
                            + ((E - D) * F - E2 + D * E) * G)
                            * a)
                        * c
                    + (((E - D) * F - D * E + D2) * I
                        + (-F2 + 2 * D * F - D2) * H
                        + (F2 + (-E - D) * F + D * E) * G)
                        * b2
                    + (((2 * D - 2 * E) * F + E2 - D2) * I
                        + (2 * F2 + (-E - 3 * D) * F + D * E + D2) * H
                        + (-(BigInt::from(2) * F2) + (3 * E + D) * F - E2 - D * E) * G)
                        * a
                        * b
                    + (((E - D) * F - E2 + D * E) * I
                        + (-F2 + (E + D) * F - D * E) * H
                        + (F2 - 2 * E * F + E2) * G)
                        * a2)
                    * i
                + ((((B - A) * C - A * B + A2) * F
                    + (-C2 + 2 * A * C - A2) * E
                    + (C2 + (-B - A) * C + A * B) * D)
                    * f
                    + (((A - B) * C + A * B - A2) * F
                        + (C2 - 2 * A * C + A2) * E
                        + (-C2 + (B + A) * C - A * B) * D)
                        * d
                    + ((A - B) * F2
                        + ((C - A) * E + (-C + 2 * B - A) * D) * F
                        + (A - C) * D * E
                        + (C - B) * D2)
                        * c
                    + ((B - A) * F2
                        + ((A - C) * E + (C - 2 * B + A) * D) * F
                        + (C - A) * D * E
                        + (B - C) * D2)
                        * a)
                    * h2
                + (((((2 * A - 2 * B) * C + B2 - A2) * F
                    + (2 * C2 + (-B - 3 * A) * C + A * B + A2) * E
                    + (-(BigInt::from(2) * C2) + (3 * B + A) * C - B2 - A * B) * D)
                    * f
                    + (((B - A) * C - A * B + A2) * F
                        + (-C2 + 2 * A * C - A2) * E
                        + (C2 + (-B - A) * C + A * B) * D)
                        * e
                    + (((B - A) * C - B2 + A * B) * F
                        + (-C2 + (B + A) * C - A * B) * E
                        + (C2 - 2 * B * C + B2) * D)
                        * d
                    + ((2 * B - 2 * A) * F2
                        + ((-(BigInt::from(2) * C) - B + 3 * A) * E + (2 * C - 3 * B + A) * D)
                            * F
                        + (C - A) * E2
                        + (B - A) * D * E
                        + (B - C) * D2)
                        * c
                    + ((A - B) * F2
                        + ((C - A) * E + (-C + 2 * B - A) * D) * F
                        + (A - C) * D * E
                        + (C - B) * D2)
                        * b
                    + ((A - B) * F2
                        + ((C + B - 2 * A) * E + (B - C) * D) * F
                        + (A - C) * E2
                        + (C - B) * D * E)
                        * a)
                    * g
                    + ((B2 - 2 * A * B + A2) * I
                        + ((A - B) * C + A * B - A2) * H
                        + ((B - A) * C - B2 + A * B) * G)
                        * f2
                    + ((((A - B) * C + A * B - A2) * I
                        + (C2 - 2 * A * C + A2) * H
                        + (-C2 + (B + A) * C - A * B) * G)
                        * e
                        + (((B - A) * C - 2 * B2 + 3 * A * B - A2) * I
                            + (-C2 + 2 * B * C - 2 * A * B + A2) * H
                            + (C2 + (A - 3 * B) * C + 2 * B2 - A * B) * G)
                            * d
                        + (((2 * A - 2 * B) * E + (2 * B - 2 * A) * D) * I
                            + ((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * H
                            + ((A - B) * F + (-C + 2 * B - A) * E + (C - B) * D) * G)
                            * c
                        + (((A - B) * F
                            + (2 * C - 2 * A) * E
                            + (-(BigInt::from(2) * C) + B + A) * D)
                            * I
                            + ((A - C) * F + (C - A) * D) * H
                            + ((C + B - 2 * A) * F + (2 * A - 2 * C) * E + (C - B) * D) * G)
                            * b
                        + (((B - A) * F + (2 * B - 2 * C) * E + (2 * C - 3 * B + A) * D) * I
                            + ((C - B) * F + (A - C) * E + (B - A) * D) * H
                            + ((A - C) * F + (3 * C - 2 * B - A) * E + (2 * B - 2 * C) * D) * G)
                            * a)
                        * f
                    + ((((B - A) * C - A * B + A2) * I
                        + (-C2 + 2 * A * C - A2) * H
                        + (C2 + (-B - A) * C + A * B) * G)
                        * d
                        + (((2 * B - 2 * A) * F + (A - C) * E + (C - 2 * B + A) * D) * I
                            + ((A - C) * F + (C - A) * D) * H
                            + ((C - 2 * B + A) * F + (C - A) * E + (2 * B - 2 * C) * D) * G)
                            * c
                        + (((2 * A - 2 * B) * F + (C - A) * E + (-C + 2 * B - A) * D) * I
                            + ((C - A) * F + (A - C) * D) * H
                            + ((-C + 2 * B - A) * F + (A - C) * E + (2 * C - 2 * B) * D) * G)
                            * a)
                        * e
                    + (((A - B) * C + B2 - A * B) * I
                        + (C2 + (-B - A) * C + A * B) * H
                        + (-C2 + 2 * B * C - B2) * G)
                        * d2
                    + ((((2 * A - 2 * B) * F + (C + 2 * B - 3 * A) * E + (A - C) * D) * I
                        + ((C - B) * F + (A - C) * E + (B - A) * D) * H
                        + ((-C + 3 * B - 2 * A) * F + (2 * A - 2 * B) * E + (C - B) * D) * G)
                        * c
                        + (((B - A) * F + (2 * A - 2 * C) * E + (2 * C - B - A) * D) * I
                            + ((C - A) * F + (A - C) * D) * H
                            + ((-C - B + 2 * A) * F + (2 * C - 2 * A) * E + (B - C) * D) * G)
                            * b
                        + (((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * I
                            + ((-(BigInt::from(2) * C) + B + A) * F
                                + (C - A) * E
                                + (C - B) * D)
                                * H
                            + ((2 * C - 2 * B) * F + (2 * B - 2 * C) * E) * G)
                            * a)
                        * d
                    + ((E2 - 2 * D * E + D2) * I
                        + ((D - E) * F + D * E - D2) * H
                        + ((E - D) * F - E2 + D * E) * G)
                        * c2
                    + ((((D - E) * F + D * E - D2) * I
                        + (F2 - 2 * D * F + D2) * H
                        + (-F2 + (E + D) * F - D * E) * G)
                        * b
                        + (((E - D) * F - 2 * E2 + 3 * D * E - D2) * I
                            + (-F2 + 2 * E * F - 2 * D * E + D2) * H
                            + (F2 + (D - 3 * E) * F + 2 * E2 - D * E) * G)
                            * a)
                        * c
                    + (((E - D) * F - D * E + D2) * I
                        + (-F2 + 2 * D * F - D2) * H
                        + (F2 + (-E - D) * F + D * E) * G)
                        * a
                        * b
                    + (((D - E) * F + E2 - D * E) * I
                        + (F2 + (-E - D) * F + D * E) * H
                        + (-F2 + 2 * E * F - E2) * G)
                        * a2)
                    * h
                + ((((B - A) * C - B2 + A * B) * F
                    + (-C2 + (B + A) * C - A * B) * E
                    + (C2 - 2 * B * C + B2) * D)
                    * f
                    + (((A - B) * C + B2 - A * B) * F
                        + (C2 + (-B - A) * C + A * B) * E
                        + (-C2 + 2 * B * C - B2) * D)
                        * e
                    + ((A - B) * F2
                        + ((C + B - 2 * A) * E + (B - C) * D) * F
                        + (A - C) * E2
                        + (C - B) * D * E)
                        * c
                    + ((B - A) * F2
                        + ((-C - B + 2 * A) * E + (C - B) * D) * F
                        + (C - A) * E2
                        + (B - C) * D * E)
                        * b)
                    * g.pow(2)
                + (((-B2 + 2 * A * B - A2) * I
                    + ((B - A) * C - A * B + A2) * H
                    + ((A - B) * C + B2 - A * B) * G)
                    * f2
                    + ((((B - A) * C + B2 - 3 * A * B + 2 * A2) * I
                        + (-C2 + (3 * A - B) * C + A * B - 2 * A2) * H
                        + (C2 - 2 * A * C - B2 + 2 * A * B) * G)
                        * e
                        + (((A - B) * C + B2 - A * B) * I
                            + (C2 + (-B - A) * C + A * B) * H
                            + (-C2 + 2 * B * C - B2) * G)
                            * d
                        + (((2 * B - 2 * A) * E + (2 * A - 2 * B) * D) * I
                            + ((A - B) * F + (A - C) * E + (C + B - 2 * A) * D) * H
                            + ((B - A) * F + (C - 2 * B + A) * E + (B - C) * D) * G)
                            * c
                        + (((B - A) * F
                            + (-(BigInt::from(2) * C) - B + 3 * A) * E
                            + (2 * C - 2 * A) * D)
                            * I
                            + ((C - B) * F
                                + (2 * C - 2 * A) * E
                                + (-(BigInt::from(3) * C) + B + 2 * A) * D)
                                * H
                            + ((A - C) * F + (B - A) * E + (C - B) * D) * G)
                            * b
                        + (((A - B) * F + (2 * C - B - A) * E + (2 * B - 2 * C) * D) * I
                            + ((-C + 2 * B - A) * F + (A - C) * E + (2 * C - 2 * B) * D) * H
                            + ((C - B) * F + (B - C) * E) * G)
                            * a)
                        * f
                    + (((A - B) * C + A * B - A2) * I
                        + (C2 - 2 * A * C + A2) * H
                        + (-C2 + (B + A) * C - A * B) * G)
                        * e2
                    + ((((B - A) * C - B2 + A * B) * I
                        + (-C2 + (B + A) * C - A * B) * H
                        + (C2 - 2 * B * C + B2) * G)
                        * d
                        + (((2 * A - 2 * B) * F + (C - B) * E + (-C + 3 * B - 2 * A) * D) * I
                            + ((C + 2 * B - 3 * A) * F + (A - C) * E + (2 * A - 2 * B) * D) * H
                            + ((A - C) * F + (B - A) * E + (C - B) * D) * G)
                            * c
                        + (((B - A) * F + (C - A) * E + (-C - B + 2 * A) * D) * I
                            + ((2 * A - 2 * C) * F + (2 * C - 2 * A) * D) * H
                            + ((2 * C - B - A) * F + (A - C) * E + (B - C) * D) * G)
                            * b
                        + (((B - A) * F
                            + (-(BigInt::from(2) * C) + B + A) * E
                            + (2 * C - 2 * B) * D)
                            * I
                            + ((C - 2 * B + A) * F + (C - A) * E + (2 * B - 2 * C) * D) * H
                            + ((B - C) * F + (C - B) * E) * G)
                            * a)
                        * e
                    + ((((2 * B - 2 * A) * F + (-C - B + 2 * A) * E + (C - B) * D) * I
                        + ((-C - B + 2 * A) * F + (2 * C - 2 * A) * E + (B - C) * D) * H
                        + ((C - B) * F + (B - C) * E) * G)
                        * c
                        + (((2 * A - 2 * B) * F + (C + B - 2 * A) * E + (B - C) * D) * I
                            + ((C + B - 2 * A) * F + (2 * A - 2 * C) * E + (C - B) * D) * H
                            + ((B - C) * F + (C - B) * E) * G)
                            * b)
                        * d
                    + ((-E2 + 2 * D * E - D2) * I
                        + ((E - D) * F - D * E + D2) * H
                        + ((D - E) * F + E2 - D * E) * G)
                        * c2
                    + ((((E - D) * F + E2 - 3 * D * E + 2 * D2) * I
                        + (-F2 + (3 * D - E) * F + D * E - 2 * D2) * H
                        + (F2 - 2 * D * F - E2 + 2 * D * E) * G)
                        * b
                        + (((D - E) * F + E2 - D * E) * I
                            + (F2 + (-E - D) * F + D * E) * H
                            + (-F2 + 2 * E * F - E2) * G)
                            * a)
                        * c
                    + (((D - E) * F + D * E - D2) * I
                        + (F2 - 2 * D * F + D2) * H
                        + (-F2 + (E + D) * F - D * E) * G)
                        * b2
                    + (((E - D) * F - E2 + D * E) * I
                        + (-F2 + (E + D) * F - D * E) * H
                        + (F2 - 2 * E * F + E2) * G)
                        * a
                        * b)
                    * g
                + ((((A - B) * H + (B - A) * G) * I
                    + (C - A) * H2
                    + (-(BigInt::from(2) * C) + B + A) * G * H
                    + (C - B) * G2)
                    * b
                    + (((B - A) * H + (A - B) * G) * I
                        + (A - C) * H2
                        + (2 * C - B - A) * G * H
                        + (B - C) * G2)
                        * a)
                    * f2
                + (((((B - A) * H + (A - B) * G) * I
                    + (A - C) * H2
                    + (2 * C - B - A) * G * H
                    + (B - C) * G2)
                    * c
                    + ((B - A) * I2
                        + ((A - C) * H + (C - 2 * B + A) * G) * I
                        + (C - A) * G * H
                        + (B - C) * G2)
                        * b
                    + ((A - B) * I2
                        + ((C - B) * H + (-C + 3 * B - 2 * A) * G) * I
                        + (C - A) * H2
                        + (-(BigInt::from(3) * C) + B + 2 * A) * G * H
                        + (2 * C - 2 * B) * G2)
                        * a)
                    * e
                    + ((((A - B) * H + (B - A) * G) * I
                        + (C - A) * H2
                        + (-(BigInt::from(2) * C) + B + A) * G * H
                        + (C - B) * G2)
                        * c
                        + ((A - B) * I2
                            + ((C + 2 * B - 3 * A) * H + (A - C) * G) * I
                            + (2 * A - 2 * C) * H2
                            + (3 * C - 2 * B - A) * G * H
                            + (B - C) * G2)
                            * b
                        + ((B - A) * I2
                            + ((-C - B + 2 * A) * H + (C - B) * G) * I
                            + (C - A) * H2
                            + (B - C) * G * H)
                            * a)
                        * d
                    + ((((E - D) * H + (D - E) * G) * I
                        + (D - F) * H2
                        + (2 * F - E - D) * G * H
                        + (E - F) * G2)
                        * b
                        + (((D - E) * H + (E - D) * G) * I
                            + (F - D) * H2
                            + (-(BigInt::from(2) * F) + E + D) * G * H
                            + (F - E) * G2)
                            * a)
                        * c
                    + ((D - E) * I2
                        + ((F - D) * H + (-F + 2 * E - D) * G) * I
                        + (D - F) * G * H
                        + (F - E) * G2)
                        * b2
                    + ((2 * E - 2 * D) * I2
                        + ((-(BigInt::from(2) * F) - E + 3 * D) * H + (2 * F - 3 * E + D) * G)
                            * I
                        + (F - D) * H2
                        + (E - D) * G * H
                        + (E - F) * G2)
                        * a
                        * b
                    + ((D - E) * I2
                        + ((F + E - 2 * D) * H + (E - F) * G) * I
                        + (D - F) * H2
                        + (F - E) * G * H)
                        * a2)
                    * f
                + (((A - B) * I2
                    + ((C - A) * H + (-C + 2 * B - A) * G) * I
                    + (A - C) * G * H
                    + (C - B) * G2)
                    * c
                    + ((B - A) * I2
                        + ((A - C) * H + (C - 2 * B + A) * G) * I
                        + (C - A) * G * H
                        + (B - C) * G2)
                        * a)
                    * e2
                + ((((2 * B - 2 * A) * I2
                    + ((-(BigInt::from(2) * C) - B + 3 * A) * H + (2 * C - 3 * B + A) * G) * I
                    + (C - A) * H2
                    + (B - A) * G * H
                    + (B - C) * G2)
                    * c
                    + ((A - B) * I2
                        + ((C - A) * H + (-C + 2 * B - A) * G) * I
                        + (A - C) * G * H
                        + (C - B) * G2)
                        * b
                    + ((A - B) * I2
                        + ((C + B - 2 * A) * H + (B - C) * G) * I
                        + (A - C) * H2
                        + (C - B) * G * H)
                        * a)
                    * d
                    + (((D - E) * H + (E - D) * G) * I
                        + (F - D) * H2
                        + (-(BigInt::from(2) * F) + E + D) * G * H
                        + (F - E) * G2)
                        * c2
                    + (((E - D) * I2
                        + ((D - F) * H + (F - 2 * E + D) * G) * I
                        + (F - D) * G * H
                        + (E - F) * G2)
                        * b
                        + ((D - E) * I2
                            + ((F + 2 * E - 3 * D) * H + (D - F) * G) * I
                            + (2 * D - 2 * F) * H2
                            + (3 * F - 2 * E - D) * G * H
                            + (E - F) * G2)
                            * a)
                        * c
                    + ((D - E) * I2
                        + ((F - D) * H + (-F + 2 * E - D) * G) * I
                        + (D - F) * G * H
                        + (F - E) * G2)
                        * a
                        * b
                    + ((E - D) * I2
                        + ((-F - E + 2 * D) * H + (F - E) * G) * I
                        + (F - D) * H2
                        + (E - F) * G * H)
                        * a2)
                    * e
                + (((A - B) * I2
                    + ((C + B - 2 * A) * H + (B - C) * G) * I
                    + (A - C) * H2
                    + (C - B) * G * H)
                    * c
                    + ((B - A) * I2
                        + ((-C - B + 2 * A) * H + (C - B) * G) * I
                        + (C - A) * H2
                        + (B - C) * G * H)
                        * b)
                    * d2
                + ((((E - D) * H + (D - E) * G) * I
                    + (D - F) * H2
                    + (2 * F - E - D) * G * H
                    + (E - F) * G2)
                    * c2
                    + (((D - E) * I2
                        + ((F - E) * H + (-F + 3 * E - 2 * D) * G) * I
                        + (F - D) * H2
                        + (-(BigInt::from(3) * F) + E + 2 * D) * G * H
                        + (2 * F - 2 * E) * G2)
                        * b
                        + ((E - D) * I2
                            + ((-F - E + 2 * D) * H + (F - E) * G) * I
                            + (F - D) * H2
                            + (E - F) * G * H)
                            * a)
                        * c
                    + ((E - D) * I2
                        + ((D - F) * H + (F - 2 * E + D) * G) * I
                        + (F - D) * G * H
                        + (E - F) * G2)
                        * b2
                    + ((D - E) * I2
                        + ((F + E - 2 * D) * H + (E - F) * G) * I
                        + (D - F) * H2
                        + (F - E) * G * H)
                        * a
                        * b)
                    * d);
        sum.try_into().map_err(|_| AocError.into())
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
