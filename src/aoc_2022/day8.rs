use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    str::FromStr,
};

struct Array2D<T, const X: usize, const Y: usize>([[T; Y]; X]);

impl<T, const X: usize, const Y: usize> Index<(usize, usize)> for Array2D<T, X, Y> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

#[derive(Debug)]
struct Vec2D<T>(Vec<Vec<T>>);

impl<T: Default + Clone> Vec2D<T> {
    fn with_size(x: usize, y: usize) -> Self {
        Self(vec![vec![T::default(); y]; x])
    }
}

impl<T> Vec2D<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(x).and_then(move |s| s.get(y))
    }
    fn width(&self) -> usize {
        self.0.len()
    }
    fn height(&self) -> usize {
        self.0.get(0).map_or_else(usize::default, |s| s.len())
    }
}
impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}
impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Vec2DWalker<'a, T> {
    map: &'a Vec2D<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Debug> Debug for Vec2DWalker<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Walker")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("value", self.value())
            .finish()
    }
}

impl<'a, T> Vec2DWalker<'a, T> {
    fn new(map: &'a Vec2D<T>, x: usize, y: usize) -> Option<Self> {
        if map.get(x, y).is_none() {
            None
        } else {
            Some(Self { map, x, y })
        }
    }

    fn value(&self) -> &'a T {
        self.map.get(self.x, self.y).unwrap()
    }

    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Self::new(self.map, self.x - 1, self.y)
        }
    }

    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Self::new(self.map, self.x, self.y - 1)
        }
    }

    fn right(&self) -> Option<Self> {
        Self::new(self.map, self.x + 1, self.y)
    }

    fn down(&self) -> Option<Self> {
        Self::new(self.map, self.x, self.y + 1)
    }

    fn in_direction(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => self.up(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
        }
    }
}

fn parse(input: &str) -> Vec2D<usize> {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut vec = Vec2D::<usize>::with_size(w, h);

    for (y, line) in input.lines().enumerate() {
        for x in 0..line.trim().len() {
            let s = &line[x..=x];
            let value = usize::from_str(s).unwrap();
            vec[(x, y)] = value;
        }
    }
    vec
}

fn view_in_direction(point: &Vec2DWalker<usize>, dir: Direction) -> (bool, usize) {
    let height = point.value();
    let mut opt_walker = point.in_direction(dir);
    let mut distance = 0_usize;
    loop {
        if opt_walker.is_none() {
            return (true, distance);
        }

        let walker = opt_walker.unwrap();
        distance += 1;

        if walker.value() >= height {
            return (false, distance);
        }

        opt_walker = walker.in_direction(dir);
    }
}
fn is_point_visible(point: Vec2DWalker<usize>) -> bool {
    [
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ]
    .into_iter()
    .any(move |d| view_in_direction(&point, d).0)
}
fn score_point(point: Vec2DWalker<usize>) -> usize {
    [
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ]
    .into_iter()
    .map(move |d| {
        let view = view_in_direction(&point, d).1;
        if point.x == 49 && point.y == 86 {
            println!("View {:?} is {}", d, view);
        }
        view
    })
    .product::<usize>()
}

pub fn task1(input: String) {
    let map = parse(&input);

        let mut visible = 2 * map.width() + 2 * map.height() - 4;
        for x in 1..map.width() - 1 {
            for y in 1..map.height() - 1 {
                let point = Vec2DWalker::new(&map, x, y).unwrap();
                if is_point_visible(point) {
                    visible += 1;
                }
            }
        }

        println!("visible: {}", visible);
    }
    pub fn task2(input: String) {
        let map = parse(&input);
        let mut max = 0_usize;

        for x in 0..map.width() {
            for y in 0..map.height() {
                let point = Vec2DWalker::new(&map, x, y).unwrap();
                let score = score_point(point);
                if score > max {
                    max = score;
                }
            }
        }

        println!("max score: {}", max);
    }
