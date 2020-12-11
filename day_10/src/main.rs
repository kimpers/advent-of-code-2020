use shared;

fn calculate_diff_multiplier(adaptors: &Vec<usize>) -> usize {
    let outlet_joltage = 0;
    let device_joltage = adaptors.iter().max().unwrap() + 3;

    let mut sorted_adaptors = adaptors.clone();
    sorted_adaptors.sort();

    let mut all_adaptors = vec![outlet_joltage];
    for adaptor in sorted_adaptors {
        all_adaptors.push(adaptor)
    }
    all_adaptors.push(device_joltage);

    let mut one_diff_count = 0;
    let mut three_diff_count = 0;

    for i in 0..all_adaptors.len() - 1 {
        if all_adaptors[i] + 1 == all_adaptors[i + 1] {
            one_diff_count += 1;
        } else if all_adaptors[i] + 3 == all_adaptors[i + 1] {
            three_diff_count += 1;
        }
    }

    one_diff_count * three_diff_count
}

fn multiplier_for_consecutive_nums(consecutive_num_count: usize) -> usize {
    match consecutive_num_count {
        0 => return 1,
        1 => return 1,
        2 => return 2,
        3 => return 4,
        4 => return 7, // HACK: Input file contains max 4 consecutive numbers
        _ => panic!("Not handled count of {}", consecutive_num_count),
    }
}

fn calculate_adapter_configurations(adaptors: &Vec<usize>) -> usize {
    let outlet_joltage = 0;
    let device_joltage = adaptors.iter().max().unwrap() + 3;

    let mut sorted_adaptors = adaptors.clone();
    sorted_adaptors.sort();

    let mut all_adaptors = vec![outlet_joltage];
    for adaptor in sorted_adaptors {
        all_adaptors.push(adaptor)
    }
    all_adaptors.push(device_joltage);

    let mut product = 1;
    let mut consecutive_numbers = 0;
    for i in 1..all_adaptors.len() {
        let prev = all_adaptors[i - 1] as isize;
        let curr = all_adaptors[i] as isize;

        match curr - prev {
            1 => consecutive_numbers += 1,
            3 => {
                product *= multiplier_for_consecutive_nums(consecutive_numbers);
                consecutive_numbers = 0;
            } // HACK: only consecutive and 3 diff sequences in input file
            diff => panic!("Diff of {} not accounted for", diff),
        }
    }

    return product;
}

fn parse_input(filename: &str) -> Vec<usize> {
    shared::read_file(filename)
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn main() {
    let adaptors = parse_input("input.txt");
    let multiplier = calculate_diff_multiplier(&adaptors);
    println!("The 1 and 3 jolt diff multiplier is {}", multiplier);

    let max_configs = calculate_adapter_configurations(&adaptors);
    println!("Max different configurations {}", max_configs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_the_adaptor_chain() {
        let adaptors = parse_input("test_input.txt");
        assert_eq!(calculate_diff_multiplier(&adaptors), 220);
    }

    #[test]
    fn it_calculates_the_different_configurations() {
        let adaptors = parse_input("test_input.txt");
        assert_eq!(calculate_adapter_configurations(&adaptors), 19208);
    }
}
