use std::cmp::{max, min};
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use crate::input::{Input, parse_lines, Reader};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Sand,
}

const SPAWN_POINT_X: usize = 500;
const SPAWN_POINT_Y: usize = 0;
const SPAWN_POINT: (usize, usize) = (SPAWN_POINT_X, SPAWN_POINT_Y);
const WIDTH: usize = 800;
const HEIGHT: usize = 200;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Point left the bounds of the world")]
    OutOfBoundsError,
}

pub struct Map([[Tile; HEIGHT]; WIDTH]);

impl Deref for Map {
    type Target = [[Tile; HEIGHT]; WIDTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Input for Map {
    type Error = Error;
    fn parse(mut read: Reader) -> Result<Self, Self::Error> {
        let mut data = [[Tile::Empty; HEIGHT]; WIDTH];
        
        parse_lines::<Error>(&mut read, |line| {
            let points = parse_line(line).try_collect::<Vec<_>>()?;
            for pair in points.windows(2) {
                let (x0, y0) = pair[0];
                let (x1, y1) = pair[1];
                let x_min = min(x0, x1);
                let y_min = min(y0, y1);
                let x_max = max(x0, x1);
                let y_max = max(y0, y1);

                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        data[x][y] = Tile::Wall;
                    }
                }
            }
            Ok(())
        })?;
        
        Ok(Self(data))
    }
}

fn parse_line<'a>(line: &'a str) -> impl 'a + Iterator<Item=Result<(usize, usize), Error>> {
    line.trim_start_matches('\u{feff}').split(" -> ").map(|s: &str| {
        let i = s.find(',').unwrap();
        let (x, y) = s.split_at(i);

        let x = usize::from_str(x)?;
        let y = usize::from_str(&y[1..])?;

        Ok((x, y))
    })
}

fn drop_point(map: &Map, (x, y): (usize, usize)) -> Result<Option<(usize, usize)>, Error> {
    let y = y + 1;

    for x_offset in [0, -1, 1] {
        let x = match x.checked_add_signed(x_offset) {
            None => continue,
            Some(x) => x,
        };

        if x >= WIDTH || y >= HEIGHT {
            return Err(Error::OutOfBoundsError);
        }

        if map[x][y] == Tile::Empty {
            return Ok(Some((x, y)));
        }
    }
    Ok(None)
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let mut string = String::with_capacity(WIDTH);
    for y in 0..HEIGHT {
        string.clear();
        for x in 0..WIDTH {
            string.push(match map[x][y] {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Sand => 'o',
            })
        }
        println!("{}", string);
    }
    println!();
    println!();
    println!();
}

fn drop_sand_particle(map: &Map) -> Result<(usize, usize), Error> {
    let mut point = SPAWN_POINT;
    while let Some(new_point) = drop_point(&map, point)? {
        point = new_point;
    }
    Ok(point)
}

pub fn task1(mut map: Map) -> Result<u64, Error>{
    let mut count = 0_u64;
    loop {
        if map[SPAWN_POINT_X][SPAWN_POINT_Y] != Tile::Empty {
            break;
        }

        let point = match drop_sand_particle(&map) {
            Ok(point) => point,
            Err(e) => match e { 
                Error::OutOfBoundsError => break,
                _ => return Err(e),
            },
        };

        map[point.0][point.1] = Tile::Sand;
        count += 1;
    }

    Ok(count)
}

pub fn task2(mut map: Map) -> Result<u64, Error>{
    let floor_height = (0..HEIGHT).rev().filter(|y| {
        (0..WIDTH).any(|x| map[x][*y] == Tile::Wall)
    }).next().unwrap() + 2;

    (0..WIDTH).for_each(|x| {
        map[x][floor_height] = Tile::Wall;
    });

    let mut count = 0_u64;
    loop {
        if map[SPAWN_POINT_X][SPAWN_POINT_Y] != Tile::Empty {
            break;
        }

        let point = drop_sand_particle(&map)?;

        map[point.0][point.1] = Tile::Sand;
        count += 1;
    }

    Ok(count)
}