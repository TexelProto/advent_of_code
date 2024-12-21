use common::{
    geometry_2d::{Direction, Point},
    input::digits::DigitMap,
    pathfinding::{self as pf, Algorithm},
    some_or_continue,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Debug)]
struct Map(Vec<Vec<u8>>);

impl Map {
    fn value_of(&self, p: Point) -> u32 {
        self.0[p.y as usize][p.x as usize] as u32
    }
}

impl<'a> pf::World<'a> for Map {
    type Index = Point;
    type Neighbors = Vec<Point>;

    fn get_neighbors(&'a self, origin: &Self::Index) -> Self::Neighbors {
        let mut neighbors = vec![];
        for dir in Direction::CARDINALS {
            let p = some_or_continue!(Point::offset(*origin, dir));
            let row = some_or_continue!(self.0.get(p.y as usize));
            let _cell = some_or_continue!(row.get(p.x as usize));
            neighbors.push(p);
        }
        neighbors
    }
}

struct Agent;

impl<'a> pf::Agent<'a, Map> for Agent {
    type Cost = u32;

    fn get_cost(&self, world: &Map, _start: &Point, destination: &Point) -> Option<Self::Cost> {
        Some(world.value_of(*destination))
    }
}

pub fn task1(input: DigitMap<u8>) -> Result<u32, Error> {
    let map = Map(input.into_inner());

    let start = Point { x: 0, y: 0 };
    let height = map.0.len();
    let width = map.0[0].len();
    let target = Point {
        x: (width - 1) as u32,
        y: (height - 1) as u32,
    };

    let path = pf::djikstra::Algorithm.get_path(&map, &Agent, start, target)
        .expect("Failed to find path");

    let result = path.positions()[1..].iter().map(|p| map.value_of(*p)).sum::<u32>();
    Ok(result)
}

pub fn copy_plus_one(src: &[u8], dest: &mut [u8]) {
    debug_assert_eq!(src.len(), dest.len());

    for i in 0..src.len() {
        let mut v = src[i];
        v += 1;
        if v == 10 { v = 1 };
        dest[i] = v;
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<u8>]) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn task2(input: DigitMap<u8>) -> Result<u32, Error> {
    let init = input.into_inner();
    let init_height = init.len();
    let init_width = init[0].len();

    let mut vecs = Vec::with_capacity(init.len() * 5);
    for y in 0..init.len() {
        // section 1
        let mut vec = vec![0; init_width * 5];
        vec[..init_width].copy_from_slice(&init[y]);
        // section 2
        let (old, new) = vec.split_at_mut(init_width);
        copy_plus_one(old, &mut new[..init_width]);
        // section 3
        let (old, new) = new.split_at_mut(init_width);
        copy_plus_one(old, &mut new[..init_width]);
        // section 4
        let (old, new) = new.split_at_mut(init_width);
        copy_plus_one(old, &mut new[..init_width]);
        // section 5
        let (old, new) = new.split_at_mut(init_width);
        copy_plus_one(old, &mut new[..init_width]);
        // done
        vecs.push(vec);
    }

    for block in 0..4 {
        for y in 0..init_height {
            let src = &vecs[y + init_height * block];
            let mut dest = vec![0; init_width * 5];
            copy_plus_one(&src, &mut dest);
            vecs.push(dest);
        }
    }
    
    let map = Map(vecs);

    let start = Point { x: 0, y: 0 };
    let height = map.0.len();
    let width = map.0[0].len();
    let target = Point {
        x: (width - 1) as u32,
        y: (height - 1) as u32,
    };

    let alg = pf::djikstra::Algorithm;
    let path = alg.get_path(&map, &Agent, start, target)
        .expect("Failed to find path");

    let result = path.positions()[1..].iter().map(|p| map.value_of(*p)).sum::<u32>();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
        .as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 40);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 315);
    }
}
