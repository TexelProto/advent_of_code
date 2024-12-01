use common::bit_set::BitSet;
use common::debug::BinDebug;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Clone)]
pub struct BitGrid {
    rows: Vec<BitSet>,
    width: usize,
    height: usize,
}

impl Debug for BitGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for row in &self.rows {
            list.entry(&BinDebug(&row));
        }
        list.finish()
    }
}

impl FromStr for BitGrid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            height += 1;

            let set = line
                .char_indices()
                .fold(BitSet::new(width), |mut set, (i, c)| {
                    if c == '#' {
                        set.set(i);
                    }
                    set
                });
            bits.push(set);
        }
        Ok(Self {
            rows: bits,
            width,
            height,
        })
    }
}

fn parse_input(input: String) -> Result<Vec<BitGrid>, Error> {
    let mut buf = String::new();
    let mut grids = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            grids.push(BitGrid::from_str(&buf)?);
            buf.clear();
            continue;
        }

        buf.push_str(line);
        buf.push('\n');
    }
    grids.push(BitGrid::from_str(&buf)?);
    Ok(grids)
}

pub fn task1(input: String) -> Result<usize, Error> {
    let grids = parse_input(input)?;

    let mut total = 0;
    for grid in grids {
        let (mut value, rotated) = find_mirror(grid).expect("Failed to find reflection");
        if rotated == false {
            value *= 100;
        }
        total += value;
    }

    Ok(total)
}

fn rotate(src: BitGrid) -> BitGrid {
    let mut bits = Vec::with_capacity(src.width);

    for x in 0..src.width {
        let mut new_row = BitSet::new(src.height);
        for y in 0..src.height {
            if src.rows[y].get(x) {
                new_row.set(y);
            }
        }
        bits.push(new_row)
    }

    BitGrid {
        rows: bits,
        width: src.height,
        height: src.width,
    }
}

fn is_mirrored(grid: &BitGrid, edge: usize) -> bool {
    let over = (0..edge).rev();
    let under = edge..grid.height;
    over.zip(under).all(|(a, b)| grid.rows[a] == grid.rows[b])
}

fn find_mirror(grid: BitGrid) -> Option<(usize, bool)> {
    for y in 1..grid.height {
        if is_mirrored(&grid, y) {
            return Some((y, false));
        }
    }

    let grid = rotate(grid);
    for y in 1..grid.height {
        if is_mirrored(&grid, y) {
            return Some((y, true));
        }
    }

    None
}

pub fn task2(input: String) -> Result<usize, Error> {
    let grids = parse_input(input)?;

    let mut total = 0;
    for grid in grids {
        let (mut value, rotated) = find_smudge_mirror(grid).expect("Failed to find smudge");
        if rotated == false {
            value *= 100;
        }
        total += value;
    }

    Ok(total)
}

fn find_smudge_mirror(grid: BitGrid) -> Option<(usize, bool)> {
    for y in 1..grid.height {
        let over = (0..y).rev();
        let under = y..grid.height;
        let diff = over
            .zip(under)
            .map(|(a, b)| {
                let a = grid.rows[a].iter();
                let b = grid.rows[b].iter();
                a.zip(b).map(|(a, b)| (a ^ b) as usize).sum::<usize>()
            })
            .sum::<usize>();

        if diff == 1 {
            return Some((y, false));
        }
    }

    let grid = rotate(grid);

    for y in 1..grid.height {
        let over = (0..y).rev();
        let under = y..grid.height;
        let diff = over
            .zip(under)
            .map(|(a, b)| {
                let a = grid.rows[a].iter();
                let b = grid.rows[b].iter();
                a.zip(b).map(|(a, b)| (a ^ b) as usize).sum::<usize>()
            })
            .sum::<usize>();

        if diff == 1 {
            return Some((y, true));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 405);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 400);
    }
}
