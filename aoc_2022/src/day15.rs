use std::{collections::HashSet, str::FromStr};

use common::input::Linewise;

pattern_parse::parse_fn!(parse_line_parts, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}");

struct Point(i32, i32);

struct Sensor {
    pos: Point,
    range: u32,
}

pub struct SensorPoint(Sensor, Point);

impl SensorPoint {
    fn into_inner(self) -> (Sensor, Point) {
        (self.0, self.1)
    }
}

impl FromStr for SensorPoint {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sx, sy, bx, by) = parse_line_parts(s)?;
        let s = Point(sx, sy);
        let b = Point(bx, by);
        let range = distance(&s, &b);
        Ok(Self(Sensor{pos: s, range}, b))
    }
}

fn distance(a: &Point, b: &Point) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn task1(input: Linewise<SensorPoint>) -> Result<usize, pattern_parse::ParseError> {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    common::for_input!(input, |line| {
        let (s,b) = line.into_inner();
        sensors.push(s);
        beacons.push(b);
    });

    const CHECK_Y: i32 = 2000000;

    let mut coverage = HashSet::new();

    for sensor in sensors {
        let y_dist = sensor.pos.1.abs_diff(CHECK_Y);

        // line is completely out of range
        if y_dist > sensor.range {
            continue;
        }

        // assuming sensor had range=5 and y_dist=3
        // the possible x positions would be [-2, -1, 0, 1, 2]
        let x_range = (sensor.range - y_dist) as i32;
        for x_offset in -x_range..=x_range {
            let x = sensor.pos.0 + x_offset;
            coverage.insert(x);
        }
    }

    for b in beacons {
        if b.1 == CHECK_Y {
            coverage.remove(&b.0);
        }
    }

    Ok(coverage.len())
}

pub fn task2(input: Linewise<SensorPoint>) -> Result<u64, pattern_parse::ParseError> {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    common::for_input!(input, |line| {
        let (s,b) = line.into_inner();
        sensors.push(s);
        beacons.push(b);
    });

    const MAX_DIST: i32 = 4_000_000;

    for y in 0..MAX_DIST {
        let mut coverage = Vec::new();

        for sensor in &sensors {
            let y_dist = sensor.pos.1.abs_diff(y);

            // line is completely out of range
            if y_dist > sensor.range {
                continue;
            }

            // assuming sensor had range=5 and y_dist=3
            // the possible x positions would be [-2, -1, 0, 1, 2]
            let x_range = (sensor.range - y_dist) as i32;
            let min = sensor.pos.0 - x_range;
            let max = sensor.pos.0 + x_range;
            coverage.push(min..=max);
        }

        let mut x = 0;
        loop {
            if x >= MAX_DIST {
                break;
            }

            match coverage.iter().find(|r| r.contains(&x)) {
                Some(range) => {
                    x = range.end() + 1;
                }
                None => {
                    if sensors.iter().all(|s| distance(&s.pos, &Point(x,y)) > s.range) {
                        let x = x as u64;
                        let y = y as u64;
                        let result = x * 4000000 + y;
                        return Ok(result);
                    }

                    x += 1;
                },
            }
        }
    }
    panic!("NONE FOUND");
}
