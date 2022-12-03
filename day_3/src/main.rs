use std::collections::HashSet;
use std::io;

fn get_score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        let c = c as u8;
        (c - b'a' + 1).into()
    } else {
        let c = c as u8;
        (c - b'A' + 27).into()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut score = 0;
    let mut score_part_2 = 0;

    let mut current_three_lines = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            current_three_lines.push(user_input.clone());

            let chars = user_input.trim().chars();
            let mut chars_in_left_part = HashSet::new();

            let length = user_input.len();
            let mut repeated_character = 'a';

            for (current_index, c) in chars.enumerate() {
                if current_index < length / 2 {
                    chars_in_left_part.insert(c);
                } else if chars_in_left_part.contains(&c) {
					repeated_character = c;
					break;                    
                }
            }
            score += get_score(repeated_character);

            if current_three_lines.len() == 3 {
                let mut chars_in_all_strings = HashSet::new();
                for (current_inner_index, line) in current_three_lines.iter().enumerate() {
                    let line_chars = line.trim().chars();
                    let mut new_chars_in_all_strings = HashSet::new();
                    for c in line_chars {
                        if current_inner_index == 0 || chars_in_all_strings.contains(&c) {
                            new_chars_in_all_strings.insert(c);
                        }
                    }
                    chars_in_all_strings = new_chars_in_all_strings;
                }
                for c in chars_in_all_strings {
                    score_part_2 += get_score(c);
                }
                current_three_lines.clear();
            }
        }
        user_input.clear();
    }

    println!("result part 1: {}", score);
    println!("result part 2: {}", score_part_2);
}
