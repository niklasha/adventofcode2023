use crate::day::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Day05 {}

type Output = usize;

impl Day for Day05 {
    fn tag(&self) -> &str {
        "05"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

lazy_static! {
    static ref SEEDS_REGEX: Regex = Regex::new("^seeds: (.*)$").unwrap();
    static ref MAP_REGEX: Regex = Regex::new("^(.*) map:$").unwrap();
    static ref MAPPING_REGEX: Regex = Regex::new("^(.*) (.*) (.*)$").unwrap();
}

enum Mode {
    Seeds,
    Map,
    Mapping,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Output>,
    mappings: HashMap<String, Vec<(Output, Output, Output)>>,
}

impl Day05 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Almanac> {
        Ok(io::BufReader::new(input)
            .lines()
            .enumerate()
            .try_fold(
                (
                    Mode::Seeds,
                    Almanac {
                        seeds: Vec::new(),
                        mappings: HashMap::new(),
                    },
                    None,
                ),
                |(mode, mut almanac, map), (_, rs)| {
                    rs.map_err(|e| e.into()).and_then(|s| match mode {
                        Mode::Seeds => {
                            let (_, [seeds]) = SEEDS_REGEX.captures(&s).ok_or(AocError)?.extract();
                            almanac.seeds = seeds
                                .split_whitespace()
                                .map(|seed| seed.parse::<Output>().map_err(Into::into))
                                .collect::<BoxResult<_>>()?;
                            Ok((Mode::Map, almanac, map))
                        }
                        Mode::Map => Ok(if s.is_empty() {
                            (mode, almanac, None)
                        } else {
                            let (_, [map]) = MAP_REGEX.captures(&s).ok_or(AocError)?.extract();
                            (Mode::Mapping, almanac, Some(map.to_string()))
                        }),
                        Mode::Mapping => BoxResult::Ok({
                            let map = map.ok_or(AocError)?;
                            if s.is_empty() {
                                let mappings = almanac.mappings.get_mut(&map).ok_or(AocError)?;
                                mappings.sort_by_key(|mapping| mapping.1);
                                (Mode::Map, almanac, None)
                            } else {
                                let (_, [dst, src, len]) =
                                    MAPPING_REGEX.captures(&s).ok_or(AocError)?.extract();
                                let mapping = (
                                    dst.parse::<Output>()?,
                                    src.parse::<Output>()?,
                                    len.parse::<Output>()?,
                                );
                                almanac
                                    .mappings
                                    .entry(map.to_owned())
                                    .and_modify(|mappings| mappings.push(mapping))
                                    .or_insert(vec![mapping]);
                                (mode, almanac, Some(map))
                            }
                        }),
                    })
                },
            )?
            .1)
    }

    const MAPS: &'static [&'static str] = &[
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let almanac = Self::parse(input)?;
        Self::MAPS
            .iter()
            .try_fold(almanac.seeds, |locations, &map| {
                let mappings = almanac.mappings.get(map).ok_or(AocError)?;
                BoxResult::Ok(
                    locations
                        .into_iter()
                        .map(|location| {
                            mappings
                                .iter()
                                .find(|&&(_, src, len)| (src..(src + len)).contains(&location))
                                .map_or(location, |&mapping| mapping.0 + (location - mapping.1))
                        })
                        .collect(),
                )
            })?
            .into_iter()
            .min()
            .ok_or(AocError.into())
    }

    fn process_location(
        location: Vec<Output>,
        mappings: &[(Output, Output, Output)],
    ) -> Vec<Vec<Output>> {
        let (start, len) = (location[0], location[1]);
        let max = start + len;
        let (mut new_locations, min) = mappings
            .iter()
            .try_fold(
                (Vec::new(), start),
                |(mut new_locations, min), &(dst, src, mapping_len)| {
                    let mapping_max = src + mapping_len;
                    if max < src {
                        return Err((new_locations, min));
                    }
                    if min < src {
                        new_locations.push(vec![min, src - min]);
                    }
                    if min < mapping_max {
                        let new_min = Output::max(min, src);
                        let new_max = Output::min(max, mapping_max);
                        new_locations.push(vec![dst + (new_min - src), new_max - new_min]);
                        Ok((new_locations, mapping_max))
                    } else {
                        Ok((new_locations, min))
                    }
                },
            )
            .unwrap_or_else(|err| err);
        if min < max {
            new_locations.push(vec![min, max - min]);
        }
        new_locations
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let almanac = Self::parse(input)?;
        let locations = almanac
            .seeds
            .chunks(2)
            .map(|slice| slice.to_vec())
            .collect::<Vec<_>>();
        Self::MAPS
            .iter()
            .try_fold(locations, |current_locations, &map| {
                let mappings = almanac.mappings.get(map).ok_or(AocError)?;
                BoxResult::Ok(
                    current_locations
                        .into_iter()
                        .flat_map(|location| Self::process_location(location, mappings))
                        .collect(),
                )
            })?
            .into_iter()
            .map(|location| location[0])
            .min()
            .ok_or(AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day05 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            35,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day05 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            46,
        );
    }
}
