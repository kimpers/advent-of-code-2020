use shared;

struct XMASCypher {
  pub data: Vec<usize>,
  pub preamble_length: usize,
}

impl XMASCypher {
  fn parse_cypher(filename: &str, preamble_length: usize) -> XMASCypher {
    let data = shared::read_file(filename)
      .iter()
      .map(|line| line.trim().parse::<usize>().unwrap())
      .collect::<Vec<usize>>();

    XMASCypher {
      data,
      preamble_length,
    }
  }

  fn has_pair_with_sum(&self, numbers: &[usize], desired_sum: usize) -> bool {
    for i in 0..numbers.len() {
      for j in 0..numbers.len() {
        if numbers[i] + numbers[j] == desired_sum && i != j {
          return true;
        }
      }
    }
    false
  }

  pub fn find_invalid(&self) -> usize {
    let mut current_index = self.preamble_length;

    while current_index < self.data.len() {
      let range_start = current_index - self.preamble_length;
      let sum_range = &self.data[range_start..current_index];

      if !self.has_pair_with_sum(sum_range, self.data[current_index]) {
        return self.data[current_index];
      }

      current_index += 1;
    }

    panic!("Could not find any invalid number");
  }

  pub fn find_weakness(&self) -> usize {
    let invalid_num = self.find_invalid();

    let potential_sum_numbers = &self.data;

    for i in 0..potential_sum_numbers.len() {
      let mut current_sum = potential_sum_numbers[i];
      let mut current_nums = vec![current_sum];
      for j in (i + 1)..potential_sum_numbers.len() {
        if j == i {
          continue;
        }

        if current_sum < invalid_num {
          current_sum += potential_sum_numbers[j];
          current_nums.push(potential_sum_numbers[j]);
        } else {
          break;
        }
      }

      if current_sum == invalid_num {
        return current_nums.iter().min().unwrap() + current_nums.iter().max().unwrap();
      }
    }

    panic!("Could not find any weakness");
  }
}

fn main() {
  let cypher = XMASCypher::parse_cypher("input.txt", 25);
  let invalid_num = cypher.find_invalid();
  println!("Invalid number in cypher is {}", invalid_num);

  let weakness_num = cypher.find_weakness();
  println!("Cypher weakness number is {}", weakness_num);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_parses_input() {
    let cypher = XMASCypher::parse_cypher("test_input.txt", 5);
    assert_eq!(cypher.data.len(), 20);
    assert_eq!(cypher.data[0], 35);
    assert_eq!(cypher.data[19], 576);
    assert_eq!(cypher.preamble_length, 5);
  }

  #[test]
  fn it_finds_the_invalid_number() {
    let cypher = XMASCypher::parse_cypher("test_input.txt", 5);
    assert_eq!(cypher.find_invalid(), 127)
  }

  #[test]
  fn it_finds_the_weakness() {
    let cypher = XMASCypher::parse_cypher("test_input.txt", 5);
    assert_eq!(cypher.find_weakness(), 62)
  }
}
