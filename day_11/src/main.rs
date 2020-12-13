use shared;

fn parse_seating_layout(filename: &str) -> Vec<Vec<String>> {
    let lines = shared::read_file(filename);

    lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn check_item(item: &String, (num_vacant, num_taken): (usize, usize)) -> (usize, usize) {
    if item == "L" {
        return (num_vacant + 1, num_taken);
    } else if item == "#" {
        return (num_vacant, num_taken + 1);
    }

    (num_vacant, num_taken)
}

#[derive(Copy, Clone)]
enum Direction {
    Increase = 1,
    Unchanged = 0,
    Decrease = -1,
}

fn find_first_non_empty_seat_in_direction(
    layout: &Vec<Vec<String>>,
    start_row: usize,
    start_col: usize,
    row_direction: Direction,
    col_direction: Direction,
) -> (usize, usize) {
    let row_length = layout.len();
    let col_length = layout[0].len();

    let mut current_row = start_row;
    let mut current_col = start_col;

    loop {
        let new_row: isize = current_row as isize + row_direction as isize;
        let new_col: isize = current_col as isize + col_direction as isize;

        if new_row < 0 || new_row >= row_length as isize {
            return (current_row, current_col);
        } else if new_col < 0 || new_col >= col_length as isize {
            return (current_row, current_col);
        }

        current_row = new_row as usize;
        current_col = new_col as usize;

        if layout[current_row][current_col] != "." {
            return (current_row, current_col);
        }
    }
}
fn check_adjacent_seats(
    layout: &Vec<Vec<String>>,
    curr_item_row: usize,
    curr_item_col: usize,
) -> (usize, usize) {
    let mut counts = (0, 0);

    // Row before
    if curr_item_row > 0 {
        let prev_row = &layout[curr_item_row - 1];

        // Col to the left
        if curr_item_col > 0 {
            let item = &prev_row[curr_item_col - 1];
            counts = check_item(item, counts);
        }

        // Same col
        let item = &prev_row[curr_item_col];
        counts = check_item(item, counts);

        // Col to the right
        if curr_item_col < prev_row.len() - 1 {
            let item = &prev_row[curr_item_col + 1];
            counts = check_item(item, counts);
        }
    }

    // Same row
    {
        let current_row = &layout[curr_item_row];
        // Col to the left
        if curr_item_col > 0 {
            let item = &current_row[curr_item_col - 1];
            counts = check_item(item, counts);
        }

        // Col to the right
        if curr_item_col < current_row.len() - 1 {
            let item = &current_row[curr_item_col + 1];
            counts = check_item(item, counts);
        }
    }

    // Next row
    if curr_item_row < layout.len() - 1 {
        let next_row = &layout[curr_item_row + 1];

        // Col to the left
        if curr_item_col > 0 {
            let item = &next_row[curr_item_col - 1];
            counts = check_item(item, counts);
        }

        // Same col
        let item = &next_row[curr_item_col];
        counts = check_item(item, counts);

        // Col to the right
        if curr_item_col < next_row.len() - 1 {
            let item = &next_row[curr_item_col + 1];
            counts = check_item(item, counts);
        }
    }

    let (num_vacant, num_taken) = counts;

    (num_vacant, num_taken)
}

fn check_non_empty_adjacent_seats(
    layout: &Vec<Vec<String>>,
    curr_item_row: usize,
    curr_item_col: usize,
) -> (usize, usize) {
    let mut to_check = vec![];

    // Row before
    if curr_item_row > 0 {
        // Col to the left
        if curr_item_col > 0 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Decrease,
                Direction::Decrease,
            ));
        }

        // Same col
        to_check.push(find_first_non_empty_seat_in_direction(
            layout,
            curr_item_row,
            curr_item_col,
            Direction::Decrease,
            Direction::Unchanged,
        ));

        // Col to the right
        if curr_item_col < layout[0].len() - 1 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Decrease,
                Direction::Increase,
            ));
        }
    }

    // Same row
    {
        // Col to the left
        if curr_item_col > 0 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Unchanged,
                Direction::Decrease,
            ));
        }

        // Col to the right
        if curr_item_col < layout[0].len() - 1 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Unchanged,
                Direction::Increase,
            ));
        }
    }

    // Next row
    if curr_item_row < layout.len() - 1 {
        // Col to the left
        if curr_item_col > 0 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Increase,
                Direction::Decrease,
            ));
        }

        // Same col
        to_check.push(find_first_non_empty_seat_in_direction(
            layout,
            curr_item_row,
            curr_item_col,
            Direction::Increase,
            Direction::Unchanged,
        ));

        // Col to the right
        if curr_item_col < layout[0].len() - 1 {
            to_check.push(find_first_non_empty_seat_in_direction(
                layout,
                curr_item_row,
                curr_item_col,
                Direction::Increase,
                Direction::Increase,
            ));
        }
    }

    let (num_vacant, num_taken) =
        to_check
            .iter()
            .fold((0, 0), |(num_vacant, num_taken), (row, col)| {
                check_item(&layout[*row][*col], (num_vacant, num_taken))
            });

    (num_vacant, num_taken)
}

fn update_seating(
    layout: &Vec<Vec<String>>,
    check_adjacent_seats: &dyn Fn(&Vec<Vec<String>>, usize, usize) -> (usize, usize),
    num_taken_limit: usize,
) -> (Vec<Vec<String>>, bool) {
    let mut new_layout = vec![];
    let mut has_changed = false;
    for current_row_num in 0..layout.len() {
        let current_row = &layout[current_row_num];
        let mut new_row = vec![];
        for current_col_num in 0..current_row.len() {
            let (_num_vacant, num_taken) =
                check_adjacent_seats(layout, current_row_num, current_col_num);
            let item = &current_row[current_col_num];
            if item == "L" && num_taken == 0 {
                has_changed = true;
                new_row.push("#".to_string());
            } else if item == "#" && num_taken >= num_taken_limit {
                has_changed = true;
                new_row.push("L".to_string());
            } else {
                new_row.push(item.to_string());
            }
        }
        new_layout.push(new_row);
    }

    (new_layout, has_changed)
}

fn count_occupied(layout: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for row in layout {
        for seat in row {
            if seat == "#" {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let layout = parse_seating_layout("input.txt");
    let mut new_layout = layout.clone();
    loop {
        let result = update_seating(&new_layout, &check_adjacent_seats, 4);
        new_layout = result.0;
        let has_changed = result.1;
        if has_changed == false {
            break;
        }
    }
    let num_occupied_seats = count_occupied(&new_layout);
    println!("Num occupied seats in stable layout {}", num_occupied_seats);

    let mut new_layout = layout.clone();
    loop {
        let result = update_seating(&new_layout, &check_non_empty_adjacent_seats, 5);
        new_layout = result.0;
        let has_changed = result.1;
        if has_changed == false {
            break;
        }
    }
    let num_occupied_seats = count_occupied(&new_layout);
    println!(
        "Num occupied seats in stable layout {} when skipping empty adjacent seats",
        num_occupied_seats
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_seating_layout() {
        let layout = parse_seating_layout("test_input.txt");
        assert_eq!(layout[0].len(), 10, "correct row length");
        assert_eq!(layout.len(), 10, "correct number of rows");
        assert_eq!(layout[0].join(""), "L.LL.LL.LL");
        assert_eq!(layout[9].join(""), "L.LLLLL.LL");
    }

    #[test]
    fn it_counts_adjacent_seats() {
        let layout = parse_seating_layout("test_input.txt");
        assert_eq!(check_adjacent_seats(&layout, 0, 0), (2, 0));
        assert_eq!(check_adjacent_seats(&layout, 5, 4), (5, 0));
        assert_eq!(check_adjacent_seats(&layout, 9, 0), (1, 0));
        assert_eq!(check_adjacent_seats(&layout, 9, 9), (2, 0));
    }

    #[test]
    fn it_updates_seating() {
        let mut layout = parse_seating_layout("test_input.txt");
        for _ in 0..5 {
            let result = update_seating(&layout, &check_adjacent_seats, 4);
            layout = result.0;
            assert_eq!(result.1, true);
        }
        let (layout, has_changed) = update_seating(&layout, &check_adjacent_seats, 4);

        let expected_result = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.#.L..#..",
            "#L##.##.L#",
            "#.#L.LL.LL",
            "#.#L#L#.##",
            "..L.L.....",
            "#L#L##L#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ];

        assert_eq!(has_changed, false);
        assert_eq!(
            layout.iter().map(|r| r.join("")).collect::<Vec<String>>(),
            expected_result
        )
    }

    #[test]
    fn it_counts_occupied_seats() {
        let mut layout = parse_seating_layout("test_input.txt");
        for _ in 0..5 {
            let result = update_seating(&layout, &check_adjacent_seats, 4);
            layout = result.0;
            assert_eq!(result.1, true);
        }

        assert_eq!(count_occupied(&layout), 37);
    }

    #[test]
    fn it_counts_occupied_seats_skipping_non_empty() {
        let mut layout = parse_seating_layout("test_input.txt");
        for _ in 0..6 {
            let result = update_seating(&layout, &check_non_empty_adjacent_seats, 5);
            layout = result.0;
            assert_eq!(result.1, true);
        }

        assert_eq!(count_occupied(&layout), 26);
    }

    #[test]
    fn it_finds_the_first_non_empty_seat_in_the_direction() {
        let mut layout = shared::parse_input_to_character_matrix(
            "
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....",
        );

        assert_eq!(
            find_first_non_empty_seat_in_direction(
                &layout,
                4,
                3,
                Direction::Decrease,
                Direction::Decrease
            ),
            (2, 1)
        );

        layout = shared::parse_input_to_character_matrix(
            "
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.",
        );

        assert_eq!(
            find_first_non_empty_seat_in_direction(
                &layout,
                3,
                3,
                Direction::Decrease,
                Direction::Decrease
            ),
            (0, 0)
        )
    }
}
