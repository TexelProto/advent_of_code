use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Line was missing split char '{0}'")]
    MissingSplit(char),
}

pub struct RangePair(RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_range(s: &str) -> Result<RangeInclusive<usize>, Error> {
    let split = s.find('-').ok_or(Error::MissingSplit('-'))?;
    let (a, b) = s.split_at(split);
    let low = usize::from_str(a)?;
    let hi = usize::from_str(&b[1..])?;
    Ok(RangeInclusive::new(low, hi))
}

impl FromStr for RangePair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.find(',').ok_or_else(|| Error::MissingSplit(','))?;
        let (a, b) = s.split_at(split);
        let a = parse_range(a)?;
        let b = parse_range(&b[1..])?;
        Ok(RangePair(a, b))
    }
}

fn ranges_containing(ranges: &RangePair) -> bool {
    ranges.0.contains(ranges.1.start()) 
        && ranges.0.contains(ranges.1.end())
        || ranges.1.contains(ranges.0.start()) 
        && ranges.1.contains(ranges.0.end())
}

fn ranges_overlapping(ranges: &RangePair) -> bool {
    ranges.0.contains(ranges.1.start())
        || ranges.0.contains(ranges.1.end())
        || ranges.1.contains(ranges.0.start())
        || ranges.1.contains(ranges.0.end())
}

pub fn task1(input: Linewise<RangePair>) -> Result<u64, Error> {
    let mut count = 0;
    for_input!(input, |pair| {
        if ranges_containing(&pair) {
            count += 1;
        }
    });

    Ok(count)
}

pub fn task2(input: Linewise<RangePair>) -> Result<u64, Error> {
    let mut count = 0;
    for_input!(input, |pair| {
        if ranges_overlapping(&pair) {
            count += 1;
        }
    });

    Ok(count)
}
