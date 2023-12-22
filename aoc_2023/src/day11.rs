use common::geometry_2d::Point;
use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: Linewise<String>) -> Result<u64, Error> {
    let mut points = vec![];
    let mut rows = vec![];
    let mut columns = vec![];

    for (y, line) in input.enumerate() {
        let line = line.unwrap();
        for (x, char) in line.char_indices() {
            if char != '#' { continue; }

            if rows.last() != Some(&y) {
                rows.push(y);
            }

            if let Err(i) = columns.binary_search(&x) {
                columns.insert(i, x);
            }

            points.push(Point { x: x as u32, y: y as u32 });
        }
    }

    let mut total = 0;

    for from_idx in 0..points.len() {
        let from = points[from_idx];
        for to_idx in from_idx + 1..points.len() {
            let to = points[to_idx];

            let dist = get_point_distance(&mut rows, &mut columns, from, to, 2);
            total += dist;
        }
    }
    Ok(total)
}

fn get_point_distance(rows: &mut Vec<usize>, columns: &mut Vec<usize>, from: Point, to: Point, empty_factor: u64) -> u64 {
    let from_x_idx = columns
        .binary_search(&(from.x as usize))
        .expect("Failed to find point column");
    let from_y_idx = rows
        .binary_search(&(from.y as usize))
        .expect("Failed to find point row");

    let to_x_idx = columns
        .binary_search(&(to.x as usize))
        .expect("Failed to find point column");
    let to_y_idx = rows
        .binary_search(&(to.y as usize))
        .expect("Failed to find point row");

    let coord_dist_x = to.x.abs_diff(from.x) as u64;

    let true_dist_x = if coord_dist_x == 0 {
        0
    } else {
        let index_dist_x = to_x_idx.abs_diff(from_x_idx) as u64;
        let empty = coord_dist_x - index_dist_x;
        index_dist_x + empty_factor * empty
    };

    let coord_dist_y = to.y.abs_diff(from.y) as u64;

    let true_dist_y = if coord_dist_y == 0 {
        0
    } else {
        let index_dist_y = to_y_idx.abs_diff(from_y_idx) as u64;
        let empty = coord_dist_y - index_dist_y;
        index_dist_y + empty_factor * empty
    };

    true_dist_x + true_dist_y
}

pub fn task2(input: Linewise<String>) -> Result<u64, Error> {
    let mut points = vec![];
    let mut rows = vec![];
    let mut columns = vec![];

    for (y, line) in input.enumerate() {
        let line = line.unwrap();
        for (x, char) in line.char_indices() {
            if char != '#' { continue; }

            if rows.last() != Some(&y) {
                rows.push(y);
            }

            if let Err(i) = columns.binary_search(&x) {
                columns.insert(i, x);
            }

            points.push(Point { x: x as u32, y: y as u32 });
        }
    }

    let mut total = 0;

    for from_idx in 0..points.len() {
        let from = points[from_idx];
        for to_idx in from_idx + 1..points.len() {
            let to = points[to_idx];

            let dist = get_point_distance(&mut rows, &mut columns, from, to, 1000000);
            total += dist;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 374);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 82000210);
    }
}
