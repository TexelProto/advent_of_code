use std::{convert::Infallible, str::FromStr, io::BufRead};

use crate::{common::pathfinding as pf, input::Input};

pub struct Vec2d<T>(Vec<Vec<T>>);

type Map = Vec2d<char>;

impl Input<'_> for Map {
    type Error = std::io::Error;
    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut lines = Vec::new();
        let mut buf = String::new();
        while read.read_line(&mut buf)? > 0 {
            let line = buf.trim().chars().collect();
            lines.push(line);
            buf.clear();
        }
        Ok(Vec2d(lines))
    }
}

impl FromStr for Map {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ))
    }
}

impl pf::World<'_> for Map {
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
        if origin.0 < self.0[0].len() - 1 {
            vec.push((origin.0 + 1, origin.1));
        }
        if origin.1 < self.0.len() - 1 {
            vec.push((origin.0, origin.1 + 1));
        }

        vec.into_iter()
    }
}

struct Agent;

impl pf::Agent<'_, Map> for Agent {
    type Score = u64;
    fn get_cost(
        &self,
        world: &Map,
        start: &<Map as pf::World>::Index,
        destination: &<Map as pf::World>::Index,
    ) -> Option<Self::Score> {
        let mut start_char = world.0[start.1][start.0];
        if start_char == 'S' {
            start_char = 'a'
        }
        let mut dest_char = world.0[destination.1][destination.0];
        if dest_char == 'E' {
            dest_char = 'z'
        }

        let start_num = start_char as u8;
        let destination_num = dest_char as u8;

        let result = if start_num + 1 >= destination_num {
            Some(1)
        } else {
            None
        };
        // dbg!(start_char, dest_char, result);
        result
    }
}

fn find(c: char, map: &Map) -> (usize, usize) {
    for y in 0..map.0.len() {
        for x in 0..map.0[0].len() {
            if map.0[y][x] == c {
                return (x, y);
            }
        }
    }
    panic!("char not found '{}'", c)
}

fn find_all(c: char, map: &Map) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    for y in 0..map.0.len() {
        for x in 0..map.0[y].len() {
            if map.0[y][x] == c {
                vec.push((x, y));
            }
        }
    }
    vec
}

pub fn task1(map: Map) -> Result<usize, pf::djikstra::Error>{
    let agent = Agent;
    let alg = pf::djikstra::Algorithm;

    let start = find('S', &map);
    let end = find('E', &map);

    let path = pf::Algorithm::get_path(&alg, &map, &agent, start, end)?;
    Ok(path.len())
}

pub fn task2(map: Map) -> Result<usize, pf::djikstra::Error>{
    let agent = Agent;
    let alg = pf::djikstra::Algorithm;

    let start = find('S', &map);
    let end = find('E', &map);

    let mut path = pf::Algorithm::get_path(&alg, &map, &agent, start, end)?;

    for start in find_all('a', &map) {
        pf::Algorithm::get_path(&alg, &map, &agent, start, end).ok().map(
            |new_path| {
                if new_path.len() < path.len() {
                    path = new_path;
                }
            }
        );  
    }
    Ok(path.len())
}

#[allow(dead_code)]
fn print_path(path: pf::Path<'_, Map>) {
    let world = path.world();

    let mut chars = world.0.clone();

    for indices in path.positions().windows(2) {
        let (x, y) = indices[0];
        let (next_x, next_y) = indices[1];
        let c = if next_x > x {
            '>'
        } else if x > next_x {
            '<'
        } else if next_y < y {
            '^'
        } else {
            'v'
        };
        chars[y][x] = c;
    }

    for line in chars {
        let s =  String::from_iter(line);
        println!("{}", s);
    }
}
