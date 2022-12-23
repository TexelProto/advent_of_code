use crate::common::pathfinding::*;

const MAP: Map = Map([[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]);

#[derive(Debug)]
struct Map([[usize; 4]; 4]);

impl World<'_> for Map {
    type Index = (usize, usize);
    type Neighbors = std::vec::IntoIter<Self::Index>;

    fn get_neighbors(&self, origin: &Self::Index) -> Self::Neighbors {
        let mut vec = Vec::new();

        if origin.0 > 0 {
            vec.push((origin.0 - 1, origin.1));
        }
        if origin.1 > 0 {
            vec.push((origin.0, origin.1 - 1));
        }
        if origin.0 < 3 {
            vec.push((origin.0 + 1, origin.1));
        }
        if origin.1 < 3 {
            vec.push((origin.0, origin.1 + 1));
        }

        vec.into_iter()
    }
}

struct TestAgent;

impl Agent<'_, Map> for TestAgent {
    type Score = NonNanF32;
    fn get_cost(
        &self,
        world: &Map,
        _start: &<Map as World>::Index,
        destination: &<Map as World>::Index,
    ) -> Option<Self::Score> {
        if world.0[destination.0][destination.1] == 0 {
            Some(NonNanF32::try_from(1.0).unwrap())
        } else {
            None
        }
    }
}

fn possible_path<'a, A: Algorithm<'a, Map, TestAgent>>(alg: &A) -> bool {
    let path = alg.get_path(&MAP, &TestAgent, (0, 0), (1, 3));
    dbg!(path).is_ok()
}

fn impossible_path<'a, A: Algorithm<'a, Map, TestAgent>>(alg: &A) -> bool {
    let path = alg.get_path(&MAP, &TestAgent, (0, 0), (3, 3));
    dbg!(path).is_err()
}

fn heuristic(_world: &Map, tile: &(usize, usize), end: &(usize, usize)) -> NonNanF32 {
    let x_diff = tile.0 as isize - end.0 as isize;
    let y_diff = tile.1 as isize - end.1 as isize;

    let sq_sum = x_diff * x_diff + y_diff * y_diff;
    NonNanF32::try_from(f32::sqrt(sq_sum as f32)).unwrap()
}

#[test]
fn possible_path_astar() {
    let alg = pathfinding::astar::Algorithm::new(heuristic);
    assert!(possible_path(&alg));
}

#[test]
fn impossible_path_astar() {
    let alg = pathfinding::astar::Algorithm::new(heuristic);
    assert!(impossible_path(&alg));
}

#[test]
fn possible_path_djikstra() {
    let alg = pathfinding::djikstra::Algorithm;
    assert!(possible_path(&alg));
}

#[test]
fn impossible_path_djikstra() {
    let alg = pathfinding::djikstra::Algorithm;
    assert!(impossible_path(&alg));
}
