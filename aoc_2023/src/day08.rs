use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use ahash::HashMap;
use common::input::{LineSeparated, Linewise};
use common::iter_ext::UnlimitedIterator;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Node([u8; 3]);

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}

impl Node {
    const AAA: Node = Node([b'A', b'A', b'A']);
    const ZZZ: Node = Node([b'Z', b'Z', b'Z']);

    fn is_a(&self) -> bool { self.0[2] == b'A' }
    fn is_z(&self) -> bool { self.0[2] == b'Z' }
}

impl<'a> From<&'a [u8]> for Node {
    fn from(value: &'a [u8]) -> Self {
        let mut array = [0; 3];
        array.copy_from_slice(value);
        Self(array)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NodeRecord {
    src: Node,
    left: Node,
    right: Node,
}

impl FromStr for NodeRecord {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        Ok(Self {
            src: Node::from(&bytes[0..3]),
            left: Node::from(&bytes[7..10]),
            right: Node::from(&bytes[12..15]),
        })
    }
}

pub fn task1<'a>(input: LineSeparated<'a, String, Linewise<'a, NodeRecord>>) -> Result<u32, Error> {
    let (moves, records) = input.into_inner();
    let mut moves = moves.chars().cycle();

    let mut map = HashMap::default();
    for rec in records {
        let rec = rec.unwrap();
        map.insert(rec.src, (rec.left, rec.right));
    }

    let mut current = Node::AAA;
    let mut steps = 0;

    while current != Node::ZZZ {
        steps += 1;
        if moves.next_unlimited() == 'R' {
            current = map[&current].1;
        } else {
            current = map[&current].0;
        }
    }

    Ok(steps)
}

/// Solution to part 2 using the smalles common multiple
///
/// All start points seem to form cycles to an end point and back to the start.
/// While this holds true the first time all start points reach an end is their smallest common multiple.
pub fn task2<'a>(input: LineSeparated<'a, String, Linewise<'a, NodeRecord>>) -> Result<u64, Error> {
    let (moves, records) = input.into_inner();
    let moves = moves.chars().collect::<Vec<_>>();

    let mut map = HashMap::default();
    for rec in records {
        let rec = rec.unwrap();
        map.insert(rec.src, (rec.left, rec.right));
    }

    let cycles = map.keys().cloned()
        .filter(Node::is_a)
        .map(|start| build_cycle(start, &moves, &map))
        .collect::<Vec<_>>();

    for (start, cycle) in &cycles {
        for c in cycle {
            assert_eq!(*c, *start);
        }
    }

    let mut primes = vec![];
    for (total, _) in cycles {
        for p in 2.. {
            if total % p == 0 {
                if primes.contains(&p) == false {
                    primes.push(p);
                }
                let r = total / p;
                if primes.contains(&r) == false {
                    primes.push(r);
                }
                break;
            }
        }
    }
    Ok(primes.into_iter().product())
}

fn build_cycle(start: Node, moves: &Vec<char>, map: &HashMap<Node, (Node, Node)>) -> (u64, Vec<u64>) {
    let mut current = start;
    let move_count = moves.len();

    let mut time = 0;
    while current.is_z() == false {
        if moves[time % move_count] == 'R' {
            current = map[&current].1;
        } else {
            current = map[&current].0;
        }
        time += 1;
    }

    let cycle_start_time = time as u64;
    let cycle_start = (current, time % move_count);
    let mut last_z_time = time;
    let mut z_cycle = vec![];

    loop {
        if moves[time % move_count] == 'R' {
            current = map[&current].1;
        } else {
            current = map[&current].0;
        }
        time += 1;

        if current.is_z() == false { continue; }
        z_cycle.push((time - last_z_time) as u64);
        last_z_time = time;

        if (current, time % move_count) == cycle_start {
            break;
        }
    }
    (cycle_start_time, z_cycle)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(b"\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".as_slice());
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(b"\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".as_slice());
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 6);
    }
}
