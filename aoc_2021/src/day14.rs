use std::str::FromStr;

use common::{
    input::{LineSeparated, Linewise},
    iter_ext::TryIterator
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(#[from] pattern_parse::ParseError),
}

pub struct Rule {
    pub pair: [u8; 2],
    pub insert: u8,
}

impl FromStr for Rule {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pattern_parse::parse_fn! {
            parse, "{char}{char} -> {char}"
        }

        let (a, b, insert) = parse(s)?;
        Ok(Self {
            pair: [a as u8, b as u8],
            insert: insert as u8,
        })
    }
}

pub fn task1<'a>(input: LineSeparated<'a, String, Linewise<'a, Rule>>) -> Result<i32, Error> {
    let (line, rules) = input.into_inner();
    let mut line = line.into_bytes();
    let rules: Vec<_> = rules.try_collect2()?;

    for _ in 0..10 {
        let mut next = vec![];

        for window in line.windows(2) {
            next.push(window[0]);
            for rule in rules.iter() {
                if rule.pair == window {
                    next.push(rule.insert);
                    break;
                }
            }
        }

        next.push(*line.last().unwrap());
        line = next;
    }

    let mut char_count = vec![];
    for char in line {
        match char_count.iter().position(|(c, _)| *c == char) {
            Some(i) => char_count[i].1 += 1,
            None => char_count.push((char, 1)),
        }
    }

    char_count.sort_by(|a, b| b.1.cmp(&a.1));
    let most = char_count[0];
    let least = char_count[char_count.len() - 1];
    Ok(most.1 - least.1)
}

pub fn task2<'a>(input: LineSeparated<'a, String, Linewise<'a, Rule>>) -> Result<u64, Error> {
    let (line, rules) = input.into_inner();
    let line = line.into_bytes();
    let rules: Vec<_> = rules.try_collect2()?;

    // divide the input into pairs (keep in mind the pairs overlap)
    // ie NNCB => [NN, NC, CB]
    let mut pairs = line
        .windows(2)
        .map(|w| {
            let mut array = [0; 2];
            array.copy_from_slice(w);
            (array, 1_u64)
        })
        .collect::<ahash::HashMap<_, _>>();

    // count occurences of each char
    let mut char_count = vec![];
    for char in line.clone() {
        match char_count.iter_mut().find(|(c, _)| *c == char) {
            Some(entry) => entry.1 += 1,
            None => char_count.push((char, 1)),
        }
    }

    for _ in 0..40 {
        let mut next = ahash::HashMap::default();
        for (pair, count) in pairs.iter() {
            match rules.iter().find(|r| r.pair == *pair) {
                Some(rule) => {
                    // break down the pair into two pairs
                    // ie AB -> C makes (A, C) and (C, B)
                    let a = [pair[0], rule.insert];
                    *next.entry(a).or_default() += count;

                    let b = [rule.insert, pair[1]];
                    *next.entry(b).or_default() += count;

                    // update the char count
                    match char_count.iter_mut().find(|(c, _)| *c == rule.insert) {
                        Some(t) => t.1 += count,
                        None => char_count.push((rule.insert, *count)),
                    };
                }
                None => {
                    *next.entry(*pair).or_default() += count;
                }
            };
        }
        pairs = next;
    }

    // sort the char count
    char_count.sort_by_cached_key(|(_, count)| *count);

    let most = char_count[0].1;
    let least = char_count[char_count.len() - 1].1;

    let result = least - most;
    // assert!(dbg!(result) > 3277772741534u64);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
        .as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1588);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2188189693529_u64);
    }
}
