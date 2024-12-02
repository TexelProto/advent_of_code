use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::ptr;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub struct Report(Vec<u32>);

impl FromStr for Report {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(s.is_ascii());
        let vec: Vec<_> = s
            .split_ascii_whitespace()
            .map(u32::from_str)
            .try_collect2()?;
        debug_assert!(vec.len() >= 2);
        Ok(Self(vec))
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let cmp = self.0[0].cmp(&self.0[1]);

        if cmp.is_eq() {
            return false;
        }

        for window in self.0.windows(2) {
            let [a, b] = window else { unreachable!() };
            if Self::is_safe_pair(a, b, cmp) == false {
                return false;
            }
        }
        true
    }

    fn is_safe_pair(a: &u32, b: &u32, cmp: Ordering) -> bool {
        if a.cmp(b) != cmp {
            return false;
        }
        if a.abs_diff(*b) > 3 {
            return false;
        }
        true
    }

    fn is_dampened_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.0.len() {
            if self.is_safe_without(i) {
                return true;
            }
        }

        false
    }

    fn is_safe_without(&self, skip: usize) -> bool {
        let mut direction: Option<Ordering> = None;
        let skipped = &self.0[skip];
        for window in self.0.windows(2) {
            let [a, b] = window else { unreachable!() };

            if ptr::eq(a, skipped) || ptr::eq(b, skipped) {
                continue;
            }

            let cmp = direction.unwrap_or_else(|| {
                let c = a.cmp(b);
                direction = Some(c);
                c
            });

            if Self::is_safe_pair(a, b, cmp) == false {
                return false;
            }
        }

        if skip > 0 && skip < self.0.len() - 1 {
            let prev = &self.0[skip - 1];
            let next = &self.0[skip + 1];

            if Self::is_safe_pair(prev, next, direction.unwrap()) == false {
                return false;
            }
        }

        true
    }
}

pub fn task1(input: Linewise<Report>) -> Result<u32, Error> {
    let mut count = 0;
    for r in input {
        let report = r?;
        if report.is_safe() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn task2(input: Linewise<Report>) -> Result<u32, Error> {
    let mut count = 0;
    for r in input {
        let report = r?;
        if report.is_dampened_safe() {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4);
    }
}
