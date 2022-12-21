use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, Clone)]
enum OperationType {
    Number,
    Add,
    Substract,
    Multiply,
    Divide,
    Equals,
}

#[derive(Debug, Clone)]
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
            let mut operation = if split_input.len() < 4 {
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
            let name = split_input[0].replace(':', "");
            if name == "root" {
                operation = OperationType::Equals
            }
            let monkey = Monkey {
                name,
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

    let backup_monkeys = monkeys.clone();
    let backup_current_monkeys = current_monkeys.clone();
    let mut really_finished = false;
    let mut result_part_2 = 0;
    let mut i = i64::pow(2, 40);
    let mut step = i;
    let mut prev_dir_up = true;
    while !really_finished {
        monkeys = backup_monkeys.clone();
        current_monkeys = backup_current_monkeys.clone();
        monkeys.get_mut(&"humn".to_string()).unwrap().result_value = Some(i);
        let mut finished = false;
        while !finished {
            let mut new_current_monkeys: HashSet<String> = HashSet::new();
            for name in &current_monkeys {
                let monkey = monkeys.get(name).unwrap();
                if !matches!(monkey.operation, OperationType::Number) {
                    let link_1 = monkey.value_1_link.clone();
                    let link_2 = monkey.value_2_link.clone();

                    let result_1_option = monkeys.get(&link_1).unwrap().result_value;
                    let result_2_option = monkeys.get(&link_2).unwrap().result_value;

                    match result_1_option {
                        None => {
                            continue;
                        }
                        Some(result_1) => {
                            match result_2_option {
                                None => {
                                    continue;
                                }
                                Some(result_2) => {
                                    if name == "root" {
                                        if result_1 == result_2 {
                                            really_finished = true;
                                            result_part_2 = i;
                                        } else {
                                            //so I checked several values manually and found out
                                            //the result of "wdzt" grows when i gets smaller.
                                            //it all possibly can be automatized too

                                            //here "dffc" doesn't depend on humn, but "wdzt" does
                                            if link_1 == *"dffc" {
                                                if result_1 > result_2 {
                                                    //make i smaller
                                                    if prev_dir_up {
                                                        step /= 2;
                                                        prev_dir_up = false;
                                                    }
                                                    i -= step;
                                                } else {
                                                    //make i greater
                                                    if !prev_dir_up {
                                                        step /= 2;
                                                        prev_dir_up = true;
                                                    }
                                                    i += step;
                                                }
                                            } else {
                                                if result_1 < result_2 {
                                                    //make i smaller

                                                    if prev_dir_up {
                                                        step /= 2;
                                                        prev_dir_up = false;
                                                    }
                                                    i -= step;
                                                } else {
                                                    //make i greater
                                                    if !prev_dir_up {
                                                        step /= 2;
                                                        prev_dir_up = true;
                                                    }
                                                    i += step;
                                                }
                                            }
                                        }
                                        finished = true;
                                        break;
                                    } else {
                                        let monkey = monkeys.get_mut(name).unwrap();
                                        monkey.result_value = match monkey.operation {
                                            OperationType::Number => {
                                                //should not happen
                                                None
                                            }
                                            OperationType::Equals => {
                                                //should not happen
                                                None
                                            }
                                            OperationType::Add => Some(result_1 + result_2),
                                            OperationType::Substract => Some(result_1 - result_2),
                                            OperationType::Multiply => Some(result_1 * result_2),
                                            OperationType::Divide => Some(result_1 / result_2),
                                        };
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
    }

    println!("result part 2: {}", result_part_2);
}
