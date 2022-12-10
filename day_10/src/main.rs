use std::io;

struct Register {
    x: i32,
    cycles_counter: u32,
    result: i32,
    result2: String,
    interesting_cycles: Vec<u32>,
    horizontal_position: u32,
}

impl Register {
    fn new(x: i32) -> Register {
        let mut interesting_cycles = vec![];
        for i in 0..6 {
            interesting_cycles.push(20 + i * 40);
        }
        Register {
            x,
            cycles_counter: 0,
            result: 0,
            result2: "\n".to_string(),
            interesting_cycles,
            horizontal_position: 0,
        }
    }

    fn signal_strength(&self) -> i32 {
        self.x * (self.cycles_counter as i32)
    }

    fn tick_cycle(&mut self, add_x: i32) {
        self.cycles_counter += 1;
        if self.interesting_cycles.contains(&self.cycles_counter) {
            self.result += self.signal_strength();
        }
        let symbol = if (self.x - self.horizontal_position as i32).abs() <= 1 {
            '#'
        } else {
            '.'
        };
        self.result2.push(symbol);
        self.horizontal_position += 1;
        if self.cycles_counter % 40 == 0 {
            self.result2.push('\n');
            self.horizontal_position = 0;
        }
        self.x += add_x;
    }

    fn add(&mut self, add_x: i32) {
        self.tick_cycle(0);
        self.tick_cycle(add_x);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut register = Register::new(1);

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let command: Vec<&str> = user_input.trim().split(' ').collect();
            if command[0] == "noop" {
                register.tick_cycle(0);
            } else {
                register.add(command[1].parse().unwrap());
            }
            if register.cycles_counter == 240 {
                break;
            }
            user_input.clear();
        }
    }

    println!("result part 1: {}", register.result);
    println!("result part 2: {}", register.result2);
}
