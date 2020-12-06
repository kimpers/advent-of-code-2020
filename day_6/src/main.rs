use std::collections::HashMap;
use std::collections::HashSet;

use shared;

fn count_any_yes_anwer(group_answers: &Vec<String>) -> usize {
  let mut unique_answers = HashSet::new();

  for person_answers in group_answers {
    for answer in person_answers.chars() {
      unique_answers.insert(answer);
    }
  }

  unique_answers.len()
}

fn count_all_yes_anwer(group_answers: &Vec<String>) -> usize {
  let mut answer_count = HashMap::new();

  for person_answers in group_answers {
    for answer in person_answers.chars() {
      match answer_count.get_mut(&answer) {
        Some(count) => {
          *count += 1;
        }
        _ => {
          answer_count.insert(answer, 1);
        }
      }
    }
  }

  let num_participants = group_answers.len();
  answer_count
    .iter()
    .filter(|(_key, value)| **value == num_participants)
    .count()
}

fn parse_groups_from_file(filename: &str) -> Vec<Vec<String>> {
  let lines = shared::read_file(filename);
  let mut groups: Vec<Vec<String>> = vec![];
  let mut current_group = vec![];

  for line in lines {
    if line == "" {
      if current_group.len() > 0 {
        groups.push(current_group);
      }

      current_group = vec![];
    } else {
      current_group.push(line)
    }
  }

  // Last group
  groups.push(current_group);

  groups
}

fn main() {
  let groups = parse_groups_from_file("input.txt");

  let answer_counts_any_yes = groups
    .iter()
    .map(|group| count_any_yes_anwer(group))
    .collect::<Vec<usize>>();
  let answer_sum_any_yes: usize = answer_counts_any_yes.iter().sum();
  println!("Sum of any yes answers {}", answer_sum_any_yes);

  let answer_counts_all_yes = groups
    .iter()
    .map(|group| count_all_yes_anwer(group))
    .collect::<Vec<usize>>();
  let answer_sum_all_yes: usize = answer_counts_all_yes.iter().sum();
  println!("Sum of all yes answers {}", answer_sum_all_yes)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_parses_groups_from_input() {
    let groups = parse_groups_from_file("test_input.txt");
    assert_eq!(groups.len(), 5, "num groups");
    assert_eq!(groups[0], vec!["abc"], "single person answers");
    assert_eq!(groups[2], vec!["ab", "ac"], "multiple people answers");
  }

  #[test]
  fn it_counts_group_answers() {
    let groups = parse_groups_from_file("test_input.txt");
    let answers = [3, 3, 3, 1, 1];

    for i in 0..groups.len() {
      assert_eq!(
        count_any_yes_anwer(&groups[i]),
        answers[i],
        "correct answer count"
      )
    }
  }

  #[test]
  fn it_counts_all_yes_answers() {
    let groups = parse_groups_from_file("test_input.txt");

    let answer_counts = groups
      .iter()
      .map(|group| count_all_yes_anwer(group))
      .collect::<Vec<usize>>();

    let answer_sum: usize = answer_counts.iter().sum();
    assert_eq!(answer_sum, 6, "num all yes answers");
  }
}
