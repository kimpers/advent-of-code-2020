use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Password {
  start: usize,
  end: usize,
  requirement: String,
  password: String,
}

impl Password {
  pub fn parse_password(line: &str) -> Password {
    let results = line.split(" ").collect::<Vec<&str>>();
    assert!(results.len() == 3, "could not parse password line");
    let requirements = results[0]
      .split("-")
      .map(|r| r.trim())
      .collect::<Vec<&str>>();
    assert!(requirements.len() == 2, "invalid password requirements");
    let (start, end) = (
      requirements[0].parse().unwrap(),
      requirements[1].parse().unwrap(),
    );

    let requirement = results[1].replace(":", "").trim().to_string();

    let password = results[2].trim().to_string();

    Password {
      start,
      end,
      requirement,
      password,
    }
  }

  pub fn is_valid_old_rule(&self) -> bool {
    let occurances = self.password.split("").into_iter().fold(0, |count, chr| {
      if chr == self.requirement {
        return count + 1;
      }

      count
    });

    if occurances >= self.start && occurances <= self.end {
      return true;
    }

    false
  }

  pub fn is_valid_new_rule(&self) -> bool {
    let password_chars = self
      .password
      .chars()
      .map(|c| c.to_string())
      .collect::<Vec<String>>();
    let is_char_idx_1 = password_chars[self.start - 1] == self.requirement;
    let is_char_idx_2 = password_chars[self.end - 1] == self.requirement;

    (is_char_idx_1 && !is_char_idx_2) || (!is_char_idx_1 && is_char_idx_2)
  }
}

fn main() {
  let mut file = File::open("input.txt").unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();
  let lines = contents.lines().collect::<Vec<&str>>();
  let num_valid_old_rule = lines
    .clone()
    .into_iter()
    .filter(|line| Password::parse_password(line).is_valid_old_rule())
    .count();

  let num_valid_new_rule = lines
    .clone()
    .into_iter()
    .filter(|line| Password::parse_password(line).is_valid_new_rule())
    .count();

  println!("# valid passwords {} with the old rule", num_valid_old_rule);
  println!("# valid passwords {} with the new rule", num_valid_new_rule);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_parses_password_list() {
    let line = "1-3 a: abcde";
    let password = Password::parse_password(line);
    assert_eq!(password.start, 1);
    assert_eq!(password.end, 3);
    assert_eq!(password.requirement, "a");
    assert_eq!(password.password, "abcde");
  }

  #[test]
  fn it_checks_password_validity_old_rule() {
    assert_eq!(
      Password::parse_password("1-3 a: abcde").is_valid_old_rule(),
      true
    );
    assert_eq!(
      Password::parse_password("1-3 b: cdefg").is_valid_old_rule(),
      false
    );
    assert_eq!(
      Password::parse_password("2-9 c: ccccccccc").is_valid_old_rule(),
      true
    );
  }

  #[test]
  fn it_checks_password_validity_new_rule() {
    assert_eq!(
      Password::parse_password("1-3 a: abcde").is_valid_new_rule(),
      true
    );
    assert_eq!(
      Password::parse_password("1-3 b: cdefg").is_valid_new_rule(),
      false
    );
    assert_eq!(
      Password::parse_password("2-9 c: ccccccccc").is_valid_new_rule(),
      false
    );
  }
}
