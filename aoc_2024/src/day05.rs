use ahash::{HashMap, HashMapExt};
use common::bit_set::BitSet;
use common::input::{LineSeparated, Linewise};
use common::iter_ext::TryIterator;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing delimiter '|' in rule")]
    MissingDelimiter,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub struct Rule {
    before: u8,
    after: u8,
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('|').ok_or(Error::MissingDelimiter)?;
        let before = l.parse()?;
        let after = r.parse()?;
        Ok(Self { before, after })
    }
}

pub struct Update(Vec<u8>);

impl DerefMut for Update {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl Deref for Update {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split(',').map(u8::from_str).try_collect2()?;
        Ok(Self(result))
    }
}

pub fn task1<'a>(
    input: LineSeparated<'a, Linewise<'static, Rule>, Linewise<'a, Update>>,
) -> Result<u32, Error> {
    let (rule_source, update_source) = input.into_inner();

    // rules contains a set of vectors specifying if KEY wants to be printed
    // all entries of VALUE have to have been printed already
    let mut rules: [Vec<u8>; 100] = std::array::from_fn(|_| Vec::new());
    for rule in rule_source {
        let rule = rule?;
        rules[rule.after as usize].push(rule.before);
    }

    let mut total = 0u32;
    let mut printed = BitSet::new(100);
    for update in update_source {
        let update = update?;
        printed.clear();

        if is_safe(&mut rules, &mut printed, &update) == false {
            continue;
        }

        let mid = update.0.len() / 2;
        total += update.0[mid] as u32;
    }

    Ok(total)
}

fn is_safe(rules: &mut [Vec<u8>; 100], printed: &mut BitSet, update: &Update) -> bool {
    for value in update.0.iter().cloned() {
        let before = &rules[value as usize];
        if before
            .iter()
            .all(|i| update.0.contains(i) == false || printed.get(*i as usize))
            == false
        {
            return false;
        }
        printed.set(value as usize);
    }
    true
}

pub fn task2<'a>(
    input: LineSeparated<'a, Linewise<'static, Rule>, Linewise<'a, Update>>,
) -> Result<u32, Error> {
    let (rule_source, update_source) = input.into_inner();

    // rules contains a set of vectors specifying if KEY wants to be printed
    // all entries of VALUE have to have been printed already
    let mut rules: [Vec<u8>; 100] = std::array::from_fn(|_| Vec::new());
    for rule in rule_source {
        let rule = rule?;
        rules[rule.after as usize].push(rule.before);
    }

    let mut total = 0u32;
    let mut printed = BitSet::new(100);
    for update in update_source {
        let mut update = update?;

        printed.clear();
        if is_safe(&mut rules, &mut printed, &update) {
            continue;
        }

        let mut dependencies = HashMap::with_capacity(update.len());
        for value in update.iter().cloned() {
            let required = &rules[value as usize];
            let order = required.iter()
                .filter(|n| update.contains(*n))
                .count();
            dependencies.insert(value, order);
        }

        update.sort_unstable_by_key(|val| dependencies[val]);

        let mid = update.len() / 2;
        total += update[mid] as u32;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 143);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 123);
        panic!()
    }
}
