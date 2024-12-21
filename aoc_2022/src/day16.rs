use std::collections::HashSet;
use std::iter::Cloned;
use std::{str::FromStr, num::ParseIntError, collections::HashMap};

use rayon::prelude::{ParallelIterator, IntoParallelIterator};

use  common::iter_ext::TryIterator;
use common::input::Linewise;
use  common::pathfinding::{self as pf, Algorithm};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError)
}

fn encode_name(s: &str) -> u16 {
    debug_assert!(s.is_ascii());
    debug_assert_eq!(s.len(), 2);

    unsafe {
        let b = s.as_bytes().as_ptr();
        std::ptr::read_unaligned(b as *const u16)
    }
}

fn split_while(s: &str, mut predicate: impl FnMut(char) -> bool) -> &str {
    let res = s.char_indices()
        .take_while(|(_i,c)| predicate(*c))
        .last();
    match res {
        Some((i,_)) => &s[..=i],
        None => &s[0..0],
    }
}

#[derive(Debug)]
pub struct Valve {
    name: u16,
    flow_rate: usize,
    connected: Vec<u16>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = &s[6..8];
        let name = encode_name(name);

        let flow_rate = split_while(&s[23..], char::is_numeric);
        let flow_rate = flow_rate.parse()?;

        let connected = s[49..].rsplit(", ").map(|s| {
                    let len = s.len();
                    encode_name(&s[len-2..len])
                }).collect::<Vec<_>>();

        Ok(Valve{name, flow_rate, connected})
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Valve {}

#[derive(Debug)]
struct Map {
    valves: HashMap<u16, Valve>,
    non_zero: Vec<u16>,
    distances: HashMap<u32, usize>,
}

fn encode_connection(v1: u16, v2 : u16) -> u32 {
    (v1 as u32) | ((v2 as u32) << 16)
}

impl Map {
    fn from_valves(valves: Vec<Valve>) -> Self {
        let valves = valves.into_iter().map(|v| (v.name, v)).collect::<HashMap<_,_>>();

        let aa_name = encode_name("AA");
        let non_zero = valves.iter().filter_map(|(id, v)| {
            if v.name == aa_name || v.flow_rate > 0 {Some(*id)} else {None}
        }).collect::<Vec<_>>();

        let mut world = Self {
            valves,
            non_zero,
            distances: HashMap::new(),
        };

        let distances = world.non_zero.iter()
            .flat_map(|v1|{
                let world = &world;
                world.non_zero.iter().map(move |v2| {
                    let id = encode_connection(*v1, *v2);
                    let dist = if v1 == v2 {
                        0
                    } else {
                        pf::djikstra::Algorithm.get_path(world, &Agent, *v1, *v2).unwrap().len()
                    };
                    (id, dist)
            })
        }).collect::<HashMap<_,_>>();

        world.distances = distances;
        world
    }
}

impl<'a> pf::World<'a> for Map {
    type Index = u16;
    type Neighbors = Cloned<std::slice::Iter<'a, u16>>;

    fn get_neighbors(&'a self, origin: &Self::Index) -> Self::Neighbors {
        self.valves[origin].connected.iter().cloned()
    }
}

struct Agent;

impl pf::Agent<'_, Map> for Agent {
    type Cost = u64;

    fn get_cost(&self, _world: &Map, _start: &u16, _destination: &u16) -> Option<Self::Cost> {
        Some(1)
    }
}

fn is_included(i: usize, mask: u32) -> bool {
    mask & (1_u32 << i) > 0
}

fn traverse_masked_map<const MAX: usize>(step: usize, valve: &Valve, mut acc: usize, map: &Map, mask: u32, visited: &mut HashSet<u16>) -> usize {
    acc += valve.flow_rate * (MAX - step);
    visited.insert(valve.name);

    let best = map.non_zero.iter().enumerate().filter_map(|(i,target)| {
        if is_included(i, mask) == false {
            return None;
        }
        if visited.contains(target) {
            return None;
        }
        let connection = encode_connection(valve.name, *target);
        let dist = map.distances.get(&connection).unwrap();
        let target_step = step + dist;
        if target_step >= MAX {
            return Some(acc);
        }

        let target = &map.valves[target];
        Some(traverse_masked_map::<MAX>(target_step, target, acc, map, mask, visited))
    }).max().unwrap_or(0);
    visited.remove(&valve.name);

    best
}

pub fn task1(valves: Linewise<Valve>) -> Result<usize, Error> {
    let valves = valves.try_collect2()?;
    let map = Map::from_valves(valves);

    let mut visited = HashSet::new();
    let root_name = encode_name("AA");
    let root_node = &map.valves[&root_name];

    let result = traverse_masked_map::<30>(0, &root_node, 0, &map, !0, &mut visited);

    Ok(result)
}

pub fn task2(valves: Linewise<Valve>) -> Result<usize, Error> {
    let valves = valves.try_collect2()?;
    let map = Map::from_valves(valves);

    let root_name = encode_name("AA");
    let root_node = &map.valves[&root_name];

    let len = map.non_zero.len() as u32;
    // nodes = 4
    // 1<<4  = 10000
    // 1<<4-1= 01111
    let max_mask = ( 1 << len ) -1;
    let result = (0_u32..=max_mask).into_par_iter().filter_map(|mask| {
        let ones = mask.count_ones();
        let zeroes = len - ones;
        if ones.abs_diff(zeroes) > 3 {
            return None;
        }

        let mut visited = HashSet::new();
        let res1 = traverse_masked_map::<26>(0, &root_node, 0, &map, mask, &mut visited);
        visited.clear();
        let res2 = traverse_masked_map::<26>(0, &root_node, 0, &map, !mask, &mut visited);
        Some(res1 + res2)
    }).max().unwrap();

    Ok(result)
}