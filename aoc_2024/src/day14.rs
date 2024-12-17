use ahash::{HashSet, HashSetExt};
use common::input::Linewise;
use common::iter_ext::TryIterator;
use nalgebra::{point, vector, Point2, Vector2};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] pattern_parse::ParseError),
}

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    position: Point2<usize>,
    velocity: Vector2<isize>,
}

impl FromStr for Robot {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pattern_parse::parse_fn!(parse, "p={usize},{usize} v={isize},{isize}");

        let (px, py, vx, vy) = parse(s)?;
        Ok(Self {
            position: Point2::new(px, py),
            velocity: Vector2::new(vx, vy),
        })
    }
}

pub fn task1(input: Linewise<Robot>) -> Result<u32, Error> {
    task1_core::<101, 103>(input)
}
fn task1_core<const WIDTH: usize, const HEIGHT: usize>(
    input: Linewise<Robot>,
) -> Result<u32, Error> {
    let mut robots = input.try_collect2::<Vec<_>>()?;

    for _step in 0..100 {
        for bot in &mut robots {
            let mut x = (bot.position.x + WIDTH).wrapping_add_signed(bot.velocity.x);
            let mut y = (bot.position.y + HEIGHT).wrapping_add_signed(bot.velocity.y);
            x %= WIDTH;
            y %= HEIGHT;
            bot.position = point![x, y];
        }
        print_map::<WIDTH, HEIGHT>(&robots);
    }

    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let mut count_tr = 0;
    let mut count_tl = 0;
    let mut count_br = 0;
    let mut count_bl = 0;

    for bot in robots {
        if bot.position.x == mid_x || bot.position.y == mid_y {
            continue;
        }
        if bot.position.y < mid_y {
            if bot.position.x < mid_x {
                count_tl += 1
            } else {
                count_tr += 1;
            }
        } else {
            if bot.position.x < mid_x {
                count_bl += 1
            } else {
                count_br += 1;
            }
        }
    }

    Ok(count_tr * count_tl * count_br * count_bl)
}

pub fn task2(input: Linewise<Robot>) -> Result<u32, Error> {
    task2_core::<101, 103>(input)
}

fn task2_core<const WIDTH: usize, const HEIGHT: usize>(
    input: Linewise<Robot>,
) -> Result<u32, Error> {
    let mut robots = input.try_collect2::<Vec<_>>()?;
    let mut step = 0;
    let mut position_cache = HashSet::new();

    loop {
        step += 1;
        for bot in &mut robots {
            let mut x = (bot.position.x + WIDTH).wrapping_add_signed(bot.velocity.x);
            let mut y = (bot.position.y + HEIGHT).wrapping_add_signed(bot.velocity.y);
            x %= WIDTH;
            y %= HEIGHT;
            bot.position = point![x, y];
        }

        position_cache.clear();
        position_cache.extend(robots.iter().map(|r| r.position));
        if has_contiguous(&position_cache, vector![1, 0], 6)
            && has_contiguous(&position_cache, vector![0, 1], 6)
        {
            if cfg!(debug_assertions) {
                print_map::<WIDTH, HEIGHT>(&robots);
            }
            break;
        }
    }

    Ok(step)
}

fn has_contiguous(robots: &HashSet<Point2<usize>>, offset: Vector2<usize>, total: usize) -> bool {
    'bot: for start in robots {
        for i in 1..total {
            let point = start + offset * i;
            if robots.contains(&point) == false {
                continue 'bot;
            }
        }

        return true;
    }
    false
}

fn print_map<const WIDTH: usize, const HEIGHT: usize>(bots: &Vec<Robot>) {
    println!("======================================================================");

    let mut positions = HashSet::with_capacity(bots.len());
    for bot in bots {
        positions.insert(bot.position);
    }

    let mut out = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&point![x, y]) {
                out.push('#')
            } else {
                out.push(' ')
            }
        }
        out.push('\n');
    }

    println!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1_core::<11, 7>(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 12);
    }
}
