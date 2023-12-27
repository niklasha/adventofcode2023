use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use regex::Regex;

use crate::day::*;

pub struct Day22 {}

type Output = usize;

impl Day for Day22 {
    fn tag(&self) -> &str {
        "22"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("^(\\d),(\\d),(\\d+)~(\\d),(\\d),(\\d+)$").unwrap();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Brick((usize, usize, usize), (usize, usize, usize));

impl PartialEq<Brick> for &mut Brick {
    fn eq(&self, other: &Brick) -> bool {
        (*self).eq(other)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_z().cmp(&other.min_z())
    }
}

impl Brick {
    fn drop(&mut self, support_map: &SupportMap) {
        // XXX BROKEN! Would need rebuilding the support map
        while !self.is_supported(support_map) {
            self.drop_one();
        }
    }

    fn drop_one(&mut self) {
        //println!("brick {:?}", self);
        self.0 .2 -= 1;
        self.1 .2 -= 1;
        //println!("  dropped to {:?}", self);
    }

    fn is_supported(&self, support_map: &SupportMap) -> bool {
        self.min_z() == 1
            || support_map
                .iter()
                .any(|(_, supportees)| supportees.contains(self))
    }

    fn is_redundant(&self, support_map: &SupportMap) -> bool {
        // Find out if brick is only supporting others that are supported by someone else.
        support_map.get(self).map_or(true, |supportees| {
            supportees.iter().all(|supportee| {
                support_map
                    .iter()
                    .filter(|(_, bricks)| bricks.contains(supportee))
                    .count()
                    > 1
            })
        })
    }

    fn covers(&self, other: &Brick) -> bool {
        self.x_interleave(other) && self.y_interleave(other)
    }

    fn x_interleave(&self, other: &Brick) -> bool {
        //        println!("x_interleave: {:?} {:?}", self, other);
        let rv = (self.min_x()..=self.max_x()).contains(&other.min_x())
            || (self.min_x()..=self.max_x()).contains(&other.max_x())
            || (self.min_x() > other.min_x() && self.max_x() < other.max_x());
        //        println!("  -> {}", rv);
        rv
    }

    fn y_interleave(&self, other: &Brick) -> bool {
        //        println!("y_interleave: {:?} {:?}", self, other);
        let rv = (self.min_y()..=self.max_y()).contains(&other.min_y())
            || (self.min_y()..=self.max_y()).contains(&other.max_y())
            || (self.min_y() > other.min_y() && self.max_y() < other.max_y());
        //        println!("  -> {}", rv);
        rv
    }

    fn min_x(&self) -> usize {
        usize::min(self.0 .0, self.1 .0)
    }

    fn max_x(&self) -> usize {
        usize::max(self.0 .0, self.1 .0)
    }

    fn min_y(&self) -> usize {
        usize::min(self.0 .1, self.1 .1)
    }

    fn max_y(&self) -> usize {
        usize::max(self.0 .1, self.1 .1)
    }

    fn min_z(&self) -> usize {
        usize::min(self.0 .2, self.1 .2)
    }

    fn max_z(&self) -> usize {
        usize::max(self.0 .2, self.1 .2)
    }
}

type SupportMap = HashMap<Brick, HashSet<Brick>>;

impl Day22 {
    fn parse_brick(spec: BoxResult<String>) -> BoxResult<Brick> {
        spec.and_then(|s| {
            let (_, [x0, y0, z0, x1, y1, z1]) = PATTERN.captures(&s).ok_or(AocError)?.extract(); // XXX
            Ok(Brick(
                (x0.parse()?, y0.parse()?, z0.parse()?),
                (x1.parse()?, y1.parse()?, z1.parse()?),
            ))
        })
    }

    fn parse(input: &mut dyn io::Read) -> BoxResult<Vec<Brick>> {
        let mut bricks = io::BufReader::new(input)
            .lines()
            .map(|rs| Self::parse_brick(rs.map_err(|e| AocError.into())))
            .collect::<BoxResult<Vec<_>>>();
        if let Ok(ref mut bricks) = bricks {
            bricks.sort();
        }
        bricks
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut bricks = Self::parse(input)?;
        Self::drop(&mut bricks);
        let support_map = Self::make_support_map(&bricks);
        Ok(bricks
            .iter()
            .filter(|brick| (*brick).is_redundant(&support_map))
            .count())
    }

    fn drop_broken(bricks: &mut [Brick], support_map: &SupportMap) {
        for brick in bricks.iter_mut() {
            // XXX BROKEN, see definition
            brick.drop(support_map);
        }
    }

    fn drop(bricks: &mut Vec<Brick>) -> Output {
        // XXX Would have loved to do this as Brick::drop but could not due to mutability and borrowing.
        // XXX Also this should be done functionally!
        let mut cnt = 0;
        for i in 0..bricks.len() {
            let mut do_drop = true;
            let mut did_drop = false;
            while do_drop {
                let z = bricks[i].min_z();
                if z == 1 {
                    do_drop = false;
                    continue;
                }
                for j in 0..bricks.len() {
                    if i != j && bricks[j].max_z() == z - 1 && bricks[i].covers(&bricks[j]) {
                        // println!("will not drop {:?} due to {:?}", bricks[i], bricks[j]);
                        do_drop = false;
                        break;
                    }
                }
                if do_drop {
                    did_drop = true;
                    bricks[i].drop_one();
                }
            }
            if did_drop {
                cnt += 1;
            }
        }
        bricks.sort();
        cnt
    }

    fn make_support_map(bricks: &Vec<Brick>) -> SupportMap {
        let support_map = bricks
            .iter()
            .fold(HashMap::new(), |mut support_map, brick| {
                for other in bricks {
                    if other.min_z() == brick.max_z() + 1 && other.covers(brick) {
                        support_map
                            .entry(*brick)
                            .or_insert(HashSet::new())
                            .insert(*other);
                    }
                }
                support_map
            });
        //println!("support_map: {:?}", support_map);
        support_map
    }

    fn disintegrate(brick: &Brick, mut bricks: Vec<Brick>) -> Output {
        // println!("disintegrate {:?} among {}", brick, bricks.len());
        bricks.retain(|other| brick != other);
        // println!("  new count {}", bricks.len());
        let rv = Self::drop(&mut bricks);
        // println!("  -> {}", rv);
        rv
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut bricks = Self::parse(input)?;
        Self::drop(&mut bricks);
        let support_map = Self::make_support_map(&bricks);
        Ok(bricks
            .iter()
            .filter(|brick| !(*brick).is_redundant(&support_map))
            .map(|brick| Self::disintegrate(brick, bricks.clone()))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day22 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
            5,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day22 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        crate::day22::tests::test2(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
            7,
        );
    }
}
