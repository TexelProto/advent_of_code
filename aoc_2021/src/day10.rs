use common::input::{FromChar, Linewise, Charwise};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Character: {0}")]
    InvalidChar(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BracketKind {
    /// ()
    Round,
    /// []
    Corner,    
    /// {}
    Curly,
    /// <>
    Angle,
}

impl BracketKind {
    fn corruption_score(self) -> u32 {
        match self {
            BracketKind::Round => 3,
            BracketKind::Corner => 57,
            BracketKind::Curly => 1197,
            BracketKind::Angle => 25137,
        }
    }

    fn completion_score(self) -> u32 {
        match self {
            BracketKind::Round => 1,
            BracketKind::Corner => 2,
            BracketKind::Curly => 3,
            BracketKind::Angle => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bracket {
    kind: BracketKind,
    closing: bool,
}

impl Bracket {
    fn opening(kind: BracketKind) -> Bracket {
        Bracket { kind, closing: false }
    }
    fn closing(kind: BracketKind) -> Bracket {
        Bracket { kind, closing: true }
    }
}


impl FromChar for Bracket {
    type Err = Error;
    fn from_char(c: char) -> Result<Self, Self::Err> {
        let b = match c {
            '(' => Self::opening(BracketKind::Round),
            ')' => Self::closing(BracketKind::Round),
            '[' => Self::opening(BracketKind::Corner),
            ']' => Self::closing(BracketKind::Corner),
            '{' => Self::opening(BracketKind::Curly),
            '}' => Self::closing(BracketKind::Curly),
            '<' => Self::opening(BracketKind::Angle),
            '>' => Self::closing(BracketKind::Angle),
            _ => return Err(Error::InvalidChar(c)),
        };
        Ok(b)
    }
}

pub fn task1(input: Linewise<Charwise<Bracket>>) -> Result<u32, Error> {
    let mut total = 0;
    'lines: for i in input {
        let i = i.unwrap();
        let mut stack = vec![];

        for b in i {
            let b = b?;
            if !b.closing {
                stack.push(b);
                continue;
            }
    
            let open = stack.pop().unwrap();
            if b.kind != open.kind {
                total += b.kind.corruption_score();
                continue 'lines;
            }    
        }
    }
    Ok(total)
}

pub fn task2(input: Linewise<Charwise<Bracket>>) -> Result<u64, Error> {
    let mut scores = vec![];
    'lines: for i in input {
        let i = i.unwrap();
        let mut stack = vec![];

        for b in i {
            let b = b?;
            if !b.closing {
                stack.push(b);
                continue;
            }
    
            let open = stack.pop().unwrap();
            if b.kind != open.kind {
                // corrupted lines are now ignored
                continue 'lines;
            }    
        }

        let score = stack.into_iter()
            .rev()
            .fold(0, |acc, b| {
            acc * 5 + b.kind.completion_score() as u64
        });
        let index = match scores.binary_search(&score){
            Ok(i) => i,
            Err(i) => i,
        };
        scores.insert(index, score);
    }
    let mid = scores.len() / 2;
    Ok(scores[mid])
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 26397);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 288957);
    }
}
