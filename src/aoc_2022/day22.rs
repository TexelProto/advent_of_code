#![allow(dead_code)] // stop complaining about the unfinished task2 >:(

use std::ops::Deref;

use grid::Grid;

/// Enumeration covering all types of tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    /// A blank tile represented in the input as ' '. It is itself not traversable but it
    /// possible to wrap around the "opposing" tile is `Free`
    Blank,
    /// A non-traversable tile represented in the input as '#'.
    Wall,
    /// A traversable tile represented in the input as '.'.
    Free,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rotate { ccw: bool },
    Move(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn score(self) -> usize {
        self as u8 as usize
    }

    fn cw(self) -> Self {
        let mut i = self as u8;
        // the variants are sorted in clockwise order so just move to the "next"
        // wrapping if neccessary
        i = (i + 1) % 4;
        // SAFETY: Rotation variants are implemented in the range 0..=3
        // the preceeding %4 ensures that this range is never exceede
        unsafe { std::mem::transmute(i) }
    }
    fn ccw(self) -> Self {
        let mut i = self as u8;
        // +3 because
        // -1 to get the previous rotation Self::Right = 0 would underflow/panic
        // to prevent that +4 since 4 % 4 == 0
        // so shortened +3
        i = (i + 3) % 4;

        // SAFETY: Rotation variants are implemented in the range 0..=3
        // the preceeding %4 ensures that this range is never exceede
        unsafe { std::mem::transmute(i) }
    }
}

#[derive(Debug)]
struct Map(Grid<Tile>);

impl Deref for Map {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_map(s: &str) -> Map {
    let mut tiles = Vec::new();
    let line_len = s.lines().map(str::trim_end).map(str::len).max().expect("Input lines were empty.");
    s.lines()
        .map(str::trim_end)
        .take_while(|s| s.len() > 0)
        .for_each(|line| {
            let new_tiles = line.chars().filter_map(|c| match c {
                ' ' => Some(Tile::Blank),
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Free),
                _ => None,
            });
            let padding = line_len - line.len();
            let padding = std::iter::repeat(Tile::Blank).take(padding);
            tiles.extend(new_tiles.chain(padding));
        });
    let grid = Grid::from_vec(tiles, line_len);
    Map(grid)
}

fn parse_moves(s: &str) -> Result<Vec<Move>, std::num::ParseIntError> {
    let mut s = s.trim_end().lines().last().unwrap();
    let mut moves = Vec::new();
    while s.is_empty() == false {
        let c = s.chars().next().unwrap();
        let mov = match c {
            'R' => (Move::Rotate { ccw: false }, 1),
            'L' => (Move::Rotate { ccw: true }, 1),
            _ => {
                let (mov, len) = pattern_parse::PatternParse::parse(s)?;
                (Move::Move(mov), len)
            }
        };
        moves.push(mov.0);
        s = &s[mov.1..];
    }
    Ok(moves)
}

fn shift(val: usize, add: isize, max: usize) -> usize {
    match val.checked_add_signed(add) {
        Some(x) => x % max,
        None => max - 1,
    }
}

fn next_point_wrapping(x: usize, y: usize, dir: Direction, map: &Map) -> (usize, usize) {
    let max_x = map.cols();
    let max_y = map.rows();

    let (nx, ny) = match dir {
        Direction::Right => (shift(x, 1, max_x), y),
        Direction::Down => (x, shift(y, 1, max_y)),
        Direction::Left => (shift(x, -1, max_x), y),
        Direction::Up => (x, shift(y, -1, max_y)),
    };
    (nx, ny)
}

fn wrapping_step(mut x: usize, mut y: usize, dir: Direction, map: &Map) -> Option<(usize, usize)> {
    loop {
        let (nx, ny) = next_point_wrapping(x, y, dir, map);

        match map[ny][nx] {
            // we hit a wall, so just stay where you were
            Tile::Wall => return None,
            // target tile is free to move to
            Tile::Free => return Some((nx, ny)),
            // tile is blank... lets see where it leads
            Tile::Blank => (x, y) = (nx, ny),
        }
    }
}

pub fn task1(input: String) -> Result<usize, std::num::ParseIntError> {
    let map = parse_map(&input);
    let moves = parse_moves(&input)?;

    // find the leftmost free tile on the top row
    let mut x = map.iter_row(0).position(|t| *t == Tile::Free).unwrap();
    let mut y = 0;
    let mut direction = Direction::Right;

    for mov in moves.into_iter() {
        match mov {
            Move::Rotate { ccw } => {
                direction = if ccw { direction.ccw() } else { direction.cw() };
            }
            Move::Move(steps) => {
                for _ in 0..steps {
                    match wrapping_step(x, y, direction, &map) {
                        Some((nx, ny)) => (x, y) = (nx, ny),
                        None => break,
                    }
                }
            }
        };
    }
    let score = 1000 * (y + 1) + 4 * (x + 1) + direction.score();
    Ok(score)
}

fn next_point_cube(x: usize, y: usize, dir: Direction, map: &Map) -> (usize, usize) {
    let max_x = map.cols();
    let max_y = map.rows();

    let (nx, ny) = match dir {
        Direction::Right => (shift(x, 1, max_x), y),
        Direction::Down => (x, shift(y, 1, max_y)),
        Direction::Left => (shift(x, -1, max_x), y),
        Direction::Up => (x, shift(y, -1, max_y)),
    };
    (nx, ny)
}

fn cube_step(mut x: usize, mut y: usize, dir: Direction, map: &Map) -> Option<(usize, usize)> {
    loop {
        let (nx, ny) = next_point_wrapping(x, y, dir, map);

        match map[ny][nx] {
            // we hit a wall, so just stay where you were
            Tile::Wall => return None,
            // target tile is free to move to
            Tile::Free => return Some((nx, ny)),
            // tile is blank... lets see where it leads
            Tile::Blank => (x, y) = (nx, ny),
        }
    }
}
