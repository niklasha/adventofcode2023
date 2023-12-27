use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use graphalgs::connect::find_bridges;
use petgraph::algo::has_path_connecting;
use petgraph::graphmap::UnGraphMap;

use crate::day::*;

pub struct Day25 {}

type Output = usize;

impl Day for Day25 {
    fn tag(&self) -> &str {
        "25"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("^(.*): (.*)$").unwrap();
}

impl Day25 {
    fn parse_wiring(spec: BoxResult<String>) -> BoxResult<HashSet<(String, String)>> {
        spec.and_then(|s| {
            let (_, [head, components]) = PATTERN.captures(&s).ok_or(AocError)?.extract(); // XXX
            Ok(components
                .split_whitespace()
                .map(|component| (head.to_string(), component.to_string()))
                .collect::<HashSet<_>>())
        })
    }

    fn parse(input: &mut dyn io::Read) -> BoxResult<HashSet<(String, String)>> {
        io::BufReader::new(input)
            .lines()
            .try_fold(HashSet::new(), |mut bonds, rs| {
                for bond in Self::parse_wiring(rs.map_err(|e| AocError.into()))? {
                    assert!(bonds.insert(bond));
                }
                Ok(bonds)
            })
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let bonds = Self::parse(input)?;
        let mut graph = UnGraphMap::new();
        for (a, b) in &bonds {
            graph.add_edge(a, b, "");
        }
        bonds.iter().tuple_combinations().flat_map(|(e1@(a1, b1), e2@(a2, b2))| {
            let mut graph = graph.clone();
            graph.remove_edge(a1, b1);
            graph.remove_edge(a2, b2);
            let bridges = find_bridges(&graph);
            if let Some(e3) = bridges.get(0) {
                Some((e1, e2, *e3))
            } else {
                None
            }
        }).next().map(|(a, b, c)| {
            graph.remove_edge(&a.0, &a.1);
            graph.remove_edge(&b.0, &b.1);
            graph.remove_edge(&c.0, &c.1);
            let (left, right) = graph.nodes().fold((0, 0), |(left, right), node| if has_path_connecting(&graph, node, &a.0, None) { (left + 1, right) } else { (left, right + 1) });
            left * right
        }).ok_or(AocError.into())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day25 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
            54,
        );
    }
}
