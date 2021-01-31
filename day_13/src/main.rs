use shared;

#[derive(Debug)]
struct Timetable {
    pub timestamp: usize,
    pub notes: Vec<String>,
}

impl Timetable {
    pub fn read_from_file(filename: &str) -> Timetable {
        let contents = shared::read_file(filename);
        let timestamp = contents[0].parse::<usize>().unwrap();
        let notes = contents[1].split(",").collect::<Vec<&str>>();

        Timetable {
            timestamp,
            notes: notes.iter().map(|e| e.to_string()).collect::<Vec<String>>(),
        }
    }

    pub fn find_earliest_matching_bus(&self) -> (usize, usize) {
        let valid_bus_ids = self
            .notes
            .iter()
            .filter(|e| (*e).to_string() != "x".to_string())
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut current_timestamp = self.timestamp;

        loop {
            for id in valid_bus_ids.iter() {
                if current_timestamp % id == 0 {
                    return (current_timestamp, *id);
                }
            }

            current_timestamp += 1;
        }
    }

    pub fn calc_timing_value(&self, timestamp_numeric: usize, bus_number: usize) -> usize {
        (timestamp_numeric - self.timestamp) * bus_number
    }
}

fn main() {
    let timetable = Timetable::read_from_file("input.txt");
    let (timestamp, bus_number) = timetable.find_earliest_matching_bus();
    let timing_value = timetable.calc_timing_value(timestamp, bus_number);
    println!("Schedule timing value is {}", timing_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_time_table() {
        let timetable = Timetable::read_from_file("test_input.txt");
        println!("{:?}", timetable);
        assert_eq!(timetable.timestamp, 939);
        assert_eq!(timetable.notes.join(","), "7,13,x,x,59,x,31,19");
    }

    #[test]
    fn it_finds_the_earliest_matching_bus() {
        let timetable = Timetable::read_from_file("test_input.txt");
        let (timestamp, bus_number) = timetable.find_earliest_matching_bus();
        assert_eq!(timestamp, 944);
        assert_eq!(bus_number, 59);
    }

    #[test]
    fn it_calculates_timing_value() {
        let timetable = Timetable::read_from_file("test_input.txt");
        let (timestamp, bus_number) = timetable.find_earliest_matching_bus();
        let timing_value = timetable.calc_timing_value(timestamp, bus_number);
        assert_eq!(timing_value, 295);
    }
}
