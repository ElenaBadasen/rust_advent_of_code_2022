use serde_json::Value;
use std::cmp::Ordering;
use std::io;

fn is_ordered(left: &Value, right: &Value) -> Option<bool> {
    match left {
        Value::Number(n_left) => match right {
            Value::Number(n_right) => match n_left.as_u64().cmp(&n_right.as_u64()) {
                Ordering::Less => Some(true),
                Ordering::Equal => None,
                Ordering::Greater => Some(false),
            },
            Value::Array(arr_right) => is_ordered(
                &Value::Array(vec![Value::Number(n_left.clone())]),
                &Value::Array(arr_right.clone()),
            ),
            _default => None,
        },
        Value::Array(arr_left) => match right {
            Value::Number(n_right) => is_ordered(
                &Value::Array(arr_left.clone()),
                &Value::Array(vec![Value::Number(n_right.clone())]),
            ),
            Value::Array(arr_right) => {
                if arr_left.is_empty() && !arr_right.is_empty() {
                    return Some(true);
                }
                for (index, v) in arr_left.iter().enumerate() {
                    if index < arr_right.len() {
                        match is_ordered(v, &arr_right[index]) {
                            None => {
                                continue;
                            }
                            Some(result) => {
                                return Some(result);
                            }
                        }
                    } else {
                        return Some(false);
                    }
                }
                if arr_left.len() < arr_right.len() {
                    Some(true)
                } else {
                    None
                }
            }
            _default => None,
        },
        _default => None,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut pair: [Value; 2] = [
        serde_json::from_str("[]").unwrap(),
        serde_json::from_str("[]").unwrap(),
    ];
    let mut pair_index = 1;
    let mut inner_pair_index = 0;
    let mut pair_indices = vec![];
    let mut packets = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else if user_input == "\n" {
            //compare the pair
            if is_ordered(&pair[0], &pair[1]).unwrap() {
                pair_indices.push(pair_index);
            }
            pair_index += 1;
        } else {
            let v: Value = serde_json::from_str(user_input.trim()).unwrap();
            pair[inner_pair_index] = v.clone();
            packets.push(v);
            inner_pair_index = (inner_pair_index + 1) % 2;
            user_input.clear();
        }
    }
    if is_ordered(&pair[0], &pair[1]).unwrap() {
        pair_indices.push(pair_index);
    }

    println!("result part 1: {}", pair_indices.iter().sum::<u32>());

    let first_marker: Value = serde_json::from_str("[[2]]").unwrap();
    let second_marker: Value = serde_json::from_str("[[6]]").unwrap();
    packets.push(first_marker.clone());
    packets.push(second_marker.clone());
    packets.sort_unstable_by(|a, b| {
        let result = is_ordered(a, b).unwrap();
        if result {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let first_marker_pos = packets.iter().position(|p| *p == first_marker).unwrap() + 1;
    let second_marker_pos = packets.iter().position(|p| *p == second_marker).unwrap() + 1;

    println!("result part 2: {:?}", first_marker_pos * second_marker_pos);
}
