use std::{cell::RefCell, str::FromStr};
use std::convert::Infallible;

struct Move {
    source: usize,
    destination: usize,
    depth: usize,
}

fn parse<'a>(input: &'a str) -> (Vec<RefCell<Vec<char>>>, impl 'a + Iterator<Item=Move>) {
    let cols = parse_columns(input);
    let moves = parse_moves(input);
    (cols, moves)
}

fn parse_columns(input: &str) -> Vec<RefCell<Vec<char>>> {
    let lines = input.lines().take_while(|s| s.trim().len() > 0);
    let last = lines.clone().last().unwrap();
    let width = last.len() / 4 + 1;

    let stacks = vec![RefCell::new(Vec::<char>::new()); width];
    for line in lines {
        for (col, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                // always insert at the bottom of the stack (reading top down)
                stacks[col].borrow_mut().insert(0, c);
            }
        }
    }

    stacks
}

fn parse_moves<'a>(input: &'a str) -> impl 'a + Iterator<Item=Move> {
    input
        .lines()
        .skip_while(|s| s.trim().len() > 0)
        .skip(1) // ignore the blank line separator
        .map(|s| {
            let mut split = s.split_whitespace();
            split.next().unwrap(); // move
            let depth = usize::from_str(split.next().unwrap()).unwrap();
            split.next().unwrap(); // from
            let source = usize::from_str(split.next().unwrap()).unwrap();
            split.next().unwrap(); // to
            let destination = usize::from_str(split.next().unwrap()).unwrap();

            Move {
                source,
                destination,
                depth,
            }
        })
}

pub fn task1(input: String) -> Result<String, Infallible> {
    let (columns, moves) = parse(&input);
    for m in moves {
        let mut source = columns[m.source - 1].borrow_mut();
        let first = source.len() - m.depth;
        let drain = source.drain(first..);

        let mut destination = columns[m.destination - 1].borrow_mut();
        destination.extend(drain.rev());
    }
    let chars = columns
        .iter()
        .map(|v| *v.borrow().last().unwrap())
        .collect::<String>();
    Ok(chars)
}

pub fn task2(input: String) -> Result<String, Infallible> {
    let (columns, moves) = parse(&input);
    for m in moves {
        let mut source = columns[m.source - 1].borrow_mut();
        let first = source.len() - m.depth;
        let drain = source.drain(first..);

        let mut destination = columns[m.destination - 1].borrow_mut();
        destination.extend(drain);
    }
    let chars = columns
        .iter()
        .map(|v| *v.borrow().last().unwrap())
        .collect::<String>();
    Ok(chars)
}
