use crate::day::*;
use std::collections::HashMap;

pub struct Day03 {}

type Output = usize;

impl Day for Day03 {
    fn tag(&self) -> &str {
        "03"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Debug)]
struct Part {
    part_nos: Vec<Output>,
    kind: u8,
}

struct PartNo {
    coord: Coord,
    part_no: String,
    kind: Option<u8>,
}

impl PartNo {
    fn new(x: usize, y: usize, c: u8) -> Self {
        Self {
            coord: Coord { x, y },
            part_no: format!("{}", c as char),
            kind: None,
        }
    }
    fn extend(&mut self, c: u8) {
        self.part_no.push(c as char);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn left_of_x(&self) -> usize {
        if self.x == 0 {
            0
        } else {
            self.x - 1
        }
    }
    fn above_y(&self) -> usize {
        if self.y == 0 {
            0
        } else {
            self.y - 1
        }
    }
}

const VOID: u8 = b'.';
const GEAR: u8 = b'*';

impl Day03 {
    fn process<R, F, G>(
        input: &mut dyn io::Read,
        update_part: F,
        collect_results: G,
    ) -> BoxResult<Vec<R>>
    where
        F: Fn(&mut Part, &mut PartNo),
        G: Fn(Vec<PartNo>, &HashMap<Coord, Part>) -> Vec<R>,
    {
        let mut unidentified_parts = HashMap::new();
        let mut part_nos = Vec::new();
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .map(|(y, rs)| {
                let _: BoxResult<()> = rs.map_err(|e| e.into()).map(|s| {
                    let mut current_part_no: Option<PartNo> = None;
                    for x in 0..s.len() {
                        let c = s.as_bytes()[x];
                        if c.is_ascii_digit() {
                            if let Some(ref mut current_part_no) = &mut current_part_no {
                                current_part_no.extend(c);
                            } else {
                                current_part_no = Some(PartNo::new(x, y, c));
                            }
                        } else if let Some(part_no) = current_part_no {
                            part_nos.push(part_no);
                            current_part_no = None;
                        }
                        if c != VOID && !c.is_ascii_digit() {
                            let coord = Coord { x, y };
                            unidentified_parts.insert(
                                coord.to_owned(),
                                Part {
                                    part_nos: vec![],
                                    kind: c,
                                },
                            );
                        }
                    }
                    if let Some(current_part_no) = current_part_no {
                        part_nos.push(current_part_no);
                    }
                });
                Ok(())
            })
            .collect::<BoxResult<Vec<_>>>()?;
        for n in &mut part_nos {
            for y in n.coord.above_y()..=(n.coord.y + 1) {
                for x in n.coord.left_of_x()..=(n.coord.x + n.part_no.len()) {
                    let p = unidentified_parts.get_mut(&Coord { x, y });
                    if let Some(p) = p {
                        p.part_nos.push(n.part_no.parse()?);
                        update_part(p, n);
                    }
                }
            }
        }
        Ok(collect_results(part_nos, &unidentified_parts))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(
            input,
            |p, n| n.kind = Some(p.kind),
            |part_nos, _| part_nos.into_iter().filter(|p| p.kind.is_some()).collect(),
        );
        Ok(r?
            .into_iter()
            .map(|p| p.part_no.parse::<Output>().unwrap())
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(
            input,
            |_, _| {},
            |_, unidentified_parts| {
                unidentified_parts
                    .values()
                    .filter(|p| p.kind == GEAR && p.part_nos.len() == 2)
                    .cloned()
                    .collect()
            },
        );
        Ok(r?
            .into_iter()
            .map(|p| p.part_nos.iter().product::<Output>())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day03 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            4361,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day03 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            467835,
        );
    }
}
