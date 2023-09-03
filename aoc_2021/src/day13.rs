use std::str::FromStr;

use common::{input::{LineSeparated, Linewise}, geometry_2d::{Point, PointParseError}};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PointParse(#[from] PointParseError),
    #[error(transparent)]
    FoldParse(#[from] pattern_parse::ParseError),
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Horizontal(u32),
    Vertical(u32),
}

impl Fold {
    pub fn fold(&self, point: &mut Point) {
        match *self {
            Fold::Horizontal(center) => {
                point.x = if point.x > center { center - (point.x - center) } else { point.x };
            },
            Fold::Vertical(center) => {
                point.y = if point.y > center { center - (point.y - center) } else { point.y };
            }
        }
    }
}

impl FromStr for Fold {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pattern_parse::parse_fn!(
            parse,
            "fold along {char}={u32}"
        );

        let (c, line) = parse(s)?;
        match c {
            'x' => Ok(Fold::Horizontal(line)),
            'y' => Ok(Fold::Vertical(line)),
            _ => unreachable!(),
        }
    }
}

pub fn task1<'a>(input: LineSeparated<'a, Linewise<'static, Point>, Linewise<'a, Fold>>) -> Result<usize, Error> {
    let (points, folds) = input.into_inner();
    let mut point_set = vec![];
    for point in points {
        point_set.push(point?);
    }

    for fold in folds.take(1) {
        let fold = fold?;
        for point in point_set.iter_mut() {
            fold.fold(point);
        }
    }

    // dedup points 
    let mut result = vec![];
    for point in point_set {
        if !result.contains(&point) {
            result.push(point);
        }
    }

    Ok(result.len())
}

fn print_map(point_set: &Vec<Point>) -> String {
    let max_x = point_set.iter().map(|p| p.x).max().unwrap();
    let max_y = point_set.iter().map(|p| p.y).max().unwrap();

    let mut text = String::new();
    text.push('\n');
    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            let c = if point_set.contains(&p) {
                '#'
            } else {
                ' '
            };
            text.push(c);
        }
        text.push('\n');
    }
    text
}

pub fn task2<'a>(input: LineSeparated<'a, Linewise<'static, Point>, Linewise<'a, Fold>>) -> Result<String, Error> {
    let (points, folds) = input.into_inner();
    let mut point_set = vec![];
    for point in points {
        point_set.push(point?);
    }

    for fold in folds {
        let fold = fold?;
        for point in point_set.iter_mut() {
            fold.fold(point);
        }
    }

    let str = print_map(&point_set);
    Ok(str)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 17);
    }
}
