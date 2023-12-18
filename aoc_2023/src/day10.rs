use std::io::BufRead;
use bitflags::bitflags;
use common::geometry_2d::{Direction, Point};
use common::input::{Charwise, Input, Linewise};
use common::iter_ext::TryIterator;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid character in map {0}")]
    InvalidMapChar(char)
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Connections: u8 {
        const TOP = 1;
        const LEFT = 2;
        const BOTTOM = 4;
        const RIGHT = 8;
        const START = 16;
    }
}

impl Connections {
    fn is_corner(self) -> bool {
        self.intersects(Self::START) ||
            self.intersects(Self::TOP) && !self.intersects(Self::BOTTOM) ||
            self.intersects(Self::BOTTOM) && !self.intersects(Self::TOP) ||
            self.intersects(Self::LEFT) && !self.intersects(Self::RIGHT) ||
            self.intersects(Self::RIGHT) && !self.intersects(Self::LEFT)
    }
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Vec<Connections>>,
}

impl Map {
    fn get(&self, p: Point) -> Connections {
        self.tiles[p.y as usize][p.x as usize]
    }
    fn find_start(&self) -> Point {
        for y in 0..self.tiles.len() {
            let line = &self.tiles[y];
            for x in 0..line.len() {
                if self.tiles[y][x] == Connections::START {
                    return Point { x: x as u32, y: y as u32 };
                }
            }
        }
        panic!("Failed to find start point")
    }
}

impl Input<'_> for Map {
    type Error = Error;
    fn parse<R: BufRead>(read: R) -> Result<Self, Self::Error> {
        let tiles = Linewise::<Charwise<char>>::parse(read).unwrap()
            .map(|line| {
                line.unwrap().map(|c| {
                    match c.unwrap() {
                        '|' => Ok(Connections::TOP | Connections::BOTTOM),
                        '-' => Ok(Connections::LEFT | Connections::RIGHT),
                        'L' => Ok(Connections::TOP | Connections::RIGHT),
                        'J' => Ok(Connections::LEFT | Connections::TOP),
                        '7' => Ok(Connections::LEFT | Connections::BOTTOM),
                        'F' => Ok(Connections::BOTTOM | Connections::RIGHT),
                        '.' => Ok(Connections::empty()),
                        'S' => Ok(Connections::START),
                        c => Err(Error::InvalidMapChar(c))
                    }
                }).try_collect2::<Vec<_>>()
            }).try_collect2::<Vec<_>>()?;
        Ok(Self { tiles })
    }
}

const DIRECTION_CONNECTIONS: [(Direction, Connections); 4] = [
    (Direction::UP, Connections::BOTTOM),
    (Direction::DOWN, Connections::TOP),
    (Direction::LEFT, Connections::RIGHT),
    (Direction::RIGHT, Connections::LEFT),
];

#[derive(Debug)]
struct Walker {
    last_dir: Direction,
    pos: Point,
}

impl Walker {
    fn step(&mut self, map: &Map) {
        let local = map.get(self.pos);
        let (_, enter_connection) = DIRECTION_CONNECTIONS.into_iter()
            .filter(|(d, _)| *d == self.last_dir)
            .next().unwrap();

        let next = local & !enter_connection;
        let (dir, _) = DIRECTION_CONNECTIONS.into_iter()
            .filter(move |(_, c)| *c == next)
            .next().unwrap();

        self.last_dir = -dir;
        self.pos -= dir;
    }
}

pub fn task1(map: Map) -> Result<i32, Error> {
    let start = map.find_start();

    let mut walkers = vec![];

    for (d, c) in DIRECTION_CONNECTIONS {
        let neighbor = start + d;

        if map.get(neighbor).intersects(c) == false {
            continue;
        }

        walkers.push(Walker {
            last_dir: d,
            pos: neighbor,
        })
    }

    let mut steps = 1;
    loop {
        for w in &mut walkers {
            w.step(&map)
        }
        steps += 1;

        if walkers[0].pos == walkers[1].pos {
            break;
        }
    }

    Ok(steps)
}

pub fn task2(map: Map) -> Result<i32, Error> {
    let start = map.find_start();

    // create a walker going in any direction
    let mut walker = DIRECTION_CONNECTIONS.into_iter().filter_map(|(d, c)| {
        let neighbor = start.offset(d)?;

        if map.get(neighbor).intersects(c) == false {
            return None;
        }

        Some(Walker {
            last_dir: d,
            pos: neighbor,
        })
    }).next().unwrap();

    // collect all corners and count the total number of points
    let mut boundary_points = 0;
    let mut points = vec![];
    loop {
        if map.get(walker.pos).is_corner() {
            points.push(walker.pos);
        }
        boundary_points += 1;
        if walker.pos == start { break; }

        walker.step(&map);
    }

    // use shoelace formular to calculate total area
    let len = points.len();
    let mut area = 0;
    for i in 0..len {
        let j = (i + 1) % len;
        area += (points[i].x as i32 * points[j].y as i32) - (points[j].x as i32 * points[i].y as i32);
    }

    // use picks theorem to calculate the number of interior points
    let interior = (area - boundary_points + 1) / 2 + 1;
    Ok(interior)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
.....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(b"\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".as_slice());
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 10);
    }
}
