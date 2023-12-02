use std::{str::FromStr, cmp::Ordering};

use common::{input::Linewise, iter_ext::TryIterator};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] pattern_parse::ParseError),
}

pattern_parse::parse_fn!(
    parse_line,
    "{u16},{u16} -> {u16},{u16}"
);

pub struct Line {
    from: (u16, u16),
    to: (u16, u16),
}

impl FromStr for Line {
    type Err = pattern_parse::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fx, fy, tx, ty) = parse_line(s)?;
        Ok(Self { from: (fx, fy), to: (tx, ty) })
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }
    fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }

    fn covered_points(&self) -> Vec<(u16,u16)> {
        self.iter().collect()
    }

    fn iter(&self) -> PointsIter {
        let x = match Ord::cmp(&self.from.0, &self.to.0) {
            Ordering::Less => AxisIter::Forward(self.from.0..=self.to.0),
            Ordering::Equal => AxisIter::Const(self.from.0),
            Ordering::Greater => AxisIter::Backward((self.to.0..=self.from.0).rev()),
        };
        let y = match Ord::cmp(&self.from.1, &self.to.1) {
            Ordering::Less => AxisIter::Forward(self.from.1..=self.to.1),
            Ordering::Equal => AxisIter::Const(self.from.1),
            Ordering::Greater => AxisIter::Backward((self.to.1..=self.from.1).rev()),
        };
        PointsIter { x, y }
    }
}

enum AxisIter {
    Forward(std::ops::RangeInclusive<u16>),
    Const(u16),
    Backward(std::iter::Rev<std::ops::RangeInclusive<u16>>),
}

struct PointsIter {
    x: AxisIter,
    y: AxisIter,
}

impl Iterator for PointsIter {
    type Item = (u16, u16);
    fn next(&mut self) -> Option<Self::Item> {
        let x = match &mut self.x {
            AxisIter::Const(n) => *n,
            AxisIter::Forward(i) => i.next()?,
            AxisIter::Backward(i) => i.next()?,
        };
        let y = match &mut self.y {
            AxisIter::Const(n) => *n,
            AxisIter::Forward(i) => i.next()?,
            AxisIter::Backward(i) => i.next()?,
        };
        Some((x,y))
    }
}

pub fn task1(input: Linewise<Line>) -> Result<usize, Error> {
    let mut lines: Vec<_> = input.try_collect2()?;
    lines.retain(|l| l.is_horizontal() || l.is_vertical());
    let points = lines.iter()
        .map(|l| l.covered_points())
        .collect::<Vec<_>>();
    let mut intersections = ahash::HashSet::default();

    for i in 0..points.len()-1 {
        let first = &points[i];
        for j in i + 1..points.len() {
            for p in lines[j].iter() {
                if first.contains(&p) {
                    intersections.insert(p);
                }
            }            
        }
    }

    Ok(intersections.len())
}

pub fn task2(input: Linewise<Line>) -> Result<usize, Error> {
    let lines: Vec<_> = input.try_collect2()?;
    let points = lines.iter()
        .map(|l| l.covered_points())
        .collect::<Vec<_>>();
    let mut intersections = ahash::HashSet::default();

    for i in 0..points.len()-1 {
        let first = &points[i];
        for j in i + 1..points.len() {
            for p in lines[j].iter() {
                if first.contains(&p) {
                    intersections.insert(p);
                }
            }            
        }
    }

    Ok(intersections.len())
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 5);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 12);
    }
}
