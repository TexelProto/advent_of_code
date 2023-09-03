use common::{input::Linewise, some_or_continue};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

fn read_2d<T>(vec: &Vec<Vec<T>>, x: usize, y: usize) -> Option<&T> {
    vec.get(y)?.get(x)
}
fn check_low_point<T: Ord>(vec: &Vec<Vec<T>>, x: usize, y: usize) -> bool {
    let value = &vec[y][x];
    if x > 0 {
        let left = &vec[y][x-1];
        if left <= value {
            return false;
        }
    }
    if y > 0 {
        let up = &vec[y-1][x];
        if up <= value {
            return false;
        }
    }
    if let Some(right) = read_2d(vec, x + 1, y) {
        if right <= value {
            return false;
        }
    }
    if let Some(down) = read_2d(vec, x, y + 1) {
        if down <= value {
            return false;
        }
    }

    true
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut rows = vec![];
    for i in input {
        let i = i.unwrap();
        let row: Vec<_> = i.bytes().map(|c| c - '0' as u8).collect();
        rows.push(row);
    }

    let height = rows.len();
    let width = rows[0].len();

    let mut total_risk = 0;

    for y in 0..height {
        for x in 0..width {
            if check_low_point(&rows, x, y) {
                let value = rows[y][x];
                total_risk += value as u32 + 1;
            }
        }
    }

    Ok(total_risk)
}

pub fn task2(input: Linewise<String>) -> Result<usize, Error> {
    let mut rows = vec![];
    for i in input {
        let i = i.unwrap();
        let row: Vec<_> = i.bytes().map(|c| c - '0' as u8).collect();
        rows.push(row);
    }

    let height = rows.len();
    let width = rows[0].len();

    let mut basins = vec![];

    for y in 0..height {
        for x in 0..width {
            if check_low_point(&rows, x, y) {
                let basin = start_basin(&rows, x, y);
                let size = basin.len();
                let i = match basins.binary_search(&size) {
                    Ok(i) => i,
                    Err(i) => i,
                };

                basins.insert(i, size);
            }
        }
    }
    
    let count = basins.len();
    let biggest = &basins[count-3..];
    let result = biggest.iter()
            .product();

    Ok(result)
}

type PointSet = ahash::HashSet<(usize, usize)>;

fn start_basin(rows: &[Vec<u8>], x: usize, y: usize) -> PointSet {
    let mut points = PointSet::default();
    points.insert((x,y));
    try_grow_from(&mut points, rows, x, y);
    points
}

fn try_grow_from(points: &mut PointSet, rows: &[Vec<u8>], x: usize, y: usize) {
    const OFFSETS: [(isize, isize); 4] = [
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1)
    ];

    let val = rows[y][x];
    for (ox, oy) in OFFSETS {
        let nx = some_or_continue!(x.checked_add_signed(ox));
        let ny = some_or_continue!(y.checked_add_signed(oy));
        let row = some_or_continue!(rows.get(ny));
        let cell = *some_or_continue!(row.get(nx));

        if cell != 9 && cell >= val + 1 && points.insert((nx,ny)) {
            // eprintln!("{:?} => {:?}", (x,y), (nx, ny));
            try_grow_from(points, rows, nx, ny)
        }
    }    
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "2199943210
3987894921
9856789892
8767896789
9899965678".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 15);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1134);
    }
}
