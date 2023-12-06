use crate::day::*;

pub struct Day06 {}

type Output = usize;

impl Day for Day06 {
    fn tag(&self) -> &str {
        "06"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day06 {
    fn parse<F>(input: &mut dyn io::Read, handle: F) -> BoxResult<Vec<Vec<Output>>>
    where
        F: Fn(String) -> BoxResult<Vec<Output>>,
    {
        io::BufReader::new(input)
            .lines()
            .map(|rs| rs.map_err(|e| e.into()).and_then(|s| handle(s)))
            .collect::<BoxResult<Vec<_>>>()
    }

    fn process(spec: Vec<Vec<Output>>) -> BoxResult<Output> {
        let [ref time, ref distance] = spec[..] else {
            return Err(AocError.into());
        };
        let races = time.iter().zip(distance.iter()).collect_vec();
        Ok(races
            .into_iter()
            .map(|(&duration, &record)| {
                (1..duration)
                    .map(|press| (duration - press) * press)
                    .filter(|&distance| distance > record)
                    .count()
            })
            .product::<Output>())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(Self::parse(input, |s| {
            s.split_whitespace()
                .skip(1)
                .map(|d| d.parse::<Output>().map_err(Into::into))
                .collect::<BoxResult<Vec<_>>>()
        })?)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(Self::parse(input, |s| {
            s.split(':')
                .skip(1)
                .map(|d| {
                    let mut d = d.to_string();
                    d.retain(|c| c.is_ascii_digit());
                    d.parse::<Output>().map_err(Into::into)
                })
                .collect::<BoxResult<Vec<_>>>()
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day06 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Time:      7  15   30
Distance:  9  40  200",
            288,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day06 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Time:      7  15   30
Distance:  9  40  200",
            71503,
        );
    }
}
