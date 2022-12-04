use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut count = 0;
    let mut count_part_2 = 0;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let chars = user_input.trim().chars();
            let mut numbers: Vec<u32> = vec![];
            let mut number_string = String::new();
            for c in chars {
                if ['-', ','].contains(&c) {
                    numbers.push(number_string.parse().expect("Wrong input!"));
                    if numbers.len() == 4 {
                        break;
                    }
                    number_string.clear();
                } else {
                    number_string.push(c);
                }
            }
            if !number_string.is_empty() {
                numbers.push(number_string.parse().expect("Wrong input!"));
            }
            if numbers[0] <= numbers[2] && numbers[1] >= numbers[3]
                || numbers[0] >= numbers[2] && numbers[1] <= numbers[3]
            {
                count += 1;
            }

            if numbers[2] <= numbers[1] && numbers[0] <= numbers[3] {
                count_part_2 += 1;
            }
        }
        user_input.clear();
    }

    println!("result part 1: {}", count);
    println!("result part 2: {}", count_part_2);
}
