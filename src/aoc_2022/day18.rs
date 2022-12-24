use std::{collections::HashSet, str::FromStr, num::ParseIntError};

use crate::{common::pathfinding as pf, input::Linewise};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing line {0}")]
    MissingLine(usize),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError)
}

struct World {
    points: Vec<Point>,
    keys: HashSet<u32>,
}

impl World {
    fn new(points: Vec<Point>) -> Self {
        let keys = points.iter().map(Self::compute_key).collect::<HashSet<_>>();
        Self { points, keys }
    }

    fn compute_key(p: &Point) -> u32 {
        (p.0 as u32) << 16 | (p.1 as u32) << 8 | (p.2 as u32)
    }

    fn contains(&self, p: &Point) -> bool {
        self.keys.contains(&Self::compute_key(p))
    }
}

impl pf::World<'_> for World {
    type Index = Point;
    type Neighbors = [Point; 6];
    fn get_neighbors(&self, origin: &Self::Index) -> Self::Neighbors {
        ALL_DIRECTIONS.map(|dir| offset(origin, dir))
    }
}

struct Agent;

impl pf::Agent<'_, World> for Agent {
    type Score = u64;

    fn get_cost(&self, world: &World, _start: &Point, destination: &Point) -> Option<Self::Score> {
        if world.contains(destination) {
            None // dont walk into walls
        } else {
            Some(1) // all movement is on a equidistant grid
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point(i8, i8, i8);

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split(",");

        let line = lines.next().ok_or(Error::MissingLine(0))?;
        let x = i8::from_str(line)?;
        let line = lines.next().ok_or(Error::MissingLine(1))?;
        let y = i8::from_str(line)?;
        let line = lines.next().ok_or(Error::MissingLine(2))?;
        let z = i8::from_str(line)?;

        Ok(Point(x,y,z))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Back,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::Forward,
    Direction::Back,
];

fn offset(p: &Point, dir: Direction) -> Point {
    match dir {
        Direction::Left => Point(p.0 - 1, p.1, p.2),
        Direction::Right => Point(p.0 + 1, p.1, p.2),
        Direction::Down =>Point (p.0, p.1 - 1, p.2),
        Direction::Up =>Point (p.0, p.1 + 1, p.2),
        Direction::Back =>Point (p.0, p.1, p.2 - 1),
        Direction::Forward => Point (p.0, p.1, p.2 + 1),
    }
}

fn manhatten_distance(_world: &World, start: &Point, end: &Point) -> u64 {
    let x_diff = i8::abs(start.0 - end.0) as u64;
    let y_diff = i8::abs(start.1 - end.1) as u64;
    let z_diff = i8::abs(start.2 - end.2) as u64;

    let r = x_diff + y_diff + z_diff;
    r * 3
}

pub fn task1(mut points: Linewise<Point>) -> Result<u64, Error>{
    let world = World::new(points.try_collect()?);
    let mut count: u64 = 0;
        
    for point in &world.points {
        for dir in ALL_DIRECTIONS {
            let neighbor = offset(point, dir);

            if world.contains(&neighbor) {
                continue;
            }
            
            count += 1;
        }
    }
    Ok(count)
}

pub fn task2(mut points: Linewise<Point>) -> Result<u64, Error>{
    let world = World::new(points.try_collect()?);
    let agent = Agent;
    let alg = pf::astar::Algorithm::new(manhatten_distance);
    let root = Point(0, 0, 0);

    let mut count: u64 = 0;
        
    for point in &world.points {
        for dir in ALL_DIRECTIONS {
            let neighbor = offset(point, dir);

            if world.contains(&neighbor) {
                continue;
            }
            if pf::Algorithm::get_path(
                &alg, &world, &agent, neighbor, root,
            )
            .is_err()
            {
                continue;
            }
            count += 1;
        }
    }
    Ok(count)
}
