use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;
use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Number {
    value: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line.cmp(&other.line).then(self.start.cmp(&other.start))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Symbol {
    value: u8,
    line: usize,
    pos: usize,
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line.cmp(&other.line).then(self.pos.cmp(&other.pos))
    }
}

fn parse_input(input: Linewise<String>) -> Result<(Vec<Number>, Vec<Symbol>), Error> {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for (i, line) in input.enumerate() {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        let mut cursor = 0;

        while cursor < line.len() {
            let c = bytes[cursor];
            if c == b'.' {
                cursor += 1;
            } else if is_digit(c) {
                let num_len = bytes[cursor..].iter().take_while(|c| is_digit(**c)).count();
                let num = u32::from_str(&line[cursor..cursor + num_len])?;
                numbers.push(Number { value: num, line: i, start: cursor, end: cursor + num_len - 1 });
                cursor += num_len;
            } else {
                symbols.push(Symbol { value: bytes[cursor], line: i, pos: cursor });
                cursor += 1;
            }
        }
    }
    Ok((numbers, symbols))
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let (numbers, symbols) = parse_input(input)?;

    let mut total = 0;
    let mut neighbors = vec![];
    for symbol in symbols {
        neighbors.clear();
        if let Some(prev) = symbol.line.checked_sub(1) {
            find_neighbors_in_row(&numbers, prev, symbol.pos, &mut neighbors);
        }
        find_neighbors_in_row(&numbers, symbol.line, symbol.pos, &mut neighbors);
        find_neighbors_in_row(&numbers, symbol.line + 1, symbol.pos, &mut neighbors);

        for i in neighbors.iter() {
            total += numbers[*i].value;
        }
    }

    Ok(total)
}

/// Searches for numbers that overlap pos or are off by one. Found indices are added to `out`
fn find_neighbors_in_row(numbers: &Vec<Number>, line: usize, pos: usize, out: &mut Vec<usize>) {
    // make up a number to search for
    let target = Number { value: 0, line, start: pos, end: pos };

    match numbers.binary_search(&target) {
        Ok(i) => {
            // found number starting right at pos there can be no other neighbors i.e.
            // ..12
            //   ^
            //  pos
            out.push(i);
        }
        Err(i) => {
            // i is the index of the first number starting after pos. i might be the right neighbor still
            // ...1
            //   ^
            //  pos
            let num_i = &numbers[i];
            if num_i.line == line && num_i.start == pos + 1 {
                out.push(i);
            }

            // there can still be a number before i
            if let Some(prev) = i.checked_sub(1) {
                // check if the end is no more then 1 unit away in any direction
                // this only works since numbers are at most 3 digits long
                // for longer numbers it would be possible a number starts and ends "out of view"
                let num_prev = numbers[prev];
                let diff = num_prev.end.abs_diff(pos);
                if num_prev.line == line && diff <= 1 {
                    out.push(prev);
                }
            }
        }
    }
}

fn is_digit(c: u8) -> bool {
    (c as char).is_digit(10)
}

pub fn task2(input: Linewise<String>) -> Result<u32, Error> {
    let (numbers, symbols) = parse_input(input)?;

    let mut total = 0;
    let mut neighbors = vec![];
    for symbol in symbols {
        if symbol.value != b'*' {
            continue;
        }

        neighbors.clear();
        if let Some(prev) = symbol.line.checked_sub(1) {
            find_neighbors_in_row(&numbers, prev, symbol.pos, &mut neighbors);
        }
        find_neighbors_in_row(&numbers, symbol.line, symbol.pos, &mut neighbors);
        find_neighbors_in_row(&numbers, symbol.line + 1, symbol.pos, &mut neighbors);

        if neighbors.len() != 2 {
            continue;
        }

        total += numbers[neighbors[0]].value * numbers[neighbors[1]].value;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4361);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 467835);
    }
}
