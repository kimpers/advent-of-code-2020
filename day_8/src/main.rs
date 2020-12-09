use std::collections::HashSet;

use shared;

#[derive(Debug)]
struct TuringMachine<'a> {
    pub instruction_counter: usize,
    pub accumulator: isize,

    pub execution_code: &'a Vec<String>,
}

impl TuringMachine<'_> {
    fn new<'a>(execution_code: &'a Vec<String>) -> TuringMachine<'a> {
        TuringMachine {
            instruction_counter: 0,
            accumulator: 0,
            execution_code,
        }
    }

    pub fn execute(&mut self, persist: bool) -> (usize, isize) {
        let data = self.execution_code[self.instruction_counter]
            .split(" ")
            .collect::<Vec<&str>>();
        let instruction_identifier = data[0];

        let mut instruction_counter = self.instruction_counter;
        let mut accumulator = self.accumulator;

        match instruction_identifier {
            "nop" => instruction_counter += 1,
            "acc" => {
                accumulator += data[1].parse::<isize>().unwrap();
                instruction_counter += 1;
            }
            "jmp" => {
                let next_instruction =
                    (instruction_counter as isize) + data[1].parse::<isize>().unwrap();

                if next_instruction <= 0 {
                    panic!("Invalid next instruction counter {}", next_instruction)
                }

                instruction_counter = next_instruction as usize;
            }
            instruct_id => panic!("Invalid instruction {}", instruct_id),
        }

        if persist {
            self.instruction_counter = instruction_counter;
            self.accumulator = accumulator;
        }

        (instruction_counter, accumulator)
    }

    pub fn skip(&mut self) {
        self.instruction_counter += 1;
    }

    pub fn reset(&mut self) {
        self.instruction_counter = 0;
        self.accumulator = 0;
    }
}

fn execute_until_loop(tm: &mut TuringMachine) {
    let mut set: HashSet<usize> = HashSet::new();

    loop {
        match set.get(&tm.instruction_counter) {
            Some(_instruction) => return,
            None => {
                set.insert(tm.instruction_counter);
                tm.execute(true);
            }
        }
    }
}

#[allow(unused_assignments)]
fn find_terminating_sequence(tm: &mut TuringMachine) {
    let mut set: HashSet<usize> = HashSet::new();
    let mut skippable_instructions = vec![];

    loop {
        match set.get(&tm.instruction_counter) {
            Some(_instruction) => break,
            None => {
                let code = &tm.execution_code[tm.instruction_counter];
                if code.contains("nop") || code.contains("jmp") {
                    skippable_instructions.push(tm.instruction_counter);
                }
                set.insert(tm.instruction_counter);
                tm.execute(true);
            }
        }
    }

    let mut is_loop = false;
    for skippable in skippable_instructions {
        tm.reset();
        set = HashSet::new();
        is_loop = false;
        while tm.instruction_counter < tm.execution_code.len() {
            match set.get(&tm.instruction_counter) {
                Some(_instruction) => {
                    is_loop = true;

                    break;
                }
                None => {
                    if tm.instruction_counter == skippable {
                        tm.skip();
                    }

                    set.insert(tm.instruction_counter);
                    tm.execute(true);
                }
            }
        }
        if !is_loop {
            return;
        }
    }

    panic!("Could not find a halting execution sequence")
}
fn main() {
    let instructions = shared::read_file("input.txt");
    let mut tm = TuringMachine::new(&instructions);

    execute_until_loop(&mut tm);
    println!("The accumulator value is {} before loop", tm.accumulator);

    let mut tm = TuringMachine::new(&instructions);
    find_terminating_sequence(&mut tm);
    println!(
        "The accumulator value is {} after successful execution",
        tm.accumulator
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_executes_the_instructions() {
        let instructions = shared::read_file("test_input.txt");

        let mut tm = TuringMachine::new(&instructions);
        assert_eq!(tm.instruction_counter, 0);
        assert_eq!(tm.accumulator, 0);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 1);
        assert_eq!(tm.accumulator, 0);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 2);
        assert_eq!(tm.accumulator, 1);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 6);
        assert_eq!(tm.accumulator, 1);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 7);
        assert_eq!(tm.accumulator, 2);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 3);
        assert_eq!(tm.accumulator, 2);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 4);
        assert_eq!(tm.accumulator, 5);

        tm.execute(true);
        assert_eq!(tm.instruction_counter, 1);
        assert_eq!(tm.accumulator, 5);
    }

    #[test]
    fn it_executes_until_loop() {
        let instructions = shared::read_file("test_input.txt");
        let mut tm = TuringMachine::new(&instructions);

        execute_until_loop(&mut tm);
        assert_eq!(tm.instruction_counter, 1);
        assert_eq!(tm.accumulator, 5);
    }

    #[test]
    fn it_finds_terminating_sequence() {
        let instructions = shared::read_file("test_input.txt");
        let mut tm = TuringMachine::new(&instructions);

        find_terminating_sequence(&mut tm);
        assert_eq!(tm.accumulator, 8, "correct final accumulator state")
    }
}
