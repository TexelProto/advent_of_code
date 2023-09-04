#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct HexInput(Vec<u8>);

impl HexInput {
    fn get_bits(&self, offset: usize, bits: usize) -> u8 {
        debug_assert!(bits <= 8, "bits must be <= 8");
        let start_byte = offset / 8;
        let end_byte = (offset + bits - 1) / 8;

        debug_assert!(
            end_byte < self.0.len(),
            "end exceeds bounds. {} > {}",
            offset + bits,
            self.0.len() * 8
        );

        if start_byte == end_byte {
            // imagine taking bits 0..4
            // turn ABCD_EFGH into 0000_ABCD
            let bit_shift = 8 - ((offset + bits) % 8) as u32;
            let mask = if bits == 8 { 255 } else { (1 << bits) - 1 };
            (self.0[start_byte].overflowing_shr(bit_shift).0) & mask
        } else {
            // imagine taking bits 14..18
            // first_bits will be 2 (14..16)
            // turn ABCD_EFGH into 0000_00GH
            let first_bits = 8 - (offset % 8);
            let first_mask = (1 << first_bits) - 1;
            let first_byte = self.0[start_byte] & first_mask;

            // second_bits will be 2 (16..18)
            // turn ABCD_EFGH into 0000_00AB
            let second_bits = bits - first_bits;
            let second_shift = 8 - second_bits;
            let second_byte = self.0[end_byte] >> second_shift;

            // combine into 0000_GHAB
            (first_byte << second_bits) | second_byte
        }
    }
}

fn convert_hex_char(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => unreachable!()
    }
}

fn sum_version_numbers(hex: &HexInput, head: &mut usize) -> u32 {
    let mut version_sum = 0u32;

    let version = hex.get_bits(*head, 3);
    version_sum += version as u32;
    *head += 3;

    let id = hex.get_bits(*head, 3);
    *head += 3;

    if id == 4 {
        // make sure to exhaust the literal data
        loop {
            let literal = hex.get_bits(*head, 5);
            *head += 5;
            if literal & 0b_10000 == 0 { return version_sum; }
        }
    }

    let length_type = hex.get_bits(*head, 1);
    *head += 1;
    match length_type {
        0 => {
            // we only support 8 bit reads so build the 15 bit number custom
            let hi = hex.get_bits(*head, 7);
            *head += 7;
            let lo = hex.get_bits(*head, 8);
            *head += 8;

            let count = ((hi as u16) << 8) | lo as u16;
            let end = *head + count as usize;

            loop {
                version_sum += sum_version_numbers(hex, head);
                if *head >= end { break; }
            }
        }
        1 => {
            let hi = hex.get_bits(*head, 3);
            *head += 3;
            let lo = hex.get_bits(*head, 8);
            *head += 8;

            let count = ((hi as u16) << 8) | lo as u16;
            for _ in 0..count {
                version_sum += sum_version_numbers(hex, head);
            }
        }
        _ => unreachable!()
    }
    version_sum
}

pub fn task1(input: String) -> Result<u32, Error> {
    let mut bytes = vec![];
    for chunk in input.into_bytes().chunks(2) {
        let a = convert_hex_char(chunk[0]);
        let b = convert_hex_char(chunk[1]);
        bytes.push((a << 4) | b);
    }
    let hex = HexInput(bytes);

    let mut head = 0;
    let sum = sum_version_numbers(&hex, &mut head);
    Ok(sum)
}

fn calculate_package_tree(hex: &HexInput, head: &mut usize) -> u64 {
    let _version = hex.get_bits(*head, 3);
    *head += 3;

    let id = hex.get_bits(*head, 3);
    *head += 3;

    if id == 4 {
        let mut value = 0u64;
        loop {
            let literal = hex.get_bits(*head, 5);
            *head += 5;
            let lit_value = literal & 0b_1111;
            value = (value << 4) | lit_value as u64;
            if literal & 0b_10000 == 0 { return value; }
        }
    }

    let length_type = hex.get_bits(*head, 1);
    *head += 1;
    let mut values = vec![];
    match length_type {
        0 => {
            // we only support 8 bit reads so build the 15 bit number custom
            let hi = hex.get_bits(*head, 7);
            *head += 7;
            let lo = hex.get_bits(*head, 8);
            *head += 8;

            let count = ((hi as u16) << 8) | lo as u16;
            let end = *head + count as usize;

            loop {
                values.push(calculate_package_tree(hex, head));
                if *head >= end { break; }
            }
        }
        1 => {
            let hi = hex.get_bits(*head, 3);
            *head += 3;
            let lo = hex.get_bits(*head, 8);
            *head += 8;

            let count = ((hi as u16) << 8) | lo as u16;
            values.reserve(count as usize);
            for _ in 0..count {
                values.push(calculate_package_tree(hex, head));
            }
        }
        _ => unreachable!()
    };

    match id {
        0 => values.into_iter().sum(),
        1 => values.into_iter().product(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => if values[0] > values[1] { 1 } else { 0 },
        6 => if values[0] < values[1] { 1 } else { 0 },
        7 => if values[0] == values[1] { 1 } else { 0 },
        _ => unreachable!()
    }
}

pub fn task2(input: String) -> Result<u64, Error> {
    let mut bytes = vec![];
    for chunk in input.into_bytes().chunks(2) {
        let a = convert_hex_char(chunk[0]);
        let b = convert_hex_char(chunk[1]);
        bytes.push((a << 4) | b);
    }
    let hex = HexInput(bytes);

    let mut head = 0;
    let sum = calculate_package_tree(&hex, &mut head);
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    #[test]
    fn test_hex_get_bits() {
        let bits = vec![0b_1010_0000, 0b_1111_1111];
        let hex = HexInput(bits);
        assert_eq!(hex.get_bits(0, 4), 0b_1010);
        assert_eq!(hex.get_bits(0, 8), 0b_1010_0000);
        assert_eq!(hex.get_bits(2, 4), 0b_1000);
        assert_eq!(hex.get_bits(4, 4), 0b_0000);
        assert_eq!(hex.get_bits(5, 6), 0b_000_111);
        assert_eq!(hex.get_bits(4, 8), 0b_0000_1111);
        assert_eq!(hex.get_bits(7, 7), 0b_0_111111);
        assert_eq!(hex.get_bits(6, 4), 0b_0011);
        assert_eq!(hex.get_bits(8, 4), 0b_1111);
        assert_eq!(hex.get_bits(8, 8), 0b_1111_1111);
    }

    fn test_task1_core(s: &str) -> u32 {
        let buf = std::io::BufReader::new(s.as_bytes());
        let result = task1(Input::parse(buf).unwrap());
        result.unwrap()
    }

    #[test]
    fn test_task1() {
        assert_eq!(test_task1_core("D2FE28"), 6);
        assert_eq!(test_task1_core("8A004A801A8002F478"), 16);
        assert_eq!(test_task1_core("620080001611562C8802118E34"), 12);
        assert_eq!(test_task1_core("C0015000016115A2E0802F182340"), 23);
        assert_eq!(test_task1_core("A0016C880162017C3686B18A3D4780"), 31);
    }
    
    fn test_task2_core(s: &str) -> u32 {
        let buf = std::io::BufReader::new(s.as_bytes());
        let result = task2(Input::parse(buf).unwrap());
        result.unwrap()
    }
    
    #[test]
    fn test_task2() {
        assert_eq!(test_task2_core("C200B40A82"), 3);
        assert_eq!(test_task2_core("04005AC33890"), 54);
        assert_eq!(test_task2_core("880086C3E88112"), 7);
        assert_eq!(test_task2_core("CE00C43D881120"), 9);
        assert_eq!(test_task2_core("D8005AC2A8F0"), 1);
        assert_eq!(test_task2_core("F600BC2D8F"), 0);
        assert_eq!(test_task2_core("9C005AC2F8F0"), 0);
        assert_eq!(test_task2_core("9C0141080250320F1802104A08"), 1);
    }
}
