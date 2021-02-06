use shared;
use std::collections::HashMap;

fn main() {
    let lines = shared::read_file("input.txt");
    let decoded_sum_v1 = decode_program(&lines, Version::V1);
    println!("Decoded value sum is {} for version v1", decoded_sum_v1);

    let decoded_sum_v2 = decode_program(&lines, Version::V2);
    println!("Decoded value sum is {} for version v2", decoded_sum_v2);
}

fn parse_to_36bit_binary(value: usize) -> String {
    let binary_representation = format!("{:036b}", value);

    binary_representation
}

fn from_binary(binary_representation: &str) -> usize {
    usize::from_str_radix(binary_representation, 2).unwrap()
}

fn apply_mask_to_value(mask: &str, value: usize) -> String {
    let mask_chars = mask.chars().rev().collect::<Vec<char>>();
    let binary_value = parse_to_36bit_binary(value);
    let binary_value_chars = binary_value.chars().rev().collect::<Vec<char>>();

    let mut masked_binary_value = String::from("");
    for i in 0..mask.len() {
        let curr: &str = &mask_chars[i].to_string().to_owned();
        let new_bit: String = match curr {
            "X" => binary_value_chars[i].to_string(),
            "0" => "0".to_string(),
            "1" => "1".to_string(),
            _ => panic!("Unhandled char"),
        };

        masked_binary_value.push_str(&new_bit)
    }

    return masked_binary_value.chars().rev().collect::<String>();
}

enum Version {
    V1,
    V2,
}

fn decode_program(lines: &Vec<String>, version: Version) -> usize {
    let mut mem_map = HashMap::new();
    let mut mask = "";
    for line in lines {
        let line_items = line.split("=").collect::<Vec<&str>>();
        let instruction = line_items[0].trim();
        let value = line_items[1].trim();

        match instruction {
            "mask" => {
                mask = value;
            }
            mem => {
                let mem_idx = mem.split("[").collect::<Vec<&str>>()[1]
                    .replace("]", "")
                    .parse::<usize>()
                    .unwrap();

                let value_numeric = value.parse::<usize>().unwrap();
                match version {
                    Version::V1 => {
                        let result = apply_mask_to_value(mask, value_numeric);

                        mem_map.insert(mem_idx, result);
                    }
                    Version::V2 => {
                        let result = parse_to_36bit_binary(value_numeric);
                        let addresses = apply_mask_to_memory_address(mask, mem_idx);

                        for address in addresses {
                            mem_map.insert(from_binary(&address), result.clone());
                        }
                    }
                }
            }
        }
    }

    let sum = mem_map
        .iter()
        .fold(0, |memo, (_key, result)| memo + from_binary(result));

    sum
}

fn apply_mask_to_memory_address(mask: &str, memory_address: usize) -> Vec<String> {
    let mask_chars = mask.chars().rev().collect::<Vec<char>>();
    let binary_memory_address = parse_to_36bit_binary(memory_address);
    let binary_memory_address_chars = binary_memory_address.chars().rev().collect::<Vec<char>>();

    let mut masked_binary_addresses = vec![String::from("")];
    for i in 0..mask.len() {
        masked_binary_addresses =
            masked_binary_addresses
                .iter()
                .fold(vec![], |mut memo, curr_adr| {
                    let curr: &str = &mask_chars[i].to_string().to_owned();

                    match curr {
                        "X" => {
                            memo.push(format!("{}{}", curr_adr, 1));
                            memo.push(format!("{}{}", curr_adr, 0));
                        }
                        "0" => {
                            memo.push(format!("{}{}", curr_adr, binary_memory_address_chars[i]));
                        }
                        "1" => {
                            memo.push(format!("{}{}", curr_adr, 1));
                        }
                        _ => panic!("Unhandled char"),
                    };

                    memo
                });
    }

    return masked_binary_addresses
        .iter()
        .map(|adr| adr.chars().rev().collect())
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_binary_conversions() {
        assert_eq!(
            &parse_to_36bit_binary(11),
            "000000000000000000000000000000001011"
        );
        assert_eq!(from_binary("000000000000000000000000000000001011"), 11);

        assert_eq!(
            &parse_to_36bit_binary(101),
            "000000000000000000000000000001100101"
        );

        assert_eq!(from_binary("000000000000000000000000000001100101"), 101);
    }

    #[test]
    fn it_applies_mask() {
        assert_eq!(
            apply_mask_to_value("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11),
            "000000000000000000000000000001001001"
        );

        assert_eq!(
            apply_mask_to_value("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101),
            "000000000000000000000000000001100101"
        );

        assert_eq!(
            apply_mask_to_value("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0),
            "000000000000000000000000000001000000"
        );
    }

    #[test]
    fn it_applies_mask_to_memory_address() {
        let addresses = apply_mask_to_memory_address("000000000000000000000000000000X1001X", 42);

        let expected = [
            "000000000000000000000000000000011010",
            "000000000000000000000000000000011011",
            "000000000000000000000000000000111010",
            "000000000000000000000000000000111011",
        ];

        for exp in expected.iter() {
            assert!(addresses.contains(&exp.to_string()))
        }
    }

    #[test]
    fn it_decodes_the_program_instructions() {
        let lines = shared::read_file("test_input.txt");
        assert_eq!(decode_program(&lines, Version::V1), 165);
    }

    #[test]
    fn it_decodes_the_program_memory() {
        let lines = shared::read_file("test_input.txt");
        assert_eq!(decode_program(&lines, Version::V2), 208);
    }
}
