use crate::{
    common::iter_ext::try_collect,
    input::{
        chars::{Charwise, FromChar},
        lines::Linewise,
    },
};

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
    shifts: &mut impl Iterator<Item = Shift>,
    occupied: &mut Vec<u8>,
    max_y: &mut usize,
) {
    let mut height = *max_y + 3;
    loop {
        try_shift(
            &mut shape,
            height,
            shifts.next().unwrap().clone(),
            &*occupied,
        );

        if height == 0 {
            break;
        }

        if shape_intersects(&shape, height - 1, 0, &*occupied) {
            break;
        }
        // let mut clone = occupied.clone();
        // while clone.len() < height + shape.len() {
        //     clone.push(0);
        // }
        // for i in 0..shape.len() {
        //     clone[height + i] |= shape[i];
        // }

        // print_map(&clone, 8);
        // println!();

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
    let shifts: Vec<_> = try_collect(chars.flat_map(|r| r.unwrap()))?;
    let mut shifts = shifts.into_iter().cycle();

    let mut occupied = vec![];
    let mut max_y = 0;

    for shape in shapes.take(2022).map(|s| s.to_vec()) {
        // move the shape 3 units above the highest occupied tile
        drop_shape(shape, &mut shifts, &mut occupied, &mut max_y);
    }

    Ok(max_y)
}
#[cfg(test)]
mod tests {
    use crate::input::Input;

    use super::*;

    const TEST_INPUT: &[u8] = b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_task1() {
        let chars = Linewise::<Charwise<Shift>>::parse(TEST_INPUT).unwrap();
        let shifts: Vec<_> = try_collect(chars.flat_map(|r| r.unwrap())).unwrap();
        let mut shifts = shifts.into_iter().cycle();
        let shapes = SHAPES.iter().cycle();

        let mut occupied = vec![];
        let mut max_y = 0;

        for shape in shapes.take(2022).map(|s| s.to_vec()) {
            // move the shape 3 units above the highest occupied tile
            drop_shape(shape, &mut shifts, &mut occupied, &mut max_y);

            print_map(&occupied, 8);
            println!();
        }

        assert_eq!(max_y, 3068);
    }

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
