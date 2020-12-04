use std::io::prelude::*;
use std::{fs::File, println};

fn parse_input_file(filename: &str) -> Vec<Vec<String>> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();
  let lines = contents.lines().collect::<Vec<&str>>();

  let matrix = lines.iter().fold(vec![], |mut rows, line| {
    let char_row = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();

    rows.push(char_row);

    rows
  });

  matrix
}

fn count_trees(map: &Vec<Vec<String>>, right_count: usize, down_count: usize) -> usize {
  let max_cols = map[0].len();
  let max_rows = map.len();

  let mut num_trees = 0;
  let mut i = 0;
  let mut j = 0;
  while i < max_rows {
    if map[i][j] == "#" {
      num_trees += 1;
    }

    j = (j + right_count) % max_cols;
    i += down_count;
  }

  num_trees
}

fn trees_for_paths_multiplied(map: &Vec<Vec<String>>, paths: &Vec<(usize, usize)>) -> usize {
  let tree_counts = paths.iter().fold(1, |counts, paths| {
    counts * count_trees(map, paths.0, paths.1)
  });

  tree_counts
}

fn main() {
  let map = parse_input_file("input.txt");
  let num_trees = count_trees(&map, 3, 1);

  println!("Num trees {} on single path", num_trees);

  let paths = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  let multiplied_count = trees_for_paths_multiplied(&map, &paths);
  println!(
    "The product of tree counts for multiple paths is {}",
    multiplied_count
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_parses_the_input_correctly() {
    let grid = parse_input_file("test_input.txt");
    println!("{:#?}", grid);
    assert_eq!(grid.len(), 11, "correct number of rows");
    assert_eq!(grid[0].len(), 66, "correct number of columns");
  }

  #[test]
  fn it_counts_trees_in_map() {
    let grid = parse_input_file("test_input.txt");
    assert_eq!(count_trees(&grid, 3, 1), 7, "correct number of trees")
  }

  #[test]
  fn it_multiplies_trees_in_path() {
    let map = parse_input_file("test_input.txt");
    let paths = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    assert_eq!(
      trees_for_paths_multiplied(&map, &paths),
      336,
      "correct multiplied tree count"
    );
  }
}
