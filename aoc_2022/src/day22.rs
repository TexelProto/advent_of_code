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

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b' ' => Self::Blank,
            b'#' => Self::Wall,
            b'.' => Self::Free,
            _ => panic!("Invalid tile: {}", value as char),
        }
    }
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
    let line_len = s
        .lines()
        .map(str::trim_end)
        .map(str::len)
        .max()
        .expect("Input lines were empty.");
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

fn next_pos_cube<const SIDE_LEN: usize>(
    x: &mut usize,
    y: &mut usize,
    direction: &mut Direction,
    map: &Map,
) {
    // calculate the current sector
    let from_sec_x = *x / SIDE_LEN;
    let from_sec_y = *y / SIDE_LEN;

    // advance to the next point / next sector
    (*x, *y) = next_point_wrapping(*x, *y, *direction, map);
    let to_sec_x = *x / SIDE_LEN;
    let to_sec_y = *y / SIDE_LEN;

    // if we are on the same sector as before, we are done
    if from_sec_x == to_sec_x && from_sec_y == to_sec_y {
        return;
    }

    // we changed sectors, so we need to adjust to the new cube-face
    // assume the shape:
    //   0 1 2
    // 0   U R
    // 1   F
    // 2 L D
    // 3 B
    // U = up, D = down, L = left, R = right, F = front, B = back
    match *direction {
        Direction::Right => match (from_sec_x, from_sec_y) {
            // right -> down
            (2, 0) => {
                *x = 2 * SIDE_LEN - 1;
                *y = 3 * SIDE_LEN - *y - 1;
                *direction = Direction::Left;
            }
            // front -> right
            (1, 1) => {
                *x = *y + SIDE_LEN;
                *y = SIDE_LEN - 1;
                *direction = Direction::Up;
            }
            // down -> right
            (1, 2) => {
                *x = 3 * SIDE_LEN - 1;
                *y = 3 * SIDE_LEN - *y - 1;
                *direction = Direction::Left;
            }
            // back -> down
            (0, 3) => {
                *x = *y - 2 * SIDE_LEN;
                *y = 3 * SIDE_LEN - 1;
                *direction = Direction::Up;
            }
            _ => {}
        },
        Direction::Left => match (from_sec_x, from_sec_y) {
            // up -> left
            (1, 0) => {
                *x = 0;
                *y = 3 * SIDE_LEN - *y - 1;
                *direction = Direction::Right;
            }
            // front -> left
            (1, 1) => {
                *x = *y - SIDE_LEN;
                *y = 2 * SIDE_LEN;
                *direction = Direction::Down;
            }
            // left -> up
            (0, 2) => {
                *x = SIDE_LEN;
                *y = 3 * SIDE_LEN - *y - 1;
                *direction = Direction::Right;
            }
            // back -> up
            (0, 3) => {
                *x = *y - 2 * SIDE_LEN;
                *y = 0;
                *direction = Direction::Down;
            }
            _ => {}
        },
        Direction::Down => match (from_sec_x, from_sec_y) {
            // bottom -> right
            (0, 3) => {
                *x += 2 * SIDE_LEN;
                *y = 0;
                *direction = Direction::Down;
            }
            // down -> back
            (1, 2) => {
                *y = *x + 2 * SIDE_LEN;
                *x = SIDE_LEN - 1;
                *direction = Direction::Left;
            }
            // right -> front
            (2, 0) => {
                *y = *x - SIDE_LEN;
                *x = 2 * SIDE_LEN - 1;
                *direction = Direction::Left;
            }
            _ => {}
        },
        Direction::Up => match (from_sec_x, from_sec_y) {
            // left -> front
            (0, 2) => {
                *y = *x + SIDE_LEN;
                *x = SIDE_LEN;
                *direction = Direction::Right;
            }
            // up -> back
            (1, 0) => {
                *y = *x + 2 * SIDE_LEN;
                *x = 0;
                *direction = Direction::Right;
            }
            // right -> bottom
            (2, 0) => {
                *x -= 2 * SIDE_LEN;
                *y = 4 * SIDE_LEN - 1;
                *direction = Direction::Up;
            }
            _ => {}
        },
    }
}

fn cube_step(
    mut x: usize,
    mut y: usize,
    mut dir: Direction,
    map: &Map,
) -> Option<(usize, usize, Direction)> {
    next_pos_cube::<50>(&mut x, &mut y, &mut dir, map);

    match map[y][x] {
        // we hit a wall, so just stay where you were
        Tile::Wall => return None,
        // target tile is free to move to
        Tile::Free => return Some((x, y, dir)),
        // tile is blank... this should be impossible
        Tile::Blank => unreachable!("Reached a blank tile @ {:?}", (x, y)),
    }
}

pub fn task2(input: String) -> Result<usize, std::num::ParseIntError> {
    let map = parse_map(&input);
    let moves = parse_moves(&input)?;

    // find the leftmost free tile on the top row
    let mut x = map.iter_row(0).position(|t| *t == Tile::Free).unwrap();
    let mut y = 0;
    let mut direction = Direction::Right;

    let mut visited = Vec::new();

    for mov in moves.into_iter() {
        match mov {
            Move::Rotate { ccw } => {
                direction = if ccw { direction.ccw() } else { direction.cw() };
            }
            Move::Move(steps) => {
                // // eprintln!("{:?}", (x, y, direction, steps));
                for _ in 0..steps {
                    visited.push((x, y, direction));
                    match cube_step(x, y, direction, &map) {
                        Some(new) => (x, y, direction) = new,
                        None => break,
                    }
                }
            }
        };
    }

    let score = 1000 * (y + 1) + 4 * (x + 1) + direction.score();
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIDE_LEN: usize = 50;
    const INPUT: &str = include_str!("../../inputs/aoc_2022/day22.txt");

    const DIRECTIONS: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    fn roundtrip_test(rotate: impl Fn(Direction) -> Direction) {
        let map = parse_map(INPUT);
        let width = map.0.cols();
        let height = map.0.rows();
        for init_dir in DIRECTIONS {
            for y in 0..height {
                for x in 0..width {
                    if map[y][x] == Tile::Blank {
                        continue;
                    }

                    eprintln!();
                    let mut dir = init_dir;
                    let (mut mx, mut my) = (x, y);

                    for i in 0..4 {
                        next_pos_cube::<50>(&mut mx, &mut my, &mut dir, &map);
                        dir = rotate(dir);

                        // tile is blank... this should be impossible
                        if map[my][mx] == Tile::Blank {
                            unreachable!("Reached a blank tile @ {:?}", (x, y));
                        }

                        // corners loop faster so exit early
                        if i != 3 && (x, y) == (mx, my) {
                            break;
                        }
                    }
                    assert_eq!((x, y), (mx, my), "Failed at ({}, {}, {:?})", x, y, init_dir);
                }
            }
        }
    }

    #[test]
    fn roundtrip_ccw() {
        roundtrip_test(Direction::ccw);
    }
    #[test]
    fn roundtrip_cw() {
        roundtrip_test(Direction::cw);
    }
}
