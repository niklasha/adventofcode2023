use num::integer::lcm;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use regex::Regex;

use crate::day::*;

pub struct Day20 {}

type Output = usize;

impl Day for Day20 {
    fn tag(&self) -> &str {
        "20"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref MODULE_PATTERN: Regex = Regex::new("^([%&]?)([a-z]+) -> (.+)$").unwrap();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Kind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: Kind,
    targets: Vec<String>,
}

trait Mod: Debug {
    fn pulse(&mut self, _: &str, _: bool) -> Option<bool>;
    fn targets(&self) -> Vec<&str>;
    fn record_source(&mut self, source: &str);
    fn kind(&self) -> Kind;
}

#[derive(Debug)]
struct FlipFlop {
    metadata: Module,
    state: bool,
}

impl Mod for FlipFlop {
    fn pulse(&mut self, _: &str, pulse: bool) -> Option<bool> {
        if !pulse {
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }

    fn targets(&self) -> Vec<&str> {
        self.metadata
            .targets
            .iter()
            .map(|s| s.as_str())
            .collect_vec()
    }

    fn record_source(&mut self, target: &str) {}

    fn kind(&self) -> Kind {
        Kind::FlipFlop
    }
}

#[derive(Debug)]
struct Conjunction {
    metadata: Module,
    memory: HashMap<String, bool>,
}

impl Mod for Conjunction {
    fn pulse(&mut self, source: &str, pulse: bool) -> Option<bool> {
        *self.memory.entry(source.to_string()).or_insert(false) = pulse;
        Some(!self.memory.iter().all(|(_, memory)| *memory))
    }

    fn targets(&self) -> Vec<&str> {
        self.metadata
            .targets
            .iter()
            .map(|s| s.as_str())
            .collect_vec()
    }

    fn record_source(&mut self, target: &str) {
        self.memory.insert(target.to_string(), false);
    }

    fn kind(&self) -> Kind {
        Kind::Conjunction
    }
}

#[derive(Debug)]
struct Broadcaster {
    metadata: Module,
}

impl Mod for Broadcaster {
    fn pulse(&mut self, source: &str, pulse: bool) -> Option<bool> {
        Some(pulse)
    }

    fn targets(&self) -> Vec<&str> {
        self.metadata
            .targets
            .iter()
            .map(|s| s.as_str())
            .collect_vec()
    }

    fn record_source(&mut self, target: &str) {}

    fn kind(&self) -> Kind {
        Kind::Broadcaster
    }
}

impl Day20 {
    fn parse_module(spec: BoxResult<String>) -> BoxResult<(String, Box<dyn Mod>)> {
        spec.and_then(|s| {
            let (_, [kind, name, targets]) = MODULE_PATTERN.captures(&s).unwrap().extract(); // XXX
            let targets = targets.split(", ").map(ToString::to_string).collect();
            let module: Box<dyn Mod> = match kind {
                "%" => Ok(Box::new(FlipFlop {
                    metadata: Module {
                        name: name.to_string(),
                        kind: Kind::FlipFlop,
                        targets,
                    },
                    state: false,
                }) as Box<dyn Mod>),
                "&" => Ok(Box::new(Conjunction {
                    metadata: Module {
                        name: name.to_string(),
                        kind: Kind::Conjunction,
                        targets,
                    },
                    memory: HashMap::new(),
                }) as Box<dyn Mod>),
                "" => Ok(Box::new(Broadcaster {
                    metadata: Module {
                        name: name.to_string(),
                        kind: Kind::Broadcaster,
                        targets,
                    },
                }) as Box<dyn Mod>),
                _ => Err(AocError),
            }?;
            Ok((name.to_string(), module))
        })
    }

    fn parse(
        input: &mut dyn io::Read,
    ) -> BoxResult<(HashMap<String, Box<dyn Mod>>, HashSet<String>)> {
        let mut modules = io::BufReader::new(input)
            .lines()
            .map(|rs| Self::parse_module(rs.map_err(|e| AocError.into())))
            .collect::<BoxResult<HashMap<_, _>>>();
        if let Ok(ref mut modules) = modules {
            let mappings = modules
                .iter()
                .flat_map(|(source, module)| {
                    module
                        .targets()
                        .iter()
                        .map(|&t| (source.to_string(), t.to_string()))
                        .collect::<Vec<_>>()
                })
                .collect_vec();
            for (source, target) in mappings {
                if let Some(target) = modules.get_mut(&target) {
                    target.record_source(&source);
                }
            }
        }
        modules.map(|m| {
            let all = m.keys().map(|k| k.to_string()).collect::<HashSet<_>>();
            (m, all)
        })
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let modules = Self::parse(input)?
            .0
            .into_iter()
            .map(|(k, v)| (k, RefCell::new(v)))
            .collect::<HashMap<_, _>>();
        let mut cnt_low = 0 as Output;
        let mut cnt_high = 0 as Output;
        for i in 0..1000 {
            let mut targets =
                VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);
            while let Some((source, target, pulse)) = targets.pop_front() {
                if pulse {
                    cnt_high += 1
                } else {
                    cnt_low += 1
                }
                if let Some(module) = modules.get(&target) {
                    let mut module = module.borrow_mut();
                    if let Some(pulse) = module.pulse(&source, pulse) {
                        targets.extend(
                            module
                                .targets()
                                .into_iter()
                                .map(ToString::to_string)
                                .map(|new_target| {
                                    (target.to_string(), new_target.to_string(), pulse)
                                })
                                .collect_vec(),
                        );
                    }
                }
            }
        }
        Ok(cnt_low * cnt_high)
    }

    fn reversed(
        modules: &HashMap<String, Box<dyn Mod>>,
    ) -> HashMap<String, HashSet<(String, Kind)>> {
        let mut r = HashMap::new();
        for (source, module) in modules {
            for (source, target, kind) in modules
                .iter()
                .flat_map(|(source, module)| {
                    module
                        .targets()
                        .iter()
                        .map(|&t| (source.to_string(), t.to_string(), module.kind()))
                        .collect::<Vec<_>>()
                })
                .collect_vec()
            {
                (*r.entry(target).or_insert(HashSet::new())).insert((source, kind));
            }
        }
        r
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (m, all) = Self::parse(input)?;
        let reversed = Self::reversed(&m);

        // XXX Assume the output is a conjunction of periodic pulses, implemented as a N-in NAND followed by a 1-in NAND.
        let rx_parents = reversed.get("rx").unwrap();
        let mut rx_grandparents: HashMap<String, Option<Output>> = rx_parents
            .iter()
            .flat_map(|(parent, _)| reversed.get(parent).unwrap())
            .map(|(s, _)| (s.to_string(), None))
            .collect();

        let modules = m
            .into_iter()
            .map(|(k, v)| (k, RefCell::new(v)))
            .collect::<HashMap<_, _>>();
        // Loop until all the conjuncts periods has been found.
        for i in 1.. {
            let mut targets =
                VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);
            while let Some((source, target, pulse)) = targets.pop_front() {
                if let Some(period) = rx_grandparents.get_mut(&target) {
                    if !pulse && period.is_none() {
                        *period = Some(i);
                        let lcm = rx_grandparents
                            .values()
                            .map(|v| v.unwrap_or(0))
                            .fold(1, lcm);
                        if lcm != 0 {
                            return Ok(lcm);
                        }
                    }
                }
                if let Some(module) = modules.get(&target) {
                    let mut module = module.borrow_mut();
                    if let Some(pulse) = module.pulse(&source, pulse) {
                        targets.extend(
                            module
                                .targets()
                                .into_iter()
                                .map(ToString::to_string)
                                .map(|new_target| {
                                    (target.to_string(), new_target.to_string(), pulse)
                                })
                                .collect_vec(),
                        );
                    }
                }
            }
        }
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day20 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
            32000000,
        );
        test1(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
            11687500,
        );
    }
}
