use std::ops::BitAnd;
use std::str::FromStr;
use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing delimiter '{0}' in input line")]
    MissingDelimiter(char),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

/// Container for (at least) 100 bit, to represent a bitmask of all mentioned numbers
#[derive(Debug, Default, Copy, Clone)]
struct Bits([u64; 2]);

impl Bits {
    pub fn get(&self, i: usize) -> bool {
        let bucket = &self.0[i / 64];
        let bit = i % 64;
        let mask = 1 << bit;
        bucket & mask != 0
    }
    pub fn set(&mut self, i: usize) {
        let bucket = &mut self.0[i / 64];
        let bit = i % 64;
        let mask = 1 << bit;
        *bucket |= mask;
    }

    pub fn count_ones(self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones()
    }
}

impl BitAnd for Bits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let first = self.0[0] & rhs.0[0];
        let second = self.0[1] & rhs.0[1];
        Self([first, second])
    }
}

fn parse_card(line: &str) -> Result<(Bits, Bits), Error> {
    // skip the Card id section
    let numbers = line.split_once(": ")
        .ok_or(Error::MissingDelimiter(':'))?.1;

    // split the number into winning numbers and 'numbers you have'
    let (win_str, owned_str) = numbers.split_once(" | ")
        .ok_or(Error::MissingDelimiter('|'))?;

    // convert the numbers into bit masks
    let mut winning = Bits::default();
    for i in win_str.split_whitespace().map(usize::from_str) {
        winning.set(i?);
    }

    let mut owned = Bits::default();
    for i in owned_str.split_whitespace().map(usize::from_str) {
        owned.set(i?);
    }
    Ok((winning, owned))
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();

        let (winning, owned) = parse_card(&line)?;

        let matches = (winning & owned).count_ones();
        if let Some(shift) = matches.checked_sub(1) {
            total += 1 << shift;
        }

    }
    Ok(total)
}

pub fn task2(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0;
    // array containing the number of times the next 10 numbers will be copied
    // since there are only 10 winning numbers copies will never be considered more than 10 cards ahead
    let mut next_copies = [0;10];
    for line in input {
        let line = line.unwrap();

        // count the current card and its copies
        let card_count = 1 + next_copies[0];
        total += card_count;
        // clear the number of copies for the current elemetn and move the number of future copies over
        next_copies[0] = 0;
        next_copies.rotate_left(1);

        let (winning, owned) = parse_card(&line)?;

        // add copies gained by the matches
        let matches = (winning & owned).count_ones();
        for i in 0..matches {
            next_copies[i as usize] += card_count;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 13);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 30);
    }
}
