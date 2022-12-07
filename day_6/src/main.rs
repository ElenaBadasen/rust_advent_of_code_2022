use std::io;

struct StartMarkerCandidate {
    candidate_vector: Vec<char>,
    buffer_length: u32,
}

impl StartMarkerCandidate {
    fn new(buffer_length: u32) -> StartMarkerCandidate {
        StartMarkerCandidate {
            candidate_vector: vec![],
            buffer_length,
        }
    }

    fn add(&mut self, c: char) -> bool {
        if let Some(pos) = self.candidate_vector.iter().position(|v| v == &c) {
            let new_candidate_vector = self.candidate_vector.split_off(pos + 1);
            self.candidate_vector = new_candidate_vector;
            self.candidate_vector.push(c);
        } else {
            self.candidate_vector.push(c);
            if self.candidate_vector.len() == self.buffer_length.try_into().unwrap() {
                return true;
            }
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    let mut number = 1;
    let mut number_part_1 = 1;
    let mut number_part_2 = 1;
    let mut candidate_1 = StartMarkerCandidate::new(4);
    let mut candidate_2 = StartMarkerCandidate::new(14);
    let mut number_part_1_found = false;

    for c in user_input.trim().chars() {
        if !number_part_1_found && candidate_1.add(c) {
            number_part_1 = number;
            number_part_1_found = true;
        }
        if candidate_2.add(c) {
            number_part_2 = number;
            break;
        }
        number += 1;
    }

    println!("result part 1: {}", number_part_1);
    println!("result part 2: {}", number_part_2);
}
