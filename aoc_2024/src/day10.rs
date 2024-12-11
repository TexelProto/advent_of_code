use ahash::{HashSet, HashSetExt};
use common::bit_set::BitSet;
use common::input::Input;
use nalgebra::{point, vector, Point2, SVector};
use std::io::BufRead;
use std::mem::swap;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct Map {
    start_points: Vec<Point2<usize>>,
    tiles: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Input<'_> for Map {
    type Error = Error;

    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut start_points = Vec::new();
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut line = String::new();
        loop {
            line.clear();
            if read.read_line(&mut line).unwrap_or(0) == 0 {
                break;
            }

            let line = line.trim();
            debug_assert!(line.chars().all(|c| char::is_ascii_digit(&c)));

            let mut row = Vec::new();

            let y = tiles.len();
            for (x, value) in line.bytes().enumerate() {
                let value = value - b'0';
                row.push(value);
                if value == 0 {
                    start_points.push(point!(x, y));
                }
            }

            width = row.len();
            tiles.push(row);
        }

        let height = tiles.len();

        Ok(Self {
            start_points,
            tiles,
            width,
            height,
        })
    }
}

fn set_visited(bits: &mut BitSet, map: &Map, point: Point2<usize>) {
    let index = point.y * map.width + point.x;
    bits.set(index);
}

fn is_visited(bits: &BitSet, map: &Map, point: Point2<usize>) -> bool {
    let index = point.y * map.width + point.x;
    bits.get(index)
}

pub fn try_advance_path(
    map: &Map,
    visited: &BitSet,
    from: Point2<usize>,
    offset: SVector<isize, 2>,
) -> Option<Point2<usize>> {
    let to_x = from.x.checked_add_signed(offset.x)?;
    let to_y = from.y.checked_add_signed(offset.y)?;
    let to = point![to_x, to_y];

    if to_x >= map.width || to_y >= map.height {
        return None;
    }

    if is_visited(visited, map, to) {
        return None;
    }

    let from_value = map.tiles[from.y][from.x];
    let to_value = map.tiles[to_y][to_x];

    if from_value + 1 != to_value {
        return None;
    }

    Some(to)
}

pub fn task1(map: Map) -> Result<i32, Error> {
    let mut reachable_peaks = 0;

    for &start in &map.start_points {
        let mut visited = BitSet::new(map.width * map.height);
        let mut open_set = vec![start];

        while let Some(point) = open_set.pop() {
            if is_visited(&visited, &map, point) {
                continue;
            }
            set_visited(&mut visited, &map, point);

            if map.tiles[point.y][point.x] == 9 {
                reachable_peaks += 1;
                continue;
            }

            if let Some(next) = try_advance_path(&map, &visited, point, vector![-1, 0]) {
                open_set.push(next);
            }

            if let Some(next) = try_advance_path(&map, &visited, point, vector![1, 0]) {
                open_set.push(next);
            }

            if let Some(next) = try_advance_path(&map, &visited, point, vector![0, -1]) {
                open_set.push(next);
            }

            if let Some(next) = try_advance_path(&map, &visited, point, vector![0, 1]) {
                open_set.push(next);
            }
        }
    }

    Ok(reachable_peaks)
}

fn write_to_neighbors(
    paths: &mut Vec<u32>,
    map: &Map,
    from: Point2<usize>,
    offset: SVector<isize, 2>,
) -> Option<Point2<usize>> {
    let to_x = from.x.checked_add_signed(offset.x)?;
    let to_y = from.y.checked_add_signed(offset.y)?;
    let to = point![to_x, to_y];

    if to_x >= map.width || to_y >= map.height {
        return None;
    }

    let from_value = map.tiles[from.y][from.x];
    let to_value = map.tiles[to_y][to_x];

    if from_value + 1 != to_value {
        return None;
    }

    let from_index = from.y * map.width + from.x;
    let to_index = to.y * map.width + to.x;
    paths[to_index] += paths[from_index];

    Some(to)
}

pub fn task2(map: Map) -> Result<u32, Error> {
    let mut paths_to = vec![0; map.width * map.height];
    for point in &map.start_points {
        let index = map.width * point.y + point.x;
        paths_to[index] = 1;
    }

    let mut read_set = map.start_points.iter().cloned().collect::<HashSet<_>>();
    let mut write_set = HashSet::new();

    for _scan_value in 1u8..10 {
        for &point in &read_set {
            if let Some(next) = write_to_neighbors(&mut paths_to, &map, point, vector![-1, 0]) {
                write_set.insert(next);
            }
            if let Some(next) = write_to_neighbors(&mut paths_to, &map, point, vector![1, 0]) {
                write_set.insert(next);
            }
            if let Some(next) = write_to_neighbors(&mut paths_to, &map, point, vector![0, -1]) {
                write_set.insert(next);
            }
            if let Some(next) = write_to_neighbors(&mut paths_to, &map, point, vector![0, 1]) {
                write_set.insert(next);
            }
        }

        read_set.clear();
        swap(&mut read_set, &mut write_set);
    }

    let mut total = 0;
    // for (y, row) in map.tiles.iter().enumerate() {
    //     for (x, value) in row.iter().enumerate() {
    //         if *value != 9 {
    //             continue;
    //         }
    //
    //         let index = map.width * y + x;
    //         total += paths_to[index];
    //     }
    // }
    for point in read_set {
        let index = point.y * map.width + point.x;
        total += paths_to[index];
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 36);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 81);
    }
}
