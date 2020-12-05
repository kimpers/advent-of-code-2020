use shared;
fn find_seat_details(row_encoding: &str) -> (usize, usize) {
  let chars = row_encoding
    .chars()
    .map(|c| c.to_string())
    .collect::<Vec<String>>();
  let mut start_row = 0;
  let mut end_row = 127;
  let mut start_col = 0;
  let mut end_col = 7;

  for char in chars {
    let chr: &str = &char;
    let half_row = (end_row - start_row) / 2;
    let half_col = (end_col - start_col) / 2;
    match chr {
      "F" => end_row = end_row - half_row - 1,
      "L" => end_col = end_col - half_col - 1,
      "B" => start_row = start_row + half_row + 1,
      "R" => start_col = start_col + half_col + 1,
      _ => {
        panic!(format!(
          "Unknown encoded value {}, expected 'F' or 'B'",
          char
        ));
      }
    }
  }

  assert_eq!(start_row, end_row, "start_row != end_row");
  assert_eq!(start_col, end_col, "start_col != end_col");

  (start_row, start_col)
}

fn calculate_seat_id((row, col): (usize, usize)) -> usize {
  row * 8 + col
}

fn find_available_seat_id(sorted_seat_ids: &Vec<usize>) -> Option<usize> {
  let mut i = 0;
  while i < sorted_seat_ids.len() - 1 {
    if sorted_seat_ids[i] + 1 != sorted_seat_ids[i + 1] {
      let available_seat_id = sorted_seat_ids[i] + 1;
      return Some(available_seat_id);
    }
    i += 1;
  }

  None
}

fn main() {
  let boarding_passes = shared::read_file("input.txt");
  let mut seat_ids = boarding_passes
    .iter()
    .map(|pass| {
      let seat_details = find_seat_details(pass);

      calculate_seat_id(seat_details)
    })
    .collect::<Vec<usize>>();

  seat_ids.sort();

  let highest_seat_id = seat_ids[seat_ids.len() - 1];

  println!("Highest seat id: {}", highest_seat_id);

  if let Some(available_seat_id) = find_available_seat_id(&seat_ids) {
    println!("Available seat id: {}", available_seat_id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_finds_seat_row() {
    let (row, col) = find_seat_details("BFFFBBFRRR");
    assert_eq!(row, 70);
    assert_eq!(col, 7);

    let (row, col) = find_seat_details("FFFBBBFRRR");
    assert_eq!(row, 14);
    assert_eq!(col, 7);

    let (row, col) = find_seat_details("BBFFBBFRLL");
    assert_eq!(row, 102);
    assert_eq!(col, 4);
  }
}
