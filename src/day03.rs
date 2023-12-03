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
    x: usize,
    y: usize,
    n: String,
    c: u8,
    cnt: usize,
    prod: Output,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Day03 {
    fn process<F, G>(
        input: &mut dyn io::Read,
        update_part: F,
        collect_results: G,
    ) -> BoxResult<Vec<Part>>
    where
        F: Fn(&mut Part, &mut Part) -> BoxResult<()>,
        G: Fn(Vec<Part>, &HashMap<Coord, Part>) -> Vec<Part>,
    {
        let mut unidentified_parts = HashMap::new();
        let mut numbers = Vec::new();
        io::BufReader::new(input)
            .lines()
            .enumerate()
            .map(|(y, rs)| {
                let _ = rs.map_err(|e| e.into()).and_then(|s| {
                    let mut p: Option<Part> = None;
                    for x in 0..s.len() {
                        let c = s.as_bytes()[x];
                        if c.is_ascii_digit() {
                            if let Some(ref mut p) = &mut p {
                                p.n.push(c as char);
                            } else {
                                p = Some(Part {
                                    x,
                                    y,
                                    n: format!("{}", c as char),
                                    c: b'.',
                                    cnt: 0,
                                    prod: 1,
                                })
                            }
                        } else if let Some(inner_p) = p {
                            numbers.push(inner_p);
                            p = None;
                        }
                        if c != b'.' && !c.is_ascii_digit() {
                            unidentified_parts.insert(
                                Coord { x, y },
                                Part {
                                    x,
                                    y,
                                    n: String::new(),
                                    c,
                                    cnt: 0,
                                    prod: 1,
                                },
                            );
                        }
                    }
                    if let Some(inner_p) = p {
                        numbers.push(inner_p);
                    }
                    BoxResult::Ok(())
                });
                Ok(())
            })
            .collect::<BoxResult<Vec<_>>>()?;
        for n in &mut numbers {
            for y in if n.y == 0 { 0 } else { n.y - 1 }..=(n.y + 1) {
                for x in if n.x == 0 { 0 } else { n.x - 1 }..=(n.x + n.n.len()) {
                    let p = unidentified_parts.get_mut(&Coord { x, y });
                    if let Some(p) = p {
                        p.n = n.n.to_owned();
                        update_part(p, n)?;
                    }
                }
            }
        }
        Ok(collect_results(numbers, &unidentified_parts))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(
            input,
            |p, n| {
                n.c = p.c;
                Ok(())
            },
            |numbers, _| numbers.into_iter().filter(|p| p.c != b'.').collect(),
        );
        Ok(r?.into_iter().map(|p| p.n.parse::<Output>().unwrap()).sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let r = Self::process(
            input,
            |p, n| {
                p.cnt += 1;
                p.prod *= n.n.parse::<Output>()?;
                Ok(())
            },
            |_, unidentified_parts| {
                unidentified_parts
                    .values()
                    .filter(|p| p.c == b'*' && p.cnt == 2)
                    .cloned()
                    .collect()
            },
        );
        Ok(r?.into_iter().map(|p| p.prod).sum())
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
