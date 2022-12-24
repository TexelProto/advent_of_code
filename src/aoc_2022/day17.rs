use std::collections::HashSet;

use crate::input::{chars::{FromChar, Charwise}, lines::Linewise};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown shift char '{0}'")]
    UnknownShift(char)
}

type Point = (u8, u32);
type Shape<'a> = &'a [Point];
type MutShape = Vec<Point>;

const SHAPES: [Shape<'static>;5] = [
    &[(0,0),(1,0),(2,0),(3,0)],
    &[(1,0),(0,1),(1,1),(1,2),(2,1)],
    &[(0,0),(1,0),(2,0),(2,1),(2,2)],
    &[(0,0),(0,1),(0,2),(0,3)],
    &[(0,0),(0,1),(1,0),(1,1)],
];

const WIDTH: u8 = 7;

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
            c => Err(Error::UnknownShift(c))
        }
    }
}

fn encode_point(p: &Point) -> u64 {
    (p.0 as u64) << 32 | (p.1 as u64)
}

fn try_shift(shape: &mut MutShape, shift: Shift, occupied: &HashSet<u64>) {
    match shift {
        Shift::Left => {
            if shape.iter().any(|p| p.0 == 0) {
                return;
            }
            if shape.iter().any(|p| occupied.contains(&encode_point(&(p.0-1, p.1)))){
                return;
            }
            shape.iter_mut().for_each(|p| p.0 -= 1);
        },
        Shift::Right => {
            if shape.iter().any(|p| p.0 == WIDTH - 1) {
                return;
            }
            if shape.iter().any(|p| occupied.contains(&encode_point(&(p.0+1, p.1)))){
                return;
            }
            shape.iter_mut().for_each(|p| p.0 += 1);            
        },
    }
}

fn can_drop(shape: &MutShape, occupied: &HashSet<u64>) -> bool {
    for point in shape {
        if point.1 == 0 {
            return false;
        }

        let down = (point.0, point.1 - 1);
        if occupied.contains(&encode_point(&down)) {
            return false;
        }
    }
    true
}

pub fn task1(chars: Linewise<Charwise<Shift>>) -> Result<u32,Error> {
    let shapes = SHAPES.iter().cycle();
    let shifts = chars.flat_map(|r| r.unwrap()).try_collect::<Vec<_>>()?;
    let mut shifts = shifts.iter().cycle();

    let mut occupied = HashSet::<u64>::new();
    let mut max_y = 0;

    for mut shape in shapes.take(20).map(|s|s.to_vec()) {
        // move the shape 3 units above the highest occupied tile
        shape.iter_mut().for_each(|p| *p = (p.0 + 2, p.1 + max_y + 3));

        loop {
            try_shift(&mut shape, shifts.next().unwrap().clone(), &occupied);

            if can_drop(&shape, &occupied) {
                shape.iter_mut().for_each(|p| p.1 -= 1);
            } else {
                break;
            }
        }

        for point in shape {
            occupied.insert(encode_point(&point));
            max_y = std::cmp::max(point.1+1, max_y);
        }
    }

    Ok(max_y)
}

pub fn task2(chars: Linewise<Charwise<Shift>>) -> Result<u32,Error> {
    let shapes = SHAPES.iter().cycle();
    let shifts = chars.flat_map(|r| r.unwrap()).try_collect::<Vec<_>>()?;
    let mut shifts = shifts.iter().cycle();

    let mut occupied = HashSet::<u64>::new();
    let mut max_y = 0;

    for mut shape in shapes.take(2022).map(|s|s.to_vec()) {
        // move the shape 3 units above the highest occupied tile
        shape.iter_mut().for_each(|p| *p = (p.0 + 2, p.1 + max_y + 3));

        loop {
            try_shift(&mut shape, shifts.next().unwrap().clone(), &occupied);

            if can_drop(&shape, &occupied) {
                shape.iter_mut().for_each(|p| p.1 -= 1);
            } else {
                break;
            }
        }

        for point in shape {
            occupied.insert(encode_point(&point));
            max_y = std::cmp::max(point.1+1, max_y);
        }
    }

    Ok(max_y)
}
