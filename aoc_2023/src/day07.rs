use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;
use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Encountered invalid card character '{0}'")]
    InvalidCardChar(char),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

common::num_enum! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    enum Card {
        Joker,
        Number2,
        Number3,
        Number4,
        Number5,
        Number6,
        Number7,
        Number8,
        Number9,
        Number10,
        Jack,
        Queen,
        King,
        Ace
    }
}

fn parse_card(value: char, joker: bool) -> Result<Card, Error> {
    match value {
        '2' => Ok(Card::Number2),
        '3' => Ok(Card::Number3),
        '4' => Ok(Card::Number4),
        '5' => Ok(Card::Number5),
        '6' => Ok(Card::Number6),
        '7' => Ok(Card::Number7),
        '8' => Ok(Card::Number8),
        '9' => Ok(Card::Number9),
        'T' => Ok(Card::Number10),
        'J' => match joker {
            false => Ok(Card::Jack),
            true => Ok(Card::Joker),
        }
        'Q' => Ok(Card::Queen),
        'K' => Ok(Card::King),
        'A' => Ok(Card::Ace),
        c => Err(Error::InvalidCardChar(c))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_cmp = self.kind.cmp(&other.kind);
        if kind_cmp != Ordering::Equal {
            return kind_cmp;
        }

        for i in 0..5 {
            let card_cmp = self.cards[i].cmp(&other.cards[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        Ordering::Equal
    }
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut hands = vec![];
    for line in input {
        let line = line.unwrap();
        let mut cards = [Card::Ace; 5];
        for (i, c) in line.chars().take(5).enumerate() {
            cards[i] = parse_card(c, false)?;
        }
        let score = u32::from_str(&line[6..])?;

        let mut occurences = [0; Card::COUNT];
        for c in cards {
            occurences[c as u8 as usize] += 1;
        }

        let kind = identify(&occurences);
        hands.push((Hand { cards, kind }, score));
    }
    hands.sort_by_key(|(hand, _)| *hand);
    let score: u32 = hands.into_iter().enumerate().map(|(index, (_hand, bid))| (index as u32 + 1) * bid).sum();
    Ok(score)
}

pub fn task2(input: Linewise<String>) -> Result<u32, Error> {
    let mut hands = vec![];
    for line in input {
        let line = line.unwrap();
        let mut cards = [Card::Ace; 5];
        for (i, c) in line.chars().take(5).enumerate() {
            cards[i] = parse_card(c, true)?;
        }
        let score = u32::from_str(&line[6..])?;

        let mut occurences = [0; Card::COUNT];
        for c in cards {
            occurences[c as u8 as usize] += 1;
        }

        let jokers = occurences[Card::Joker as u8 as usize];
        occurences[Card::Joker as u8 as usize] = 0;

        let index_of_max = occurences
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();


        occurences[index_of_max] += jokers;

        let kind = identify(&occurences);
        hands.push((Hand { cards, kind }, score));
    }
    hands.sort_by_key(|(hand, _)| *hand);
    let score: u32 = hands.into_iter().enumerate().map(|(index, (_hand, bid))| (index as u32 + 1) * bid).sum();
    Ok(score)
}

fn identify(counts: &[u8; Card::COUNT]) -> HandKind {
    if counts.contains(&5) { return HandKind::FiveOfAKind; }
    if counts.contains(&4) { return HandKind::FourOfAKind; }
    if counts.contains(&3) {
        if counts.contains(&2) {
            return HandKind::FullHouse;
        }
        return HandKind::ThreeOfAKind;
    }
    let twos = counts.iter().filter(|c| **c == 2).count();
    match twos {
        2 => HandKind::TwoPair,
        1 => HandKind::OnePair,
        _ => HandKind::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 6440);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 5905);
    }
}
