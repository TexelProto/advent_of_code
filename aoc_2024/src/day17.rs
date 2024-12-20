use common::input::Linewise;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pattern_parse::parse_fn!(parse_register, "Register {char}: {u64}");

pub fn task1(mut input: Linewise<String>) -> Result<String, Error> {
    let line_a = input.next().unwrap().unwrap();
    let line_b = input.next().unwrap().unwrap();
    let line_c = input.next().unwrap().unwrap();
    let mut registers = [
        parse_register(line_a.as_str()).unwrap().1,
        parse_register(line_b.as_str()).unwrap().1,
        parse_register(line_c.as_str()).unwrap().1,
    ];

    _ = input.next().unwrap();
    let instructions = input.next().unwrap().unwrap()[9..]
        .split(',')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut output = vec![];
    run_program(&instructions, &mut registers, &mut output);

    let mut result = String::new();
    for i in output {
        result.push((b'0' + i) as char);
        result.push(',');
    }
    result.remove(result.len() - 1);
    Ok(result)
}

fn run_program(instructions: &Vec<u8>, mut registers: &mut [u64; 3], mut output: &mut Vec<u8>) {
    let mut instruction_ptr = 0;
    while instruction_ptr < instructions.len() {
        advance(
            &mut output,
            &mut registers,
            &instructions,
            &mut instruction_ptr,
        );
    }
}

fn advance(output: &mut Vec<u8>, registers: &mut [u64; 3], instructions: &Vec<u8>, ip: &mut usize) {
    let instruction = instructions[*ip];
    let operand = instructions[*ip + 1];
    *ip += 2;

    match instruction {
        0 => {
            let denominator = 1 << read_combo_operand(&*registers, operand);
            registers[0] = registers[0] / denominator;
        }
        1 => {
            registers[1] ^= operand as u64;
        }
        2 => {
            let value = read_combo_operand(&*registers, operand) % 8;
            registers[1] = value;
        }
        3 => {
            if registers[0] != 0 {
                *ip = operand as usize;
            }
        }
        4 => {
            registers[1] ^= registers[2];
        }
        5 => {
            let value = read_combo_operand(&*registers, operand) % 8;
            output.push(value as u8);
        }
        6 => {
            let denominator = 1 << read_combo_operand(&*registers, operand);
            registers[1] = registers[0] / denominator;
        }
        7 => {
            let denominator = 1 << read_combo_operand(&*registers, operand);
            registers[2] = registers[0] / denominator;
        }
        _ => unreachable!(),
    }
}

fn read_combo_operand(registers: &[u64; 3], operand: u8) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}
pub fn task2(mut input: Linewise<String>) -> Result<u64, Error> {
    _ = input.next().unwrap().unwrap();
    _ = input.next().unwrap().unwrap();
    _ = input.next().unwrap().unwrap();
    _ = input.next().unwrap();
    let instructions = input.next().unwrap().unwrap()[9..]
        .split(',')
        .map(|s| u8::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let init_a = match_incremental(&instructions, 0).unwrap();

    Ok(init_a)
}

fn match_incremental(instructions: &Vec<u8>, init_a: u64) -> Option<u64> {
    let mut output = vec![];
    let mut registers = [init_a, 0, 0];
    run_program(&instructions, &mut registers, &mut output);

    if output.iter().eq(instructions) {
        return Some(init_a);
    }

    let shifted = init_a << 3;
    for add in 0..8 {
        if shifted == 0 && add == 0 {
            continue;
        }

        output.clear();
        registers = [shifted + add, 0, 0];
        run_program(&instructions, &mut registers, &mut output);

        let cmp_start = instructions.len() - output.len();
        let cmp_instructions = &instructions[cmp_start..];
        let partial_match = cmp_instructions.iter().eq(&output);

        if partial_match {
            if let Some(total) = match_incremental(instructions, shifted + add) {
                return Some(total);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT1: &[u8] = b"\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &[u8] = b"\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT1);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, "7,1,2,3,2,6,7,2,5");
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT2);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 117440);
    }
}
