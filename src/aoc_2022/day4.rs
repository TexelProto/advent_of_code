use std::ops::RangeInclusive;

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (first, second) = input.split_once('-').unwrap();
    let start = usize::from_str_radix(first.trim(), 10).unwrap();
    let end = usize::from_str_radix(second.trim(), 10).unwrap();

    RangeInclusive::new(start, end)
}

fn parse_ranges(input: &str) -> [RangeInclusive<usize>; 2] {
    let (first, second) = input.split_once(',').unwrap();
    [parse_range(first), parse_range(second)]
}

fn ranges_containing(ranges: &[RangeInclusive<usize>; 2]) -> bool {
    ranges[0].contains(ranges[1].start()) && ranges[0].contains(ranges[1].end())
        || ranges[1].contains(ranges[0].start()) && ranges[1].contains(ranges[0].end())
}

fn ranges_overlapping(ranges: &[RangeInclusive<usize>; 2]) -> bool {
    ranges[0].contains(ranges[1].start())
        || ranges[0].contains(ranges[1].end())
        || ranges[1].contains(ranges[0].start())
        || ranges[1].contains(ranges[0].end())
}

pub fn task1(input: String) {
    let count = input
            .lines()
            .map(parse_ranges)
            .filter(ranges_containing)
            .count();

        println!("Overlaps: {}", count)
    }
    pub fn task2(input: String) {
        let count = input
            .lines()
            .map(parse_ranges)
            .filter(ranges_overlapping)
            .count();

        println!("Overlaps: {}", count)
    }
