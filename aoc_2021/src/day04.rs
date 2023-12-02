use std::str::FromStr;

use common::{iter_ext::TryIterator, input::Multiline, debug::BinDebug};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub struct Board {
    numbers: [u8;25],
    marked_mask: u32,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Board")
            .field("numbers", &self.numbers)
            .field("marked_mask", &BinDebug(&self.marked_mask))
            .finish()
    }
}

impl Board {
    pub fn try_mark(&mut self, num: u8) -> bool {
        if let Some(i) = self.numbers.iter()
            .position(move |x| *x == num) 
        {
            self.marked_mask |= 1 << i;
            true
        } else {
            false
        }
    }

    pub fn has_bingo(&self) -> bool {
        // bit mask representing a horizontal row of flags
        const ROW: u32 = 0b_00000_00000_00000_00000_11111;
        // bit mask representing a vertical column of flags
        const COLUMN: u32 = 0b_00001_00001_00001_00001_00001;

        for y in 0..5 {
            let mask = ROW << (5 * y);
            if self.marked_mask & mask == mask { return true; }
        }
        for x in 0..5 {
            let mask = COLUMN << x;
            if self.marked_mask & mask == mask { return true; }
        }

        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        let mut acc = 0_u32;

        for i in 0..25 {
            let mask = 1 << i;
            if self.marked_mask & mask == 0 {
                acc += self.numbers[i] as u32;
            }
        }

        acc
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [0; 25];

        let parsed = s.lines()
        .flat_map(|l| {
            l.split_ascii_whitespace()
            .map(|s| u8::from_str(s.trim()))
        }).enumerate();

        for (i, num) in parsed {
            numbers[i] = num?;
        }

        Ok(Self{numbers, marked_mask: 0})
    }
}

pub struct Input {
    numbers: Vec<u8>,
    boards: Vec<Board>
}

impl<'a> common::input::Input<'a> for Input {
    type Error = Error;

    fn parse<R: 'a + std::io::BufRead>(mut read: R) -> Result<Self, Self::Error> {
        // read the line containing all the called numbers
        let mut line = String::new();
        read.read_line(&mut line)?;
        let numbers =
            line.split(',')
            .map(|s| u8::from_str(s.trim()))
            .try_collect2()?;

        // blank line separating 
        line.clear();
        read.read_line(&mut line)?;

        let multiline = Multiline::<Board, 5, true>::parse(read)
            .unwrap();
        let boards = multiline.try_collect2()?;

        Ok(Self { numbers, boards })
    }
}

pub fn task1(input: Input) -> Result<u32, Error> {
    let mut boards = input.boards;
    let mut winning = None;

    'outer: for num in input.numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.try_mark(num) && board.has_bingo() {
                winning = Some((i, num));
                break 'outer;
            }
        }
    }

    let (board_index, num) = winning.expect("Failed to find bingo");
    let board = &boards[board_index];

    let score = board.unmarked_sum() * (num as u32);
    Ok(score)
}

pub fn task2(input: Input) -> Result<u32, Error> {
    let mut numbers = input.numbers.into_iter();
    let mut boards = input.boards;

    for num in &mut numbers {
        boards.retain_mut(move |board| {
            let done = board.try_mark(num) & board.has_bingo();
            !done
        });
        if boards.len() == 1 { break; }
    }
    assert_eq!(boards.len(), 1, "Failed to find last board");

    let board = &mut boards[0];
    let mut winning = None;
    for num in &mut numbers {
        if board.try_mark(num) && board.has_bingo() {
            winning = Some(num);
            break;
        }
    }

    let num = winning.expect("Failed to find bingo");
    let score = board.unmarked_sum() * (num as u32);
    Ok(score)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4512);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1924);
    }
}
