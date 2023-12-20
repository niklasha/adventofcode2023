use crate::day::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Day19 {}

type Output = usize;

impl Day for Day19 {
    fn tag(&self) -> &str {
        "19"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref WORKFLOW_PATTERN: Regex = Regex::new("^([a-z]+)\\{(.+)\\}$").unwrap();
    static ref RULE_PATTERN: Regex =
        Regex::new("^(?:([xmas])([<>])(\\d+):)?([a-z]+|A|R)$").unwrap();
    static ref PART_PATTERN: Regex = Regex::new("^\\{(.+)\\}$").unwrap();
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Rule {
    condition: Option<(u8, u8, Output)>,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part(HashMap<u8, Output>);

impl Workflow {
    fn run(&self, part: &Part) -> &str {
        &self
            .rules
            .iter()
            .find(|rule| match rule.condition {
                Some((category, op, rule_value)) => {
                    let &part_value = part.0.get(&category).unwrap();
                    match op {
                        b'<' => part_value < rule_value,
                        b'>' => part_value > rule_value,
                        b'=' => part_value == rule_value,
                        _ => unreachable!(),
                    }
                }
                _ => true,
            })
            .unwrap()
            .target
    }
}
impl Day19 {
    fn parse_workflows<I, E>(spec: I) -> Result<HashMap<String, Workflow>, E>
    where
        I: Iterator<Item = Result<String, E>>,
        E: error::Error,
    {
        spec.map(|rs| {
            rs.map(|s| {
                let (_, [name, rules]) = WORKFLOW_PATTERN.captures(&s).unwrap().extract(); // XXX
                let name = name.to_owned();
                let rules = rules
                    .split(',')
                    .map(|rule| {
                        let cap = RULE_PATTERN
                            .captures(rule)
                            .unwrap_or_else(|| panic!("haystack {} did not match", rule)); // XXX
                        let [xmas, op, value, target] = cap
                            .iter()
                            .skip(1)
                            .map(|o| o.map_or("", |m| m.as_str()))
                            .collect_vec()[..]
                        else {
                            todo!()
                        };
                        let condition = if xmas.is_empty() {
                            None
                        } else {
                            Some((xmas.as_bytes()[0], op.as_bytes()[0], value.parse().unwrap()))
                            // XXX
                        };
                        Rule {
                            condition,
                            target: target.to_string(),
                        }
                    })
                    .collect_vec();
                (name.clone(), Workflow { name, rules })
            })
        })
        .collect::<Result<HashMap<_, _>, _>>()
    }

    fn parse_parts<I, E>(spec: I) -> Result<Vec<Part>, E>
    where
        I: Iterator<Item = Result<String, E>>,
    {
        spec.map(|rs| {
            rs.map(|s| {
                let (_, [part]) = PART_PATTERN.captures(&s).unwrap().extract(); // XXX
                let categories = part
                    .split(',')
                    .map(|category| category.split('=').collect_tuple().unwrap())
                    .map(|(xmas, value)| (xmas.as_bytes()[0], value.parse().unwrap()))
                    .collect::<HashMap<_, _>>();
                Part(categories)
            })
        })
        .collect::<Result<Vec<_>, _>>()
    }

    fn parse(input: &mut dyn io::Read) -> BoxResult<(HashMap<String, Workflow>, Vec<Part>)> {
        let binding = io::BufReader::new(input)
            .lines()
            .group_by(|r| r.as_ref().map_or(false, |s| s.is_empty()));
        let mut iter = binding.into_iter();
        let workflows = Self::parse_workflows(iter.next().ok_or(AocError)?.1)?;
        if !iter.next().ok_or(AocError)?.0 {
            Err(AocError)?
        };
        let parts = Self::parse_parts(iter.next().ok_or(AocError)?.1)?;
        Ok((workflows, parts))
    }

    fn accept(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
        (0..)
            .try_fold("in", |name, _| {
                match workflows.get(name).unwrap().run(part) {
                    "A" => Err(true),
                    "R" => Err(false),
                    name => Ok(name),
                }
            })
            .unwrap_err()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (workflows, parts) = Self::parse(input)?;
        Ok(parts
            .into_iter()
            .filter(|part| Self::accept(&workflows, part))
            .map(|part| part.0.values().sum::<Output>())
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (workflows, _) = Self::parse(input)?;
        let combinations: HashMap<u8, (Output, Output)> = HashMap::from_iter(
            "xmas"
                .as_bytes()
                .iter()
                .map(|b| (*b, (1 as Output, 4000 as Output))),
        );
        (0..)
            .try_fold((vec![("in", combinations)], 0), |(states, sum), _| {
                let (new_states, sum) = states.into_iter().fold(
                    (vec![], sum),
                    |(mut new_states, sum), (state, mut combinations)| {
                        let rules = &workflows.get(state).unwrap().rules;
                        let (states, sum) = rules.iter().fold(
                            (vec![], sum),
                            |(mut new_states, sum), Rule { condition, target }| {
                                let combinations = if let Some((category, op, value)) = condition {
                                    let mut new_combinations = combinations.clone();
                                    let n = new_combinations.get_mut(category).unwrap();
                                    let o = combinations.get_mut(category).unwrap();
                                    match op {
                                        b'<' => {
                                            n.1 = *value - 1;
                                            o.0 = *value;
                                        }
                                        b'>' => {
                                            n.0 = *value + 1;
                                            o.1 = *value;
                                        }
                                        _ => (),
                                    };
                                    new_combinations
                                } else {
                                    combinations.clone()
                                };
                                let product = if target == "A" {
                                    combinations
                                        .values()
                                        .map(|(a, b)| (b - a + 1))
                                        .product::<Output>()
                                } else {
                                    0
                                };
                                if target != "A" && target != "R" {
                                    new_states.push((target.as_str(), combinations));
                                }
                                (new_states, sum + product)
                            },
                        );
                        new_states.extend(states);
                        (new_states, sum)
                    },
                );
                if new_states.is_empty() {
                    Err(Ok(sum))
                } else {
                    Ok((new_states, sum))
                }
            })
            .unwrap_err()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day19 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
            19114,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day19 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
            167409079868000,
        );
    }
}
