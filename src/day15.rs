use crate::day::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Day15 {}

type Output = usize;

impl Day for Day15 {
    fn tag(&self) -> &str {
        "15"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new("^(.*)([-=])(.*)$").unwrap();
}

impl Day15 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Vec<String>> {
        let s = io::BufReader::new(input).lines().next().unwrap()?;
        Ok(s.split(',').map(ToString::to_string).collect())
    }

    fn hash(v: &[u8]) -> Output {
        v.iter()
            .fold(0 as Output, |h, &b| ((h + b as Output) * 17 % 256))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let v = Self::parse(input)?;
        Ok(v.into_iter()
            .map(|s| Self::hash(&s.bytes().collect_vec()))
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let v = Self::parse(input)?;
        let mut boxes: HashMap<Output, Vec<(String, Output)>> = HashMap::new();
        v.into_iter()
            .map(|s| {
                let (_, [label, op, arg]) = PATTERN.captures(&s).ok_or(AocError)?.extract();
                let hash = Self::hash(&label.bytes().collect_vec());
                match op.chars().next() {
                    Some('-') => {
                        if let Some(v) = boxes.get_mut(&hash) {
                            v.retain(|(s, _)| s != label);
                        }
                        Ok(())
                    }
                    Some('=') => {
                        let value = arg.parse::<Output>()?;
                        boxes
                            .entry(hash)
                            .and_modify(|v| {
                                if let Some(i) = v.iter().position(|(s, _)| s == label) {
                                    v[i].1 = value;
                                } else {
                                    v.push((label.to_string(), value));
                                };
                            })
                            .or_insert(vec![(label.to_string(), value)]);
                        Ok(())
                    }
                    _ => Err(AocError.into()),
                }
            })
            .collect::<BoxResult<Vec<_>>>()?;
        Ok(boxes
            .iter()
            .map(|(k, v)| {
                (k + 1)
                    * v.iter()
                        .enumerate()
                        .map(|(i, (_, lens))| (i + 1) * *lens)
                        .sum::<Output>()
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day15 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("HASH", 52);
        test1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day15 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145);
    }
}
