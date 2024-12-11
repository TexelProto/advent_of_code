use ahash::{HashSet, HashSetExt};
use common::bit_set::BitSet;
use common::input::Input;
use nalgebra::{point, vector, Point2, SVector};
use std::io::BufRead;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing guard token '^' in input")]
    MissingGuard,
}

pub struct Map {
    obstacles: BitSet,
    width: usize,
    height: usize,
    guard_start: Point2<i32>,
}

impl Input<'_> for Map {
    type Error = Error;

    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut obstacles = Vec::new();
        let mut width = 0;
        let mut y = 0;
        let mut guard_start = None;
        let mut line = String::new();
        loop {
            line.clear();
            match read.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                _ => {}
            };

            let line = line.trim();
            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    obstacles.push((x, y));
                } else if c == b'^' {
                    guard_start = Some(point!(x as i32, y));
                }
            }

            width = line.len();
            y += 1;
        }

        let height = y as usize;
        let mut obstacle_bits = BitSet::new(width * height);
        for (x, y) in obstacles {
            let index = (y as usize * width) + x;
            obstacle_bits.set(index)
        }

        let guard_start = guard_start.ok_or(Error::MissingGuard)?;
        Ok(Self {
            obstacles: obstacle_bits,
            width,
            height,
            guard_start,
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn offset(self) -> SVector<i32, 2> {
        match self {
            Direction::Up => vector![0, -1],
            Direction::Right => vector![1, 0],
            Direction::Down => vector![0, 1],
            Direction::Left => vector![-1, 0],
        }
    }
}

pub fn task1(input: Map) -> Result<usize, Error> {
    let mut states = HashSet::new();

    let did_loop = get_path(&input, &mut states);
    assert_eq!(did_loop, false, "Found infinite loop in input");

    let positions = states
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();
    Ok(positions.len())
}

fn get_path(map: &Map, cache: &mut HashSet<(Point2<i32>, Direction)>) -> bool {
    debug_assert!(cache.is_empty());

    let mut position = map.guard_start;
    let mut direction = Direction::Up;
    loop {
        match cast_ray(map, position, direction) {
            Ok(end) => {
                if insert_ray_points(cache, position, end, direction) {
                    return true;
                }

                position = end;
                direction = direction.next();
            }
            Err(end) => {
                return insert_ray_points(cache, position, end, direction);
            }
        }
    }
}

fn insert_ray_points(
    cache: &mut HashSet<(Point2<i32>, Direction)>,
    start: Point2<i32>,
    end: Point2<i32>,
    direction: Direction,
) -> bool {
    let offset = end - start;
    let step = vector![offset.x.signum(), offset.y.signum()];

    let mut insert = start;
    loop {
        if cache.insert((insert, direction)) == false {
            return true;
        }
        if insert == end {
            break;
        }
        insert += step;
    }
    false
}

fn cast_ray(
    map: &Map,
    mut pos: Point2<i32>,
    direction: Direction,
) -> Result<Point2<i32>, Point2<i32>> {
    let offset = direction.offset();
    loop {
        let next = pos + offset;

        if next.x < 0 || next.y < 0 {
            return Err(pos);
        }

        if next.x as usize >= map.width || next.y as usize >= map.height {
            return Err(pos);
        }

        let index = next.y as usize * map.width + next.x as usize;
        if map.obstacles.get(index) {
            return Ok(pos);
        }

        pos = next;
    }
}

pub fn task2(mut map: Map) -> Result<i32, Error> {
    let mut init_path = HashSet::new();

    let init_loop = get_path(&map, &mut init_path);
    assert_eq!(init_loop, false, "Found infinite loop in initial path");

    let positions = init_path
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    let mut loops = 0;
    let mut loop_cache = HashSet::new();
    for block_point in positions {
        let block_index = block_point.y as usize * map.width + block_point.x as usize;
        map.obstacles.set(block_index);
        loop_cache.clear();
        let did_loop = get_path(&map, &mut loop_cache);
        map.obstacles.unset(block_index);

        if did_loop {
            loops += 1;
        }
    }

    Ok(loops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 41);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 0);
    }
}
