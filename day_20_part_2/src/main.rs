use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    //vec of [value, initial position]
    let mut message: Vec<[i64; 2]> = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let value: i64 = user_input.trim().parse().unwrap();
            message.push([value * 811589153, message.len() as i64]);
            user_input.clear();
        }
    }

    let len = message.len() as i64;
    for _cycle in 0..10 {
        for i in 0..len {
            let initial_position = message.iter().position(|m| m[1] == i as i64).unwrap();
            let element = message.remove(initial_position);
            let mut insertion_index = (initial_position as i64 + element[0]).rem_euclid(len - 1);
            if insertion_index == 0 {
                insertion_index = len - 1;
            }
            message.insert(insertion_index as usize, element);
        }
    }
    let zero_position = message.iter().position(|m| m[0] == 0_i64).unwrap();
    let result = message[(zero_position + 1000) % len as usize][0]
        + message[(zero_position + 2000) % len as usize][0]
        + message[(zero_position + 3000) % len as usize][0];
    println!("result: {}", result);
}
