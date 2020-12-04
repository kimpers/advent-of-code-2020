use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn parse_input_file(filename: &str) -> Vec<String> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();

  let entries = contents
    .split("\n\n")
    .map(|entry| entry.to_string())
    .collect::<Vec<String>>();
  entries
}

fn has_all_required_fields(entry: &str) -> bool {
  let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  required_fields.iter().all(|field| entry.contains(field))
}

fn has_valid_field_data(entry: &str) -> bool {
  let cleaned_entry = entry.replace("\n", " ");

  let fields: HashMap<&str, &str> =
    cleaned_entry
      .split(" ")
      .fold(HashMap::new(), |mut map, raw_entry| {
        let kv_pairs = raw_entry.split(" ");

        for kv_pair in kv_pairs {
          let kv = kv_pair.split(":").collect::<Vec<&str>>();
          if kv.len() == 2 {
            map.insert(kv[0], kv[1]);
          }
        }

        map
      });

  match fields.get("byr") {
    Some(byr) => match (*byr).parse::<usize>() {
      Ok(byr_value) => {
        if byr_value < 1920 || byr_value > 2002 {
          return false;
        }
      }
      _ => return false,
    },
    _ => return false,
  }

  match fields.get("iyr") {
    Some(iyr) => match (*iyr).parse::<usize>() {
      Ok(iyr_value) => {
        if iyr_value < 2010 || iyr_value > 2020 {
          return false;
        }
      }
      _ => return false,
    },
    _ => return false,
  }

  match fields.get("eyr") {
    Some(eyr) => match (*eyr).parse::<usize>() {
      Ok(eyr_value) => {
        if eyr_value < 2010 || eyr_value > 2030 {
          return false;
        }
      }
      _ => return false,
    },
    _ => return false,
  }

  match fields.get("hgt") {
    Some(hgt) => {
      let hgt_val = *hgt;
      if hgt_val.contains("cm") {
        let hgt_num = hgt_val.replace("cm", "").parse::<usize>().unwrap();
        if hgt_num < 150 || hgt_num > 193 {
          return false;
        }
      } else if hgt_val.contains("in") {
        let hgt_num = hgt_val.replace("in", "").parse::<usize>().unwrap();
        if hgt_num < 59 || hgt_num > 76 {
          return false;
        }
      } else {
        return false;
      }
    }
    _ => return false,
  }

  match fields.get("hcl") {
    Some(hcl) => {
      let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
      if !hcl_re.is_match(*hcl) {
        return false;
      }
    }
    _ => return false,
  }

  match fields.get("ecl") {
    Some(ecl) => {
      let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
      if !valid_eye_colors.contains(&ecl) {
        return false;
      }
    }
    _ => return false,
  }

  match fields.get("pid") {
    Some(pid) => {
      let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
      if !pid_re.is_match(pid) {
        return false;
      }
    }
    _ => return false,
  }

  return true;
}

fn count_valid_entries(entries: &Vec<String>, validate_fn: &dyn Fn(&str) -> bool) -> usize {
  entries.iter().filter(|entry| validate_fn(entry)).count()
}

fn main() {
  let entries = parse_input_file("input.txt");
  let num_valid_field_entries = count_valid_entries(&entries, &has_all_required_fields);
  println!(
    "Number of entries with all required keys: {}",
    num_valid_field_entries
  );
  let num_valid_data_entries = count_valid_entries(&entries, &has_valid_field_data);
  println!(
    "Number of entries with valid data: {}",
    num_valid_data_entries
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_parses_correct_number_of_entries() {
    assert_eq!(
      parse_input_file("test_input.txt").len(),
      4,
      "correct number of entries parsed"
    );
  }

  #[test]
  fn it_checks_entry_validity() {
    let entries = parse_input_file("test_input.txt");

    assert_eq!(
      has_all_required_fields(&entries[0]),
      true,
      "valid, all fileds"
    );
    assert_eq!(
      has_all_required_fields(&entries[1]),
      false,
      "invalid, missing hgt"
    );
    assert_eq!(
      has_all_required_fields(&entries[2]),
      true,
      "valid, missing cid"
    );
    assert_eq!(
      has_all_required_fields(&entries[3]),
      false,
      "invalid, missing cid and byr"
    );
  }

  #[test]
  fn it_counts_valid_entries() {
    let entries = parse_input_file("test_input.txt");
    assert_eq!(
      count_valid_entries(&entries, &has_all_required_fields),
      2,
      "valid entries in test data"
    )
  }

  #[test]
  fn it_checks_validity_of_entries() {
    let entries = parse_input_file("test_input.txt");
    has_valid_field_data(&entries[0]);
  }

  #[test]
  fn it_validates_invalid_entry_data() {
    let invalid_entries = parse_input_file("invalid_format_input.txt");
    assert_eq!(invalid_entries.len(), 4);

    for entry in invalid_entries {
      assert_eq!(has_valid_field_data(&entry), false, "invalid entry data");
    }
  }

  #[test]
  fn it_validates_valid_entry_data() {
    let valid_entries = parse_input_file("valid_format_input.txt");
    assert_eq!(valid_entries.len(), 4);

    for entry in valid_entries {
      assert_eq!(has_valid_field_data(&entry), true, "valid entry data");
    }
  }
}
