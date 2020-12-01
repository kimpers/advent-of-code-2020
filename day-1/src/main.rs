use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn find_matching_number_pair(arr: &Vec<i32>, desired_sum: i32) -> Option<i32> {
    let mut sorted = arr.clone();
    sorted.sort();

    let mut reverse_sorted = sorted.clone();
    reverse_sorted.reverse();

    for expense_low in sorted.iter() {
        for expense_high in reverse_sorted.iter() {
            if expense_high + expense_low == desired_sum {
                return Some(expense_high * expense_low);
            }
        }
    }
    None
}

fn find_matching_3_number_sequence(arr: &Vec<i32>, desired_sum: i32) -> Option<i32> {
    // NOTE: assumption numbers are unique. Holds true for test set but in reality duplicates would
    // be expected
    let numbers_set: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());

    for (i, expense) in arr.iter().enumerate() {
        for j in i + 1..arr.len() {
            let expense_2 = arr[j];

            let remaining = desired_sum - expense - expense_2;
            if numbers_set.contains(&remaining) {
                return Some(expense * expense_2 * remaining);
            }
        }
    }

    None
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let expenses: Vec<i32> = contents
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect();

    let desired_sum = 2020;
    println!("Results");
    println!(
        "2 numbers {}",
        find_matching_number_pair(&expenses, desired_sum).unwrap()
    );
    println!(
        "3 numbers {}",
        find_matching_3_number_sequence(&expenses, desired_sum).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sums_number_pair() {
        let test_numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(
            find_matching_number_pair(&test_numbers, 2020).unwrap(),
            514579
        );
    }

    #[test]
    fn it_sums_3_number_sequence() {
        let test_numbers = vec![979, 366, 675];
        assert_eq!(
            find_matching_3_number_sequence(&test_numbers, 2020).unwrap(),
            241861950
        )
    }
}
