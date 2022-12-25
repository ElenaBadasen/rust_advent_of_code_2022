use std::io;

fn parse(s: &str) -> i64 {
    let mut value = 0;
    for (index, c) in s.chars().rev().enumerate() {
        let coef = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => 0, // should not happen
        };
        value += coef * i64::pow(5, index as u32);
    }
    value
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut result = 0;
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let value = parse(user_input.trim());
            result += value;
            user_input.clear();
        }
    }

    let mut result_vec: Vec<char> = vec![];
    let mut add_to_next_digit = 0;
    for i in 1.. {
        result += add_to_next_digit * i64::pow(5, i - 1);
        let residual = result % i64::pow(5, i);
        let c = match residual / i64::pow(5, i - 1) {
            0 => {
                add_to_next_digit = 0;
                '0'
            }
            1 => {
                add_to_next_digit = 0;
                '1'
            }
            2 => {
                add_to_next_digit = 0;
                '2'
            }
            3 => {
                add_to_next_digit = 1;
                '='
            }
            4 => {
                add_to_next_digit = 1;
                '-'
            }
            _ => '0', //should not happen
        };
        result_vec.insert(0, c);
        result -= residual;
        if result == 0 {
            break;
        }
    }

    let string_result = result_vec.into_iter().collect::<String>();
    println!("result part 1: {}", string_result);
}
