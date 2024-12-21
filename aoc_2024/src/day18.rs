use ahash::HashSet;
use common::input::Input;
use common::pathfinding::Algorithm;
use nalgebra::{point, vector, Vector2};
use std::hash::{Hash, Hasher};
use std::io::BufRead;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PatternParse(#[from] pattern_parse::ParseError),
    #[error(transparent)]
    NoPath(#[from] common::pathfinding::astar::Error),
}

pattern_parse::parse_fn!(parse, "{u8},{u8}");

type Point = nalgebra::Point2<u8>;

pub struct Points(Vec<Point>);

impl Input<'_> for Points {
    type Error = Error;

    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut points = Vec::new();
        let mut line = String::new();
        while matches!(read.read_line(&mut line), Ok(i) if i > 0) {
            let (x, y) = parse(&line)?;
            points.push(point![x, y]);
            line.clear();
        }

        Ok(Self(points))
    }
}

#[derive(Debug)]
pub struct Map<const SIZE: u8>(HashSet<Point>);

impl<const SIZE: u8> Map<SIZE> {
    fn from_input(points: &Points, count: usize) -> Self {
        let point_set = points.0.iter().take(count).cloned().collect();
        Self(point_set)
    }
}

impl<const SIZE: u8> common::pathfinding::World<'_> for Map<SIZE> {
    type Index = Index;
    type Neighbors = std::vec::IntoIter<Index>;

    fn get_neighbors(&self, origin: &Self::Index) -> Self::Neighbors {
        fn try_find_neighbor<const SIZE: u8>(origin: &Index, offset: Vector2<i8>) -> Option<Index> {
            let x = origin.point.x.checked_add_signed(offset.x)?;
            let y = origin.point.y.checked_add_signed(offset.y)?;

            if x >= SIZE || y >= SIZE {
                return None;
            }

            Some(Index {
                point: point![x, y],
                time: origin.time.map(|t| t + 1),
            })
        }

        let mut neighbors = vec![];
        neighbors.extend(try_find_neighbor::<SIZE>(origin, vector![-1, 0]));
        neighbors.extend(try_find_neighbor::<SIZE>(origin, vector![1, 0]));
        neighbors.extend(try_find_neighbor::<SIZE>(origin, vector![0, -1]));
        neighbors.extend(try_find_neighbor::<SIZE>(origin, vector![0, 1]));
        neighbors.into_iter()
    }
}

impl<const SIZE: u8> Map<SIZE> {
    fn is_blocked(&self, index: &Index) -> bool {
        self.0.contains(&index.point)
    }
}

struct Agent;

impl<const SIZE: u8> common::pathfinding::Agent<'_, Map<SIZE>> for Agent {
    type Cost = u32;

    fn get_cost(
        &self,
        world: &Map<SIZE>,
        start: &Index,
        destination: &Index,
    ) -> Option<Self::Cost> {
        if world.is_blocked(start) || world.is_blocked(destination) {
            None
        } else {
            let x_diff = start.point.x.abs_diff(destination.point.x) as u32;
            let y_diff = start.point.y.abs_diff(destination.point.y) as u32;
            let score = x_diff + y_diff;
            Some(score)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Index {
    point: Point,
    time: Option<u32>,
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state)
    }
}

impl PartialEq<Self> for Index {
    fn eq(&self, other: &Self) -> bool {
        match (self.time, other.time) {
            (Some(a), Some(b)) => a == b && self.point == other.point,
            _ => self.point == other.point,
        }
    }
}

impl Eq for Index {}

pub fn task1(input: Points) -> Result<usize, Error> {
    task1_core(Map::<71>::from_input(&input, 1024))
}

pub fn task1_core<const SIZE: u8>(map: Map<SIZE>) -> Result<usize, Error> {
    let agent = Agent;
    let start = Index {
        point: point![0, 0],
        time: Some(0),
    };
    let end = Index {
        point: point![SIZE - 1, SIZE - 1],
        time: None,
    };
    let path = common::pathfinding::astar::Algorithm::<Map<SIZE>, _, _>::new(|_, start, end| {
        let x_diff = start.point.x.abs_diff(end.point.x) as u32;
        let y_diff = start.point.y.abs_diff(end.point.y) as u32;
        x_diff + y_diff
    })
    .get_path(&map, &agent, start, end)?;

    Ok(path.len() - 1)
}

pub fn task2(input: Points) -> Result<String, Error> {
    task2_core::<71>(input)
}

pub fn task2_core<const SIZE: u8>(input: Points) -> Result<String, Error> {
    let agent = Agent;
    let start = Index {
        point: point![0, 0],
        time: Some(0),
    };
    let end = Index {
        point: point![SIZE - 1, SIZE - 1],
        time: None,
    };

    let mut low = 0;
    let mut high = input.0.len(); // Maximum points to consider
    let mut min_blocker = high; // Store the minimum number of points found
    while low <= high {
        let cursor = (low + high) / 2;
        let map = Map::from_input(&input, cursor);

        let path =
            common::pathfinding::astar::Algorithm::<Map<SIZE>, _, _>::new(|_, start, end| {
                let x_diff = start.point.x.abs_diff(end.point.x) as u32;
                let y_diff = start.point.y.abs_diff(end.point.y) as u32;
                x_diff + y_diff
            })
            .try_get_path(&map, &agent, start, end, Some(250_000));
        let path_exists = path.is_ok();

        if path_exists {
            // Path exists, so we need to block more points
            low = cursor + 1;
        } else {
            // Path is blocked, try fewer points
            min_blocker = cursor;
            high = cursor - 1;
        }
    }

    let blocker = input.0[min_blocker - 1];
    let result = format!("{},{}", blocker.x, blocker.y);
    assert_ne!(&result, "55,46");
    assert_ne!(&result, "62,39");
    // 54,44
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let points = Points::parse(buf).unwrap();
        let map = Map::<7>::from_input(&points, 12);
        let result = task1_core(map);
        let val = result.unwrap();
        assert_eq!(val, 22);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2_core::<7>(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val.as_str(), "6,1");
    }
}
