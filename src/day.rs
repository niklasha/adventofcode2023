pub use itertools::Itertools;
pub use std::error;
use std::fmt;
pub use std::io;
pub use std::io::BufRead;

pub type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub struct AocError;

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected error")
    }
}

impl error::Error for AocError {}

pub trait Day {
    fn tag(&self) -> &str;
    fn part1(&self, _input: &dyn Fn() -> Box<dyn io::Read>) {}
    fn part2(&self, _input: &dyn Fn() -> Box<dyn io::Read>) {}
}

pub struct Utils;

#[allow(unused)]
impl Utils {
    pub fn numbers(input: &mut dyn io::Read) -> impl Iterator<Item = BoxResult<i64>> + '_ {
        let lines = io::BufReader::new(input).lines();
        lines.map(|rs| {
            rs.map_err(|e| e.into())
                .and_then(|s| s.parse::<i64>().map_err(|e| e.into()))
        })
    }

    pub fn numbers2(
        input: &mut dyn io::Read,
        radix: u32,
    ) -> impl Iterator<Item = BoxResult<i64>> + '_ {
        let lines = io::BufReader::new(input).lines();
        lines.map(move |rs| {
            rs.map_err(|e| e.into())
                .and_then(move |s| i64::from_str_radix(&s, radix).map_err(|e| e.into()))
        })
    }

    pub fn byte_matrix(input: &mut dyn io::Read) -> BoxResult<Vec<Vec<u8>>> {
        io::BufReader::new(input)
            .split(b'\n')
            .map(|rv| rv.map_err(|e| e.into()))
            .collect()
    }
}
