use crate::input::Input;
use bitflags::bitflags;
use std::collections::hash_map::Entry;
use std::convert::Infallible;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use ahash::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unexpected char '{0}' in input")]
    UnexpectedInputChar(char),
}

bitflags! {
    struct Direction: i8 {
        const NORTH = 1;
        const SOUTH = 2;
        const WEST = 4;
        const EAST = 8;
    }
}

#[derive(Debug, Copy, Clone)]
struct Elf {
    x: i32,
    y: i32,
}

impl Hash for Elf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Elf {}

impl Elf {
    fn next_move(&self, step_index: usize, others: &HashSet<Elf>) -> Option<Elf> {
        const CARDINALS: [Direction; 4] = [
            Direction::NORTH,
            Direction::SOUTH,
            Direction::WEST,
            Direction::EAST,
        ];

        const DIRECTIONS: [Direction; 8] = [
            Direction::NORTH,
            Direction::NORTH.union(Direction::WEST),
            Direction::NORTH.union(Direction::EAST),
            Direction::SOUTH,
            Direction::SOUTH.union(Direction::WEST),
            Direction::SOUTH.union(Direction::EAST),
            Direction::WEST,
            Direction::EAST,
        ];

        // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
        let alone = DIRECTIONS.into_iter().all(move |dir| {
            let (x, y) = dir.offset(self.x, self.y);
            let elf = Elf { x, y };
            others.contains(&elf) == false
        });

        if alone {
            return None;
        }

        // Otherwise, the Elf looks in each of four directions [...] and proposes
        // moving one step in the first valid direction:
        for i in 0..4 {
            // Finally, at the end of the round, the first direction the Elves
            // considered is moved to the end of the list of directions. For
            // example, during the second round, the Elves would try proposing
            // a move to the south first, then west, then east, then north.
            let index = (i + step_index) % 4;
            let m = CARDINALS[index];

            let dir_free = m.with_adjacent().into_iter().all(move |m| {
                let (x, y) = m.offset(self.x, self.y);
                let elf = Elf { x, y };
                others.contains(&elf) == false
            });
            if dir_free {
                let (x, y) = m.offset(self.x, self.y);
                return Some(Elf { x, y });
            }
        }

        None
    }
}

impl Direction {
    fn with_adjacent(self) -> [Direction; 3] {
        assert_eq!(self.bits.count_ones(), 1);
        if self.intersects(Direction::NORTH | Direction::SOUTH) {
            [self, self | Direction::WEST, self | Direction::EAST]
        } else {
            [self, self | Direction::NORTH, self | Direction::SOUTH]
        }
    }
    fn offset(self, mut x: i32, mut y: i32) -> (i32, i32) {
        if self.contains(Direction::NORTH) {
            y -= 1
        }
        if self.contains(Direction::SOUTH) {
            y += 1
        }
        if self.contains(Direction::WEST) {
            x -= 1
        }
        if self.contains(Direction::EAST) {
            x += 1
        }
        (x, y)
    }
}

#[derive(Debug)]
pub struct Map(HashSet<Elf>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_min, x_max, y_min, y_max) = self.get_bounds();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if self.0.contains(&Elf { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Input<'a> for Map {
    type Error = Error;

    fn parse<R: 'a + std::io::BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut line = String::new();
        let mut elves = HashSet::default();
        let mut y = 0;
        loop {
            y += 1;
            line.clear();
            if read.read_line(&mut line)? == 0 {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert(Elf { x: x as i32, y });
                }
            }
        }
        Ok(Map(elves))
    }
}

impl Map {
    fn step(&mut self, step_index: usize) -> bool {
        let mut moves = HashMap::default();
        let mut unchanged = vec![];

        for elf in &self.0 {
            match elf.next_move(step_index, &self.0) {
                Some(next) => match moves.entry(next) {
                    Entry::Vacant(vac) => {
                        vac.insert(vec![*elf]);
                    }
                    Entry::Occupied(mut occ) => {
                        occ.get_mut().push(*elf);
                    }
                },
                None => unchanged.push(*elf),
            }
        }

        if moves.is_empty() {
            return false;
        }

        self.0.clear();
        self.0.extend(unchanged.into_iter());
        for (target, moving) in moves.drain() {
            if moving.len() == 1 {
                self.0.insert(target);
            } else {
                self.0.extend(moving);
            }
        }
        true
    }

    fn get_occupied_tile_count(&self) -> i32 {
        let count = self.0.len();
        let (x_min, x_max, y_min, y_max) = self.get_bounds();

        let area = (x_max - x_min + 1) * (y_max - y_min + 1);
        let free = area - count as i32;
        free
    }

    /// Returns the (x_min, x_max, y_min, y_max)
    fn get_bounds(&self) -> (i32, i32, i32, i32) {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        for e in self.0.iter() {
            x_min = std::cmp::min(x_min, e.x);
            x_max = std::cmp::max(x_max, e.x);
            y_min = std::cmp::min(y_min, e.y);
            y_max = std::cmp::max(y_max, e.y);
        }
        (x_min, x_max, y_min, y_max)
    }
}

pub fn task1(mut map: Map) -> Result<i32, Infallible> {
    for i in 0..10 {
        map.step(i);
    }

    let free = map.get_occupied_tile_count();
    Ok(free)
}

pub fn task2(mut map: Map) -> Result<usize, Infallible> {
    let mut round = 0;
    loop {
        let any_movement = map.step(round);
        round += 1;

        if any_movement == false {
            break;
        }
    }

    Ok(round)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::input::Input;

    use super::*;

    const INPUT: &[u8] = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#
        .as_bytes();

    #[test]
    fn test_task1() {
        let mut map = Map::parse(BufReader::new(INPUT)).unwrap();

        println!("Initial state:\n{map}");
        for i in 0..10 {
            map.step(i);
            println!("After step {i}:\n{map}");
        }

        let free = map.get_occupied_tile_count();
        assert_eq!(free, 110);
    }
    #[test]
    fn test_task2() {
        let mut map = Map::parse(BufReader::new(INPUT)).unwrap();

        let mut round = 0;
        loop {
            let any_movement = map.step(round);
            round += 1;

            if any_movement == false {
                break;
            }
        }

        assert_eq!(round, 20);
    }
}
