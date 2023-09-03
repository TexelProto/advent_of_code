use common::{input::digits::DigitMap, some_or_continue};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: DigitMap<u8>) -> Result<u32, Error> {
    let mut map = input.into_inner();

    let mut flashes = 0;
    for _step in 0..100 {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                increase_energy(&mut map, x, y, &mut flashes);
            }
        }
        for row in map.iter_mut() {
            for cell in row.iter_mut() {
                if *cell > 9 { *cell = 0; }
            }
        }
    }

    Ok(flashes)
}

fn increase_energy(map: &mut Vec<Vec<u8>>, x: usize, y: usize, flashes: &mut u32) {
    let cell = &mut map[y][x];
    *cell += 1;

    if *cell != 10 { return; }

    *flashes += 1;

    for y_offset in -1..=1 {
        let new_y = some_or_continue!(y.checked_add_signed(y_offset));
        let width = some_or_continue!(map.get(new_y)).len();
        for x_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 { continue; }

            let new_x = some_or_continue!(x.checked_add_signed(x_offset));
            if new_x < width {
                increase_energy(map, new_x, new_y, flashes);
            }
        }
    }
}

pub fn task2(input: DigitMap<u8>) -> Result<u32, Error> {
    let mut map = input.into_inner();
    let size = (map.len() * map[0].len()) as u32;

    for i in 0.. {
        let mut flashes = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                increase_energy(&mut map, x, y, &mut flashes);
            }
        }

        if flashes == size { 
            return Ok(i + 1);
        }

        for row in map.iter_mut() {
            for cell in row.iter_mut() {
                if *cell > 9 { *cell = 0; }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1656);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 195);
    }
}
