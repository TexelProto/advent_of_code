use std::{
    cmp::Ordering,
    fmt::Debug,
    str::FromStr, num::ParseIntError,
};

use crate::{input::Chunked, common::iter_ext::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    List(Vec<Item>),
    Number(usize),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_lists(l1: &[Item], l2: &[Item]) -> Ordering {
    let min_len = std::cmp::min(l1.len(), l2.len());
    for i in 0..min_len {
        let cmp = l1[i].cmp(&l2[i]);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    l1.len().cmp(&l2.len())
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => n1.cmp(n2),
            (Self::List(l1), Self::List(l2)) => cmp_lists(l1.as_slice(), l2.as_slice()),
            (Self::List(l), Self::Number(n)) => cmp_lists(l.as_slice(), &[Item::Number(*n)]),
            (Self::Number(n), Self::List(l)) => cmp_lists(&[Item::Number(*n)], l.as_slice()),
        };
        result
    }
}

fn parse_list(input: &str, cursor: &mut usize) -> Result<Item, Error>  {
    let mut items = Vec::new();
    *cursor += 1;
    loop {
        if input.as_bytes()[*cursor] == b']' {
            break;
        }
        items.push(parse(input, cursor)?);
        if input.as_bytes()[*cursor] == b']' {
            break;
        }
        *cursor += 1;
    }

    *cursor += 1;
    Ok(Item::List(items))
}

fn parse_number(input: &str, cursor: &mut usize) -> Result<Item, Error> {
    let start = *cursor;
    loop {
        *cursor += 1;
        let c = input.as_bytes()[*cursor] as char;
        if c.is_numeric() == false {
            break;
        }
    }

    let num = input[start..*cursor].parse()?;
    Ok(Item::Number(num))
}

fn parse(input: &str, cursor: &mut usize) -> Result<Item, Error> {
    if input.as_bytes()[*cursor] == b'[' {
        parse_list(input, cursor)
    } else {
        parse_number(input, cursor)
    }
}

impl FromStr for Item {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s, &mut 0)
    }
}

pub fn task1(items: Chunked<Item, 2, true>)  -> Result<usize, Error>  {
    let acc = items
        .enumerate()
        .try_fold(0_usize, |acc, (i, pair)| {
            let pair = pair?;
            let val = if pair[0] < pair[1] {
                i + 1
            } else {
                0
            };
            Ok::<usize, Error>(acc + val)
         })?;
    Ok(acc)
}

pub fn task2(items: Chunked<Item, 2, true>) -> Result<usize, Error> {
    let mut items: Vec<_> = try_collect(try_flatten(items))?;
    let marker1 = Item::List(vec!(Item::List(vec![Item::Number(2)])));
    let marker2 = Item::List(vec!(Item::List(vec![Item::Number(6)])));
    items.push(marker1.clone());
    items.push(marker2.clone());
    items.sort_unstable();
    
    let a = items.binary_search(&marker1).unwrap() + 1;
    let b = items.binary_search(&marker2).unwrap() + 1;
    
    Ok(a*b)
}
