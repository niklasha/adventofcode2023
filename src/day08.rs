use crate::day::*;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

pub struct Day08 {}

type Output = usize;

impl Day for Day08 {
    fn tag(&self) -> &str {
        "08"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref NODE_PATTERN: Regex = Regex::new("^(.*) = \\((.*), (.*)\\)$").unwrap();
}

impl Day08 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(String, HashMap<String, (String, String)>)> {
        let mut lines = io::BufReader::new(input).lines();
        let steps = lines
            .next()
            .ok_or::<AocError>(AocError)?
            .map_err(|_| AocError)?;
        let _ = lines
            .next()
            .ok_or::<AocError>(AocError)?
            .map_err(|_| AocError)?;
        BoxResult::Ok(
            lines
                .try_fold(HashMap::new(), |mut nodes, rs| {
                    rs.map_err(|_| AocError).and_then(|s| {
                        Ok({
                            let (_, [node, from, to]) =
                                NODE_PATTERN.captures(&s).ok_or(AocError)?.extract();
                            nodes.insert(node.to_string(), (from.to_string(), to.to_string()));
                            nodes
                        })
                    })
                })
                .map_err(|_| AocError)
                .map(|nodes| (steps, nodes))?,
        )
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (steps, nodes) = Self::parse(input)?;
        Self::distance_to(&steps, &nodes, "AAA", |node, _, _| node == "ZZZ")
    }

    fn distance_to<F>(
        steps: &str,
        nodes: &HashMap<String, (String, String)>,
        start: &str,
        mut is_at_end: F,
    ) -> BoxResult<Output>
    where
        F: FnMut(&str, Output, usize) -> bool,
    {
        is_at_end(start, 0, 0);
        steps
            .chars()
            .enumerate()
            .cycle()
            .try_fold((start, 1 as Output), |(node, count), step| {
                let (left, right) = nodes.get(node).ok_or(Err(AocError.into()))?;
                let node = match step.1 {
                    'L' => left,
                    'R' => right,
                    _ => Err(Err(AocError.into()))?,
                }
                .as_str();
                if is_at_end(node, count, step.0) {
                    Err(Ok(count))
                } else {
                    Ok((node, count + 1))
                }
            })
            .and_then(|_| Err(Err(AocError.into())))
            .or_else(|count| count)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (steps, map) = Self::parse(input)?;
        let a_nodes = map
            .keys()
            .map(|s| s.as_str())
            .filter(|s| s.ends_with('A'))
            .collect_vec();
        let cycles = a_nodes
            .iter()
            .map(|&start| {
                let mut visited = HashMap::new();
                let mut z_node = None;
                let mut cycle = None;
                let _ = Self::distance_to(&steps, &map, start, |node, count, step| {
                    if node.ends_with('Z') {
                        z_node = Some(count);
                    }
                    if let Some(&start) = visited.get(&(node.to_string(), step)) {
                        cycle = Some((node.to_string(), start, count - start, z_node.unwrap()));
                        true
                    } else {
                        visited.insert((node.to_string(), step), count);
                        false
                    }
                });
                (start.to_string(), cycle.unwrap())
            })
            .collect::<HashMap<_, _>>();
        Ok(cycles
            .into_iter()
            .map(|(_, (_, _offset, _period, z_offset))| z_offset)
            .fold(1, lcm))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day08 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
            2,
        );
        test1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
            6,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day08 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
            6,
        );
    }
}
