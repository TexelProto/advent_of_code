use std::{
    cell::RefCell,
    collections::HashSet,
    convert::Infallible,
    iter::{Repeat, Take},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 1);
        let value = match s {
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Unknown pattern {}", s),
        };
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy)]
struct MultiMove {
    mov: Move,
    repetitions: usize,
}

impl FromStr for MultiMove {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (m, i) = s.split_at(1);
        let mov = Move::from_str(m.trim()).unwrap();
        let i = usize::from_str(i.trim()).unwrap();
        let value = Self {
            mov,
            repetitions: i,
        };
        Ok(value)
    }
}

impl IntoIterator for MultiMove {
    type Item = Move;
    type IntoIter = Take<Repeat<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat(self.mov).take(self.repetitions)
    }
}

fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = Move> {
    input.lines().flat_map(|s| MultiMove::from_str(s).unwrap())
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn move_point(&mut self, mov: Move) {
        match mov {
            Move::Left => self.0 -= 1,
            Move::Right => self.0 += 1,
            Move::Down => self.1 -= 1,
            Move::Up => self.1 += 1,
        }
    }
    fn follow(&mut self, other: &Point) {
        let x_diff = other.0 - self.0;
        let y_diff = other.1 - self.1;

        // points are adjacent -> no movement
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            return;
        }

        // one axis is zero -> movement is axis aligned
        if x_diff == 0 || y_diff == 0 {
            self.0 += x_diff / 2;
            self.1 += y_diff / 2;
        } else {
            self.0 += x_diff / 2 + x_diff % 2;
            self.1 += y_diff / 2 + y_diff % 2;
        }
    }
}

pub fn task1(input: String) {
    let mut visited = HashSet::new();

        let mut head = Point::default();
        let mut tail = Point::default();
        for m in parse(&input) {
            head.move_point(m);
            tail.follow(&head);

            visited.insert(tail.clone());
        }

        println!("visited: {}", visited.len());
    }
    pub fn task2(input: String) {
        let mut visited = HashSet::new();

        let points = vec![RefCell::new(Point::default()); 10];
        for m in parse(&input) {
            {
                points[0].borrow_mut().move_point(m);
            }

            for i in 1..points.len() {
                let previous = points[i-1].borrow();
                let mut point = points[i].borrow_mut();
                point.follow(&previous);
            }

            let tail = points.last().unwrap().borrow();
            visited.insert(tail.clone());
        }

        println!("visited: {}", visited.len());
    }
