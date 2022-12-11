use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, Clone)]
enum OpType {
    Multiply,
    Add,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    items_part_2: Vec<HashMap<u32, u32>>,
    operation_type: OpType,
    operation_value: u32,
    test_value: u32,
    test_true: u32,
    test_false: u32,
}

impl Monkey {
    fn process(&mut self) -> HashMap<u32, Vec<u32>> {
        let mut result: HashMap<u32, Vec<u32>> = HashMap::new();
        for item in &self.items {
            let mut new_item_value = match self.operation_type {
                OpType::Multiply => item * self.operation_value,
                OpType::Add => item + self.operation_value,
                OpType::Square => item * item,
            };
            new_item_value /= 3;
            let new_monkey_index = if new_item_value % self.test_value == 0 {
                self.test_true
            } else {
                self.test_false
            };
            let entry = result.entry(new_monkey_index).or_default();
            entry.push(new_item_value);
        }
        self.items = vec![];
        result
    }

    fn process_part_2(
        &mut self,
        test_values: &HashSet<u32>,
    ) -> HashMap<u32, Vec<HashMap<u32, u32>>> {
        let mut result: HashMap<u32, Vec<HashMap<u32, u32>>> = HashMap::new();
        for item in &self.items_part_2 {
            let mut new_item_value: HashMap<u32, u32> = HashMap::new();
            for value in test_values {
                new_item_value.insert(
                    *value,
                    match self.operation_type {
                        OpType::Multiply => {
                            (item.get(value).unwrap() * self.operation_value) % value
                        }
                        OpType::Add => (item.get(value).unwrap() + self.operation_value) % value,
                        OpType::Square => {
                            (item.get(value).unwrap() * item.get(value).unwrap()) % value
                        }
                    },
                );
            }
            let new_monkey_index = if *new_item_value.get(&self.test_value).unwrap() == 0 {
                self.test_true
            } else {
                self.test_false
            };
            let entry = result.entry(new_monkey_index).or_default();
            entry.push(new_item_value);
        }
        self.items_part_2 = vec![];
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut items: Vec<u32> = vec![];
    let mut operation_type = OpType::Multiply;
    let mut operation_value = 0;
    let mut test_value = 0;
    let mut test_true = 0;
    let mut test_false = 0;

    let mut monkeys = vec![];
    let mut test_values = HashSet::new();

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            if user_input == "\n" {
                monkeys.push(Monkey {
                    items: items.clone(),
                    items_part_2: vec![],
                    operation_type: operation_type.clone(),
                    operation_value,
                    test_value,
                    test_true,
                    test_false,
                });
            } else {
                let command: Vec<&str> = user_input.trim().split(' ').collect();
                match command[0] {
                    "Monkey" => {}
                    "Starting" => {
                        items = vec![];
                        for item in command.iter().skip(2) {
                            items.push(item.replace(',', "").parse().unwrap());
                        }
                    }
                    "Operation:" => {
                        if command[4] == "+" {
                            operation_type = OpType::Add;
                            operation_value = command[5].parse().unwrap();
                        } else if command[5] == "old" {
                            operation_type = OpType::Square;
                            operation_value = 0;
                        } else {
                            operation_type = OpType::Multiply;
                            operation_value = command[5].parse().unwrap();
                        }
                    }
                    "Test:" => {
                        test_value = command[3].parse().unwrap();
                        test_values.insert(test_value);
                    }
                    "If" => {
                        if command[1] == "true:" {
                            test_true = command[5].parse().unwrap();
                        } else {
                            test_false = command[5].parse().unwrap();
                        }
                    }
                    _default => {}
                }
            }
            user_input.clear();
        }
    }
    monkeys.push(Monkey {
        items,
        items_part_2: vec![],
        operation_type,
        operation_value,
        test_value,
        test_true,
        test_false,
    });
    let mut monkeys_part_2 = monkeys.clone();

    let mut inspected_items = vec![0; monkeys.len()];
    for _cycle in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let new_items_subdivision = monkey.process();
            for (monkey_num, v) in new_items_subdivision {
                *inspected_items.get_mut(i).unwrap() += v.len();
                let new_monkey = monkeys.get_mut(monkey_num as usize).unwrap();
                new_monkey.items.extend(&v);
            }
        }
    }
    inspected_items.sort_unstable();
    inspected_items.reverse();
    println!("result part 1: {}", inspected_items[0] * inspected_items[1]);

    for i in 0..monkeys_part_2.len() {
        let monkey = monkeys_part_2.get_mut(i).unwrap();
        for item in &monkey.items {
            let mut residuals = HashMap::new();
            for x in &test_values {
                residuals.insert(*x, item % x);
            }
            monkey.items_part_2.push(residuals)
        }
    }

    let mut inspected_items_part_2 = vec![0; monkeys_part_2.len()];
    for _cycle in 0..10000 {
        for i in 0..monkeys_part_2.len() {
            let monkey = monkeys_part_2.get_mut(i).unwrap();
            let new_items_subdivision = monkey.process_part_2(&test_values);
            for (monkey_num, v) in new_items_subdivision {
                *inspected_items_part_2.get_mut(i).unwrap() += v.len();
                let new_monkey = monkeys_part_2.get_mut(monkey_num as usize).unwrap();
                new_monkey.items_part_2.extend(v);
            }
        }
    }
    inspected_items_part_2.sort_unstable();
    inspected_items_part_2.reverse();
    println!(
        "result part 2: {}",
        inspected_items_part_2[0] * inspected_items_part_2[1]
    );
}
