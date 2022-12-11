use std::collections::HashSet;

#[derive(Debug)]
struct Compartment<'a>(&'a str);

impl<'a> Compartment<'a> {
    fn from_line(bag: &'a str) -> Self {
        Compartment(bag)
    }

    fn from_bag(bag: &'a str) -> (Self, Self) {
        let middle = bag.len() / 2;
        let parts = bag.split_at(middle);
        (Compartment(parts.0), Compartment(parts.1))
    }
}

fn get_duplicates<'a>(all: impl IntoIterator<Item = &'a Compartment<'a>>) -> Vec<char> {
    let mut iter = all.into_iter();
    let mut set = match iter.next() {
        Some(c) => c.0.chars().collect::<HashSet<_>>(),
        None => return Vec::new(),
    };
    while let Some(c) = iter.next() {
        let other = c.0.chars().collect::<HashSet<_>>();
        set = HashSet::intersection(&set, &other)
            .cloned()
            .collect::<HashSet<_>>();
    }
    set.into_iter().collect::<Vec<_>>()
}

fn score_item(c: char) -> usize {
    let score = if c.is_uppercase() {
        (c as u8) - ('A' as u8) + 27
    } else {
        (c as u8) - ('a' as u8) + 1
    };
    score as usize
}

pub fn task1(input: String) {
    let score = input
            .lines()
            .map(Compartment::from_bag)
            .flat_map(|(c1, c2)| get_duplicates([&c1, &c2]))
            .map(score_item)
            .sum::<usize>();

        println!("Score: {}", score);
    }
    pub fn task2(input: String) {
        let score = input
            .lines()
            .map(Compartment::from_line)
            .collect::<Vec<_>>()
            .chunks(3)
            .flat_map(get_duplicates)
            .map(score_item)
            .sum::<usize>();

        println!("Score: {}", score);
    }
