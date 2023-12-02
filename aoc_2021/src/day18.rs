use std::{fmt::Display, str::FromStr};
use std::fmt::Write;

use common::input::Linewise;
use common::iter_ext::TryIterator;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone)]
pub struct SnailfishTree {
    root: SnailfishNode,
}

#[derive(Debug, Clone)]
pub enum SnailfishNode {
    Number(u32),
    Pair(Box<[SnailfishNode; 2]>),
}

impl SnailfishNode {
    fn is_number(&self) -> bool {
        matches!(self, SnailfishNode::Number(_))
    }

    fn get_pair_numbers(&mut self) -> Option<(u32, u32)> {
        match self {
            SnailfishNode::Pair(a) => match (&a[0], &a[1]) {
                (SnailfishNode::Number(x), SnailfishNode::Number(y)) => Some((*x, *y)),
                _ => None,
            }
            _ => None,
        }
    }

    fn get_pair_nodes_mut(&mut self) -> Option<(&mut SnailfishNode, &mut SnailfishNode)> {
        match self {
            SnailfishNode::Pair(a) => {
                let (a, b) = a.split_at_mut(1);
                Some((&mut a[0], &mut b[0]))
            }
            _ => None
        }
    }
}

impl SnailfishNode {
    fn absorb_explosion_left(&mut self, value: u32) {
        match self {
            SnailfishNode::Number(n) => *n += value,
            SnailfishNode::Pair(a) => a[0].absorb_explosion_left(value),
        }
    }

    fn absorb_explosion_right(&mut self, value: u32) {
        match self {
            SnailfishNode::Number(n) => *n += value,
            SnailfishNode::Pair(a) => a[1].absorb_explosion_right(value),
        }
    }
}

impl Display for SnailfishNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNode::Number(n) => write!(f, "{}", n),
            SnailfishNode::Pair(a) => write!(f, "[{},{}]", a[0], a[1]),
        }
    }
}

impl SnailfishNode {
    fn try_explode(&mut self, depth: usize) -> Option<Explosion> {
        // individual numbers cant explode
        if self.is_number() {
            return None;
        }

        if depth > 4 {
            if let Some((left, right)) = self.get_pair_numbers() {
                return Some(Explosion::new(left, right));
            }
        }

        let (left, right) = self.get_pair_nodes_mut().unwrap();
        if let Some(mut explosion) = left.try_explode(depth + 1) {
            if explosion.immediate {
                *left = SnailfishNode::Number(0);
            }
            if let Some(right_value) = explosion.right {
                right.absorb_explosion_left(right_value);
                explosion.right = None;
            }
            return Some(explosion.non_immediate());
        }
        if let Some(mut explosion) = right.try_explode(depth + 1) {
            if explosion.immediate {
                *right = SnailfishNode::Number(0);
            }
            if let Some(left_value) = explosion.left {
                left.absorb_explosion_right(left_value);
                explosion.left = None;
            }
            return Some(explosion.non_immediate());
        }
        None
    }

    fn try_split(&mut self) -> bool {
        match self {
            SnailfishNode::Number(n) => {
                if *n < 10 {
                    return false;
                }

                let l = *n / 2;
                let r = *n / 2 + *n % 2;
                *self = SnailfishNode::Pair(Box::new([
                    SnailfishNode::Number(l),
                    SnailfishNode::Number(r),
                ]));
                return true;
            }
            SnailfishNode::Pair(a) =>
                Self::try_split(&mut a[0]) || Self::try_split(&mut a[1])
        }
    }

    pub fn get_magnitude(&self) -> u32 {
        match self {
            SnailfishNode::Number(n) => *n,
            SnailfishNode::Pair(a) => {
                let left = a[0].get_magnitude();
                let right = a[1].get_magnitude();
                3 * left + 2 * right
            }
        }
    }
}

impl FromStr for SnailfishTree {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_node_rec(s: &mut &str) -> Result<SnailfishNode, Error> {
            if &s[0..1] == "[" {
                *s = &s[1..];
                let left = parse_node_rec(s)?;
                *s = &s[1..];
                let right = parse_node_rec(s)?;
                *s = &s[1..];

                Ok(SnailfishNode::Pair(Box::new([left, right])))
            } else {
                let c = s.chars().take_while(|c| char::is_digit(*c, 10)).count();
                let i = u32::from_str(&s[..c])?;
                *s = &s[c..];
                Ok(SnailfishNode::Number(i))
            }
        }
        debug_assert!(s.is_ascii());
        let mut s = s;
        let root = parse_node_rec(&mut s)?;
        Ok(Self { root })
    }
}

impl Display for SnailfishTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_rec(num: &SnailfishNode, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match num {
                SnailfishNode::Number(n) => write!(f, "{n}"),
                SnailfishNode::Pair(a) => {
                    f.write_char('[')?;
                    fmt_rec(&a[0], f)?;
                    f.write_char(',')?;
                    fmt_rec(&a[1], f)?;
                    f.write_char(']')?;
                    Ok(())
                }
            }
        }
        fmt_rec(&self.root, f)
    }
}

impl std::ops::Add for SnailfishTree {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = Self {
            root: SnailfishNode::Pair(Box::new([self.root, rhs.root]))
        };
        while new.try_reduce() {
            // println!("STEP {new}");
        }
        new
    }
}

struct Explosion {
    left: Option<u32>,
    right: Option<u32>,
    immediate: bool,
}

impl Explosion {
    fn new(left: u32, right: u32) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            immediate: true,
        }
    }

    fn non_immediate(mut self) -> Self {
        self.immediate = false;
        self
    }
}

impl SnailfishTree {
    pub fn try_reduce(&mut self) -> bool {
        self.root.try_explode(1).is_some() ||
            self.root.try_split()
    }

    pub fn get_magnitude(&self) -> u32 {
        self.root.get_magnitude()
    }
}

pub fn task1(mut input: Linewise<SnailfishTree>) -> Result<u32, Error> {
    let mut acc = input.next().expect("Input iterator was empty")?;
    while let Some(next) = input.next() {
        acc = acc + next?;
    }

    Ok(acc.get_magnitude())
}

pub fn task2(input: Linewise<SnailfishTree>) -> Result<u32, Error> {
    let inputs: Vec<_> = input.try_collect2()?;
    let count = inputs.len();

    let mut max = 0;

    for a in 0..count {
        for b in 0..count {
            let sum = inputs[a].clone() + inputs[b].clone();
            let mag = sum.get_magnitude();
            max = max.max(mag);
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    fn parse_roundtrip(s: &str) -> String {
        let num = SnailfishTree::from_str(s).unwrap();
        format!("{}", num)
    }

    #[test]
    fn test_parse() {
        const INPUT_A: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        assert_eq!(parse_roundtrip(INPUT_A), INPUT_A);
        const INPUT_B: &str = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        assert_eq!(parse_roundtrip(INPUT_B), INPUT_B);
        const INPUT_C: &str = "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]";
        assert_eq!(parse_roundtrip(INPUT_C), INPUT_C);
    }

    fn explode_step(s: &str) -> String {
        let mut input = SnailfishTree::from_str(s).unwrap();
        let explosion = input.root.try_explode(1);
        assert!(explosion.is_some());
        format!("{input}")
    }

    #[test]
    fn test_explode() {
        assert_eq!(&explode_step("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
        assert_eq!(&explode_step("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
        assert_eq!(&explode_step("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
        assert_eq!(&explode_step("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(&explode_step("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    fn split_step(s: &str) -> String {
        let mut input = SnailfishTree::from_str(s).unwrap();
        let split = input.root.try_split();
        assert!(split);
        format!("{input}")
    }

    #[test]
    fn test_split() {
        assert_eq!(&split_step("[[[[0,7],4],[15,[0,13]]],[1,1]]"), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert_eq!(&split_step("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4140);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3993);
    }
}
