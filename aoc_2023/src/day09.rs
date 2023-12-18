use std::num::ParseIntError;
use common::input::{Linewise, SpaceSeparated};
use common::iter_ext::TryIterator;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn task1(input: Linewise<SpaceSeparated<i32>>) -> Result<i32, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();
        let numbers = line.try_collect2::<Vec<_>>()?;

        let mut derivatives = vec![numbers];
        build_derivatives(&mut derivatives);

        let mut change = 0;
        for row in derivatives.into_iter().rev() {
            let value = row.last().unwrap();
            change += *value;
        }

        total += change;
    }

    Ok(total)
}

pub fn task2(input: Linewise<SpaceSeparated<i32>>) -> Result<i32, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();
        let numbers = line.try_collect2::<Vec<_>>()?;

        let mut derivatives = vec![numbers];
        build_derivatives(&mut derivatives);

        let mut change = 0;
        for row in derivatives.into_iter().rev() {
            let value = row[0];
            change = value - change;
        }

        total += change;
    }

    Ok(total)
}

fn build_derivatives(numbers: &mut Vec<Vec<i32>>) {
    loop {
        let top = numbers.last().unwrap();
        let count = top.len() - 1;
        let mut new = Vec::with_capacity(count);
        for i in 0..count {
            new.push(top[i + 1] - top[i]);
        }

        if new.iter().all(|n| *n == 0) {
            break;
        }

        numbers.push(new);
    }
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 114);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2);
    }
}
