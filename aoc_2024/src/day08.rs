use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use common::input::Input;
use nalgebra::{point, Point2};
use std::io::BufRead;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<u8, Vec<Point2<i32>>>,
}

impl Input<'_> for Map {
    type Error = Error;

    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut antennas = HashMap::<u8, Vec<Point2<i32>>>::new();
        let mut y = 0;
        let mut line = String::new();
        let mut width = 0;
        loop {
            line.clear();
            if read.read_line(&mut line).unwrap_or(0) == 0 {
                break;
            }

            let line = line.trim();
            width = line.len();

            for (x, c) in line.as_bytes().iter().cloned().enumerate() {
                if c == b'.' {
                    continue;
                }

                let list = antennas.entry(c).or_default();
                list.push(point![x as i32, y])
            }

            y += 1;
        }
        let height = y as usize;

        Ok(Self {
            width,
            height,
            antennas,
        })
    }
}

pub fn task1(map: Map) -> Result<usize, Error> {
    let mut antinodes = HashSet::new();
    for group in map.antennas.values() {
        let count = group.len();
        for i in 1..count {
            let first = group[i];
            for j in 0..i {
                let second = group[j];
                try_create_antinode(&map, &mut antinodes, first, second);
                try_create_antinode(&map, &mut antinodes, second, first);
            }
        }
    }

    let count = antinodes.len();
    Ok(count)
}

fn try_create_antinode(
    map: &Map,
    nodes: &mut HashSet<Point2<i32>>,
    a: Point2<i32>,
    b: Point2<i32>,
) {
    let offset = a - b;
    let next = a + offset;

    if next.x < 0 || next.y < 0 {
        return;
    }

    if next.x as usize >= map.width || next.y as usize >= map.height {
        return;
    }

    nodes.insert(next);
}

pub fn task2(map: Map) -> Result<usize, Error> {
    let mut antinodes = HashSet::new();
    for group in map.antennas.values() {
        let count = group.len();
        for i in 0..count {
            let first = group[i];
            for &other in &group[(i + 1)..] {
                try_create_antinode_repeating(&map, &mut antinodes, first, other);
                try_create_antinode_repeating(&map, &mut antinodes, other, first);
            }
        }
    }

    let count = antinodes.len();
    Ok(count)
}

fn try_create_antinode_repeating(
    map: &Map,
    nodes: &mut HashSet<Point2<i32>>,
    a: Point2<i32>,
    b: Point2<i32>,
) {
    nodes.insert(a);
    nodes.insert(b);

    let mut point = a;
    let offset = a - b;
    loop {
        let next = point + offset;

        if next.x < 0 || next.y < 0 {
            return;
        }

        if next.x as usize >= map.width || next.y as usize >= map.height {
            return;
        }

        nodes.insert(next);
        point = next;
    }
}

#[allow(unused)]
fn print_result(map: &Map, antinodes: &HashSet<Point2<i32>>) {
    let mut lines = Vec::with_capacity(map.height);
    for y in 0..map.height {
        let y = y as i32;
        let mut line = String::new();
        for x in 0..map.width {
            let x = x as i32;
            let c = if antinodes.contains(&point![x, y]) {
                '#'
            } else {
                '.'
            };
            line.push(c);
        }
        lines.push(line);
    }

    for (&key, points) in &map.antennas {
        for &point in points {
            let row = unsafe { lines[point.y as usize].as_bytes_mut() };
            row[point.x as usize] = key;
        }
    }

    for line in lines {
        println!("{line}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 14);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 34);
    }
}
