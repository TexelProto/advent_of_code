#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(#[from] pattern_parse::ParseError),
}

pattern_parse::parse_fn!(parse, "target area: x={i32}..{i32}, y={i32}..{i32}");

pub fn task1(input: String) -> Result<i32, Error> {
    let (min_x, max_x, min_y, max_y) = parse(&input)?;
    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;

    let mut total_max_y = 0;

    // brute force solution: just try a bunch of initial conditions and see what works
    for init_x in 1..100 {
        for init_y in 1..100 {
            let max = plot_arc(&x_range, &y_range, init_x, init_y);
            if let Some(max) = max {
                total_max_y = total_max_y.max(max);
            }
        }
    }

    Ok(total_max_y)
}

type AreaRange = std::ops::RangeInclusive<i32>;

fn plot_arc(
    x_range: &AreaRange,
    y_range: &AreaRange,
    mut step_x: i32,
    mut step_y: i32,
) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    loop {
        x += step_x;
        step_x = (step_x - 1).max(0);
        y += step_y;
        step_y -= 1;
        max_y = max_y.max(y);

        // check if the probe went beyond the target area
        if x > *x_range.end() || y < *y_range.start() {
            return None;
        } else if x_range.contains(&x) && y_range.contains(&y) {
            // we hit the target area
            return Some(max_y);
        }

        // we're still before the target area, so keep going
    }
}

pub fn task2(input: String) -> Result<u32, Error> {
    let (min_x, max_x, min_y, max_y) = parse(&input)?;
    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;

    let mut hits = 0;

    // brute force solution: just try a bunch of initial conditions and see what works
    // 4M checks might seem like a lot but computes (in release mode on a i5-6600) in 26ms
    // realistically the search space could be significantly smaller but lets be safe 
    for init_x in 0..2000 {
        for init_y in -1000..1000 {
            let max = plot_arc(&x_range, &y_range, init_x, init_y);
            if max.is_some() {
                hits += 1
            }
        }
    }

    Ok(hits)
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = "target area: x=20..30, y=-10..-5".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 45);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 112);
    }
}
