use std::collections::hash_map::Entry;

use common::{
    iter_ext::TryIterator,
    input::{
        chars::{Charwise, FromChar},
        lines::Linewise,
    },
};
use ahash::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown shift char '{0}'")]
    UnknownShift(char),
}

type Row = u8;
type Shape<'a> = &'a [Row];
type MutShape = Vec<Row>;

#[rustfmt::skip]
const SHAPES: [Shape<'static>; 5] = [
    // ####
    &[0b_00011110],
    // .#.
    // ###
    // .#.
    &[
        0b_00001000, 
        0b_00011100, 
        0b_00001000,
    ],
    // ..#
    // ..#
    // ###
    &[
        0b_00011100, 
        0b_00000100, 
        0b_00000100,
    ],
    // #
    // #
    // #
    // #
    &[
        0b_00010000, 
        0b_00010000, 
        0b_00010000, 
        0b_00010000
    ],
    // ##
    // ##
    &[
        0b_00011000, 
        0b_00011000
    ],
];

#[derive(Debug, Clone, Copy)]
pub enum Shift {
    Left,
    Right,
}

impl FromChar for Shift {
    type Err = Error;
    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            c => Err(Error::UnknownShift(c)),
        }
    }
}
const WIDTH: u8 = 7;
const LEFT_EDGE: u8 = 1 << (WIDTH - 1);
const RIGHT_EDGE: u8 = 1;

fn try_shift(shape: &mut MutShape, height: usize, shift: Shift, occupied: &Vec<u8>) {
    match shift {
        Shift::Left => {
            let blocked = shape.iter().any(|p| {
                // row is already at the left edge
                p & LEFT_EDGE != 0
            }) || shape_intersects(shape, height, 1, occupied);

            if blocked {
                return;
            }

            shape.iter_mut().for_each(|p| *p <<= 1);
        }
        Shift::Right => {
            let blocked = shape.iter().any(|p| {
                // row is already at the right edge
                p & RIGHT_EDGE != 0
            }) || shape_intersects(shape, height, -1, occupied);

            if blocked {
                return;
            }

            shape.iter_mut().for_each(|p| *p >>= 1);
        }
    }
}

#[inline]
fn shape_intersects(shape: &MutShape, height: usize, shift_left: i8, occupied: &Vec<u8>) -> bool {
    for i in 0..shape.len() {
        let row = match occupied.get(i + height) {
            Some(x) => x,
            None => break,
        };

        let shape_line = match shift_left < 0 {
            true => shape[i] >> -shift_left,
            false => shape[i] << shift_left,
        };
        // check if the shape, after the shift, intersects with the occupied tiles
        if shape_line & row != 0 {
            return true;
        }
    }
    false
}

fn drop_shape(
    mut shape: Vec<u8>,
    shifts: &[Shift],
    shift_index: &mut usize,
    occupied: &mut Vec<u8>,
    max_y: &mut usize,
) {
    let mut height = *max_y + 3;
    loop {
        try_shift(
            &mut shape,
            height,
            shifts[*shift_index],
            &*occupied,
        );
        *shift_index = (*shift_index + 1) % shifts.len();

        if height == 0 {
            break;
        }

        if shape_intersects(&shape, height - 1, 0, &*occupied) {
            break;
        }

        height -= 1;
    }

    *max_y = std::cmp::max(height + shape.len(), *max_y);
    while occupied.len() < *max_y {
        occupied.push(0);
    }
    for i in 0..shape.len() {
        occupied[height + i] |= shape[i];
    }
}

pub fn task1(chars: Linewise<Charwise<Shift>>) -> Result<usize, Error> {
    let shapes = SHAPES.iter().cycle();
    let shifts: Vec<_> = chars.flat_map(|r| r.unwrap()).try_collect2()?;
    let mut shift_index = 0;

    let mut occupied = vec![];
    let mut max_y = 0;

    for shape in shapes.take(2022).map(|s| s.to_vec()) {
        // move the shape 3 units above the highest occupied tile
        drop_shape(shape, &shifts, &mut shift_index, &mut occupied, &mut max_y);
    }

    Ok(max_y)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct CacheKey {
    top_rows: [u8; CACHE_KEY_SIZE],
    shape_index: u16,
    shift_index: u16,
}
const CACHE_KEY_SIZE: usize = 23;

pub fn task2(chars: Linewise<Charwise<Shift>>) -> Result<u64, Error> {
    let shifts: Vec<_> = chars.flat_map(|r| r.unwrap()).try_collect2()?;
    let mut shift_index = 0;

    let mut cache = HashMap::<CacheKey, _>::new();

    let mut occupied = vec![];
    let mut max_y = 0;
    let mut dropped: u64 = 0;

    let collision = loop {
        let shape_index = (dropped % SHAPES.len() as u64) as usize;
        let shape: Vec<u8> = SHAPES[shape_index].to_vec();
        // move the shape 3 units above the highest occupied tile
        drop_shape(shape, &shifts, &mut shift_index, &mut occupied, &mut max_y);
        dropped += 1;

        if occupied.len() < CACHE_KEY_SIZE {
            continue;
        }

        // take the top rows of the occupied tiles as the cache key
        let mut top_rows = [0u8; CACHE_KEY_SIZE];
        let min = occupied.len() - CACHE_KEY_SIZE;
        top_rows.copy_from_slice(&occupied[min..]);

        let key = CacheKey {
            top_rows,
            shape_index: shape_index as u16,
            shift_index: shift_index as u16,
        };
        match cache.entry(key) {
            Entry::Occupied(occ) => break *occ.get(),
            Entry::Vacant(vac) => vac.insert((dropped, max_y)),
        };
    };

    const TOTAL_STEPS: u64 = 1000000000000;
    
    let prev_iter = collision.0;
    let prev_y = collision.1;

    // calculate the number of loops and the growth during each loop
    let loop_iterations = dropped - prev_iter;
    let y_diff = max_y - prev_y;
    let remaining_steps = TOTAL_STEPS - dropped;
    let full_loops = remaining_steps / loop_iterations as u64;
    let loop_growth = full_loops * y_diff as u64;
    
    let remaining_steps = remaining_steps - (full_loops * loop_iterations as u64);

    for _ in 0..remaining_steps {
        let index = (dropped % SHAPES.len() as u64) as usize;
        let shape: Vec<u8> = SHAPES[index].to_vec();
        // move the shape 3 units above the highest occupied tile
        drop_shape(shape, &shifts, &mut shift_index, &mut occupied, &mut max_y);
        dropped += 1;
    }

    dropped += loop_iterations * full_loops;
    assert_eq!(dropped, TOTAL_STEPS);

    Ok(max_y as u64 + loop_growth)
}

#[cfg(test)]
mod tests {
    use common::input::Input;

    use super::*;

    const TEST_INPUT: &[u8] = b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_task1() {
        let chars = Linewise::<Charwise<Shift>>::parse(TEST_INPUT).unwrap();
        let max_y = task1(chars).unwrap();
        assert_eq!(max_y, 3068);
    }
    #[test]
    fn test_task2() {
        let chars = Linewise::<Charwise<Shift>>::parse(TEST_INPUT).unwrap();
        let total = task2(chars).unwrap();
        assert_eq!(total, 1514285714288_u64);
    }

    #[allow(dead_code)]
    fn print_map(occupied: &Vec<u8>, height: usize) {
        let min = occupied.len().checked_sub(height).unwrap_or(0);
        let max = min + height;
        let mut row_str = String::with_capacity(8);
        for y in (min..max).rev() {
            row_str.clear();
            let row = occupied.get(y).cloned().unwrap_or(0);
            for x in (0..WIDTH).rev() {
                if row & (1 << x) != 0 {
                    row_str.push('#');
                } else {
                    row_str.push('.');
                }
            }
            println!("{}", row_str);
        }
    }
}
