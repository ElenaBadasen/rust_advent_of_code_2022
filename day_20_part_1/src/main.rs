use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    //vec of [value, initial position]
    let mut message: Vec<[i32; 2]> = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let value = user_input.trim().parse().unwrap();
            message.push([value, message.len() as i32]);
            user_input.clear();
        }
    }

    let len = message.len() as i32;
    for i in 0..len {
        let initial_position = message.iter().position(|m| m[1] == i as i32).unwrap();
        let element = message.remove(initial_position);
        //println!("HERE");
        //println!("insertion index: {}", (initial_position as i32 + element[0]).rem_euclid(len - 1));
        let mut insertion_index = (initial_position as i32 + element[0]).rem_euclid(len - 1);
        if insertion_index == 0 {
            insertion_index = len - 1;
        }
        message.insert(insertion_index as usize, element);
    }
    //println!("message: {:?}", message);
    let zero_position = message.iter().position(|m| m[0] == 0_i32).unwrap();
    let result = message[(zero_position + 1000) % len as usize][0]
        + message[(zero_position + 2000) % len as usize][0]
        + message[(zero_position + 3000) % len as usize][0];
    println!("result: {}", result);
}
