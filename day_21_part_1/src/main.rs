use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, Clone)]
enum OperationType {
    Number,
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Monkey {
    name: String,
    operation: OperationType,
    value_1_link: String,
    value_2_link: String,
    result_value: Option<i64>,
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let mut current_monkeys: HashSet<String> = HashSet::new();
    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let split_input: Vec<&str> = user_input.trim().split(' ').collect();
            let mut value_1_link = "".to_string();
            let mut value_2_link = "".to_string();
            let mut result_value: Option<i64> = None;
            let operation = if split_input.len() < 4 {
                result_value = Some(split_input[1].parse().unwrap());
                OperationType::Number
            } else {
                value_1_link = split_input[1].to_string();
                value_2_link = split_input[3].to_string();
                match split_input[2] {
                    "+" => OperationType::Add,
                    "-" => OperationType::Substract,
                    "*" => OperationType::Multiply,
                    "/" => OperationType::Divide,
                    _default => OperationType::Number, //should not happen
                }
            };
            let monkey = Monkey {
                name: split_input[0].replace(':', ""),
                operation: operation.clone(),
                value_1_link: value_1_link.clone(),
                value_2_link: value_2_link.clone(),
                result_value,
            };

            if matches!(operation, OperationType::Number) {
                current_monkeys.insert(monkey.name.clone());
            } else {
                dependencies
                    .entry(value_1_link)
                    .or_default()
                    .insert(monkey.name.clone());
                dependencies
                    .entry(value_2_link)
                    .or_default()
                    .insert(monkey.name.clone());
            }

            monkeys.insert(monkey.name.clone(), monkey);

            user_input.clear();
        }
    }

    let mut finished = false;
    let mut result = 0;
    while !finished {
        let mut new_current_monkeys: HashSet<String> = HashSet::new();
        for name in &current_monkeys {
            let monkey = monkeys.get(name).unwrap();
            if !matches!(monkey.operation, OperationType::Number) {
                let link_1 = monkey.value_1_link.clone();
                match monkeys.get(&link_1).unwrap().result_value {
                    None => {
                        continue;
                    }
                    Some(result_1) => {
                        let link_2 = monkey.value_2_link.clone();
                        match monkeys.get(&link_2).unwrap().result_value {
                            None => {
                                continue;
                            }
                            Some(result_2) => {
                                let monkey = monkeys.get_mut(name).unwrap();
                                monkey.result_value = match monkey.operation {
                                    OperationType::Number => {
                                        //should not happen
                                        None
                                    }
                                    OperationType::Add => Some(result_1 + result_2),
                                    OperationType::Substract => Some(result_1 - result_2),
                                    OperationType::Multiply => Some(result_1 * result_2),
                                    OperationType::Divide => Some(result_1 / result_2),
                                };
                                if name == "root" {
                                    result = monkey.result_value.unwrap();
                                    finished = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            for dependent_monkey_name in dependencies.entry(name.clone()).or_default().iter() {
                new_current_monkeys.insert(dependent_monkey_name.clone());
            }
        }
        current_monkeys = new_current_monkeys;
    }
    println!("result part 1: {}", result);
}
