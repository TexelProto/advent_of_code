use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::cmp::{min, Ordering};
use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing delimiter '{0}' in input line")]
    MissingDelimiter(char),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    PatternParse(#[from] pattern_parse::ParseError),
}

#[derive(Debug)]
pub struct Map {
    ranges: Vec<RangeLine>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct RangeLine {
    range: Range<u64>,
    offset: i64,
}

impl PartialOrd for RangeLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeLine {
    fn cmp(&self, other: &RangeLine) -> Ordering {
        Ord::cmp(&self.range.start, &other.range.start)
    }
}

impl RangeLine {
    fn remap_forward(&self, i: u64) -> u64 {
        if self.range.contains(&i) {
            i.checked_add_signed(self.offset).expect("int overflow")
        } else {
            i
        }
    }
    fn remap_range_forward(&self, start: u64, range: u64) -> (u64, u64) {
        let dest = if self.range.contains(&start) {
            start.checked_add_signed(self.offset).expect("int overflow")
        } else {
            start
        };
        (dest, range)
    }
}

pattern_parse::parse_fn!(parse_range, "{u64} {u64} {u64}");

fn parse_input(input: &mut Linewise<String>) -> Result<(Vec<u64>, Vec<Map>), Error> {
    let seed_line = input.next().expect("Failed to read seed-line").unwrap();
    let (_, seed_str) = seed_line
        .split_once(':')
        .ok_or(Error::MissingDelimiter(':'))?;
    let x = seed_str
        .split_ascii_whitespace()
        .map(u64::from_str)
        .try_collect2::<Vec<_>>()?;

    // blank line
    let _ = input.next();
    let mut maps = vec![];

    'outer: loop {
        // header line
        let _ = input.next();

        maps.push(Map { ranges: vec![] });

        let map = maps.last_mut().unwrap();

        loop {
            match input.next() {
                None => break 'outer,
                Some(s) => {
                    let s = s.unwrap();
                    if s.len() == 0 {
                        break;
                    }
                    let (dest_start, src_start, len) = parse_range(&s)?;
                    let range = src_start..(src_start + len);
                    let offset = dest_start as i64 - src_start as i64;
                    map.ranges.push(RangeLine { range, offset });
                }
            }
        }
    }
    Ok((x, maps))
}

pub fn task1(mut input: Linewise<String>) -> Result<u64, Error> {
    let (seeds, mut maps) = parse_input(&mut input)?;

    maps.iter_mut()
        .for_each(|m| m.ranges.sort_by(|a, b| a.range.end.cmp(&b.range.end)));

    let mut min_location = u64::MAX;

    for location in seeds {
        let location = translate_point(&mut maps, location);
        min_location = min(min_location, location);
    }

    Ok(min_location)
}

fn translate_point(maps: &Vec<Map>, mut location: u64) -> u64 {
    for map in maps.iter() {
        // either the search returns the index of the range that ends at 'location' (inclusive)
        // or the the first range ending after the location
        let range = map
            .ranges
            .binary_search_by(move |r| r.range.end.cmp(&location))
            .ok()
            .and_then(|i| map.ranges.get(i))
            .cloned()
            .unwrap_or_default();

        // if were nearby any range try to remap
        location = range.remap_forward(location);
    }
    location
}

pub fn task2(mut input: Linewise<String>) -> Result<u64, Error> {
    let (seeds, mut maps) = parse_input(&mut input)?;
    maps.iter_mut()
        .for_each(|m| m.ranges.sort_by(|a, b| a.range.end.cmp(&b.range.end)));
    let mut min_location = u64::MAX;

    for range in seeds.chunks_exact(2) {
        let &[start, len] = range else { unreachable!() };
        let mut ranges = vec![(start, len)];
        for map in &maps {
            ranges = translate_ranges(map, ranges);
        }
        let range_min = ranges.iter().map(|r| r.0).min().unwrap();
        min_location = min(min_location, range_min);
    }

    Ok(min_location)
}

fn translate_ranges(map: &Map, source: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut dest = vec![];
    for (mut cursor, len) in source.iter().cloned() {
        let end = cursor + len;
        while cursor < end {
            let index = map
                .ranges
                .binary_search_by(move |r| (r.range.end - 1).cmp(&cursor))
                .unwrap_or_else(|i| i);

            let mapped_range = match map.ranges.get(index) {
                Some(line) if line.range.contains(&cursor) => {
                    let mapped_len = min(line.range.end - cursor, end - cursor);
                    line.remap_range_forward(cursor, mapped_len)
                }
                Some(next_line) => {
                    let end = min(end, next_line.range.start);
                    let len = end - cursor;
                    (cursor, len)
                }
                None => (cursor, end - cursor),
            };

            debug_assert_ne!(mapped_range.1, 0);
            dest.push(mapped_range);
            cursor += mapped_range.1;
        }
    }

    dest
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 35);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 46);
    }
}
