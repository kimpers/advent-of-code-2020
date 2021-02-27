use std::collections::HashMap;

#[derive(Debug)]
struct SequenceMemory {
    turn_spoken: HashMap<usize, usize>,
    pub turn: usize,
    pub next_num: usize,
}

impl SequenceMemory {
    fn new(initial_sequence: Vec<usize>) -> SequenceMemory {
        let mut turn = 1 as usize;
        let mut turn_spoken: HashMap<usize, usize> = HashMap::new();
        let mut next_num = 0;

        for num in initial_sequence {
            turn_spoken.insert(num, turn);
            turn += 1;
            next_num = num;
        }

        SequenceMemory {
            turn,
            next_num,
            turn_spoken,
        }
    }

    fn play_round(&mut self) {
        let last_turn_num = self.turn - 1;
        let spoken_num = match self.turn_spoken.get(&self.next_num) {
            Some(prev_spoken) => last_turn_num - *prev_spoken,
            _ => 0,
        };

        self.turn_spoken.insert(self.next_num, last_turn_num);
        self.turn += 1;
        self.next_num = spoken_num;
    }

    fn play_rounds(&mut self, num_rounds: usize) -> usize {
        while self.turn <= num_rounds {
            self.play_round();
        }

        self.next_num
    }
}

fn main() {
    let mut seq = SequenceMemory::new(vec![0, 13, 1, 16, 6, 17]);
    let spoken_num = seq.play_rounds(2020);
    println!("2020th spoken num is {}", spoken_num);

    // TODO: optimized solution
    let mut seq = SequenceMemory::new(vec![0, 13, 1, 16, 6, 17]);
    let spoken_num = seq.play_rounds(30000000);
    println!("30000000th spoken num is {}", spoken_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_initial_sequence() {
        let seq = SequenceMemory::new(vec![3, 1, 2]);
        assert_eq!(seq.turn, 4);
        assert_eq!(seq.next_num, 2);
    }

    #[test]
    fn it_plays_round() {
        let mut seq = SequenceMemory::new(vec![0, 3, 6]);
        assert_eq!(seq.turn, 4);
        seq.play_round();
        assert_eq!(seq.next_num, 0);
        assert_eq!(seq.turn, 5);
        seq.play_round();
        assert_eq!(seq.next_num, 3);
        assert_eq!(seq.turn, 6);
    }

    #[test]
    fn it_plays_n_rounds() {
        let mut seq = SequenceMemory::new(vec![0, 3, 6]);
        let num = seq.play_rounds(10);
        assert_eq!(num, 0);
    }
}
