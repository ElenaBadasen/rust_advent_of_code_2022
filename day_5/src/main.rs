use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut answer = "".to_string();
    let mut answer_part_2 = "".to_string();

    let mut stacks_lines: Vec<String> = vec![];
    let mut stacks = vec![];
    let mut stacks_part_2 = vec![];
    let mut initial_stacks_filled = false;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else if user_input.starts_with(" 1") {
            let numbers: Vec<usize> = user_input
                .trim()
                .split("   ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            for _number in &numbers {
                stacks.push(vec![]);
            }
            for line in stacks_lines.iter().rev() {
                let mut number = 1;
                for (index, c) in line.chars().enumerate() {
                    if index == 4 * (number - 1) + 1 {
                        if c != ' ' {
                            stacks.get_mut(number - 1).unwrap().push(c);
                        }
                        number += 1;
                    }
                }
            }
            stacks_part_2 = stacks.clone();
            initial_stacks_filled = true;
            stdin.read_line(&mut user_input).unwrap(); //just reading the empty line that should be in input
        } else if initial_stacks_filled {
            let command: Vec<&str> = user_input.trim().split(' ').collect();
            let count = command[1].parse::<usize>().unwrap();
            let from = command[3].parse::<usize>().unwrap() - 1;
            let to = command[5].parse::<usize>().unwrap() - 1;

            //part 1
            for _i in 0..count {
                let item = stacks.get_mut(from).unwrap().pop().unwrap();
                stacks.get_mut(to).unwrap().push(item);
            }

            //part 2
            let v = stacks_part_2.get_mut(from).unwrap();
            let mut tmp_stack_holder = v.split_off(v.len() - count);
            stacks_part_2
                .get_mut(to)
                .unwrap()
                .append(&mut tmp_stack_holder);
        } else {
            stacks_lines.push(user_input.clone());
        }
        user_input.clear();
    }

    for mut v in stacks {
        answer.push(v.pop().unwrap());
    }

    for mut v in stacks_part_2 {
        answer_part_2.push(v.pop().unwrap());
    }

    println!("result part 1: {}", answer);
    println!("result part 2: {}", answer_part_2);
}
