use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Sand,
}

const SPAWN_POINT_X: usize = 500;
const SPAWN_POINT_Y: usize = 0;
const SPAWN_POINT: (usize, usize) = (SPAWN_POINT_X, SPAWN_POINT_Y);
const WIDTH: usize = 800;
const HEIGHT: usize = 200;

type Map = [[Tile; HEIGHT]; WIDTH];

fn parse<'a>(input: &'a str) -> impl Iterator<Item=impl 'a + Iterator<Item=(usize, usize)>> {
    input.lines().map(|line: &str| {
        line.trim().split(" -> ").map(|part: &str| {
            let i = part.find(',').unwrap();
            let (x, y) = part.split_at(i);

            let x = usize::from_str(x).unwrap();
            let y = usize::from_str(&y[1..]).unwrap();

            (x, y)
        })
    })
}

fn create_map(walls: impl Iterator<Item=impl Iterator<Item=(usize, usize)>>) -> Map {
    let mut map = [[Tile::Empty; HEIGHT]; WIDTH];
    for wall in walls {
        let points = wall.collect::<Vec<_>>();
        for pair in points.windows(2) {
            let (x0, y0) = pair[0];
            let (x1, y1) = pair[1];
            let x_min = min(x0, x1);
            let y_min = min(y0, y1);
            let x_max = max(x0, x1);
            let y_max = max(y0, y1);

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    map[x][y] = Tile::Wall;
                }
            }
        }
    }
    map
}

struct OutOfBoundsError;

fn drop_point(map: &Map, (x, y): (usize, usize)) -> Result<Option<(usize, usize)>, OutOfBoundsError> {
    let y = y + 1;

    for x_offset in [0, -1, 1] {
        let x = match x.checked_add_signed(x_offset) {
            None => continue,
            Some(x) => x,
        };

        if x >= WIDTH || y >= HEIGHT {
            return Err(OutOfBoundsError);
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

fn drop_sand_particle(map: &Map) -> Result<(usize, usize), OutOfBoundsError> {
    let mut point = SPAWN_POINT;
    while let Some(new_point) = drop_point(&map, point)? {
        point = new_point;
    }
    Ok(point)
}

pub fn task1(input: String) {
    let walls = parse(&input[3..]);
    let mut map = create_map(walls);

    let mut count = 0_u64;
    loop {
        if map[SPAWN_POINT_X][SPAWN_POINT_Y] != Tile::Empty {
            break;
        }

        let point = match drop_sand_particle(&map) {
            Ok(point) => point,
            Err(_) => break,
        };

        map[point.0][point.1] = Tile::Sand;
        count += 1;
    }

    dbg!(count);
}

pub fn task2(input: String) {
    let walls = parse(&input[3..]);
    let mut map = create_map(walls);

    let floor_height = (0..HEIGHT).rev().filter(|y| {
        (0..WIDTH).any(move |x| map[x][*y] == Tile::Wall)
    }).next().unwrap() + 2;

    (0..WIDTH).for_each(|x| {
        map[x][floor_height] = Tile::Wall;
    });

    let mut count = 0_u64;
    loop {
        if map[SPAWN_POINT_X][SPAWN_POINT_Y] != Tile::Empty {
            break;
        }

        let point = match drop_sand_particle(&map) {
            Ok(point) => point,
            Err(_) => panic!("Sand falling outside of the simulation should be impossible due to the floor"),
        };

        map[point.0][point.1] = Tile::Sand;
        count += 1;
    }

    println!("count = {}", count);
}