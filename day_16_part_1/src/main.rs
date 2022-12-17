use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Valve {
	name: String,
	flow_rate: u32,
	next_valves: HashSet<String>,
}

#[derive(Clone, Debug)]
struct PathStep {
	step: String,
	open_valves: HashSet<String>,
	visited_options: HashSet<String>,
	last_open_valve: String,
	last_open_valve_index: usize,
}

fn pressure(paths: &[PathStep], valves: &HashMap<String, Valve>, steps: u32) -> u32 {
	let mut pressure = 0;
	let mut current_valve_name = "AA".to_string();
	let mut open_valves_flows_sum = 0;
	for i in 1..(steps + 1) {
		pressure += open_valves_flows_sum;
		match paths.get(i as usize) {
			None => {},
			Some(s) => {
				if s.step == "open" {
					open_valves_flows_sum += valves.get(&current_valve_name).unwrap().flow_rate;
				} else {
					current_valve_name = s.step.clone();
				}
			},
		}
	}
	pressure
}

fn find_shortest_path(left_valve: &str, right_valve: &str, valves: &HashMap<String, Valve>) -> u32 {
	let mut current_valves: HashSet<String> = HashSet::new();
	let mut visited_valves: HashSet<String> = HashSet::new();
	current_valves.insert(left_valve.to_string());
	let mut current_index = 1;
	let mut result = valves.len();
	loop {
		visited_valves.extend(current_valves.clone());
		let mut new_current_valves = HashSet::new();
		for valve in current_valves {
			new_current_valves.extend(valves.get(&valve).unwrap().next_valves.clone());
		}
		new_current_valves = &new_current_valves - &visited_valves;
		if new_current_valves.contains(right_valve) {
			result = current_index;
			break;
		} else if new_current_valves.is_empty() {
			break;
		} else {
			current_valves = new_current_valves;
		}
		current_index += 1;
	}
	return result as u32;
}

fn process(steps: u32, valves: &HashMap<String, Valve>) -> u32 {
	let mut paths: Vec<PathStep> = vec![PathStep{
		step: "AA".to_string(),
		open_valves: HashSet::new(), 
		visited_options: HashSet::new(),
		last_open_valve: "".to_string(),
		last_open_valve_index: 0,
	}];
    let mut current_valve = "AA".to_string();
    let mut max_pressure = 0;
    
    let mut shortest_paths: HashMap<String, u32> = HashMap::new();
    let valves_len = valves.len();
    for (name1, left_valve) in valves {
		if left_valve.flow_rate == 0 {
			if name1 != "AA" {
				continue;
			}
		}
		for (name2, _right_valve) in valves {
			let min_path_len = if name1 == name2 {
				0
			} else {
				find_shortest_path(name1, name2, valves)
			};
			shortest_paths.insert(name1.clone() + &name2, min_path_len);
		}
	}
    let mut processed_paths = 0;
    loop {
		let len = paths.len();
		if len > steps as usize {
			//we are at the end of a path, time is out
			//but we will skip non-productive tails, they do not add to meanungful result
			if paths.last().unwrap().step == "open" {
				let pressure = pressure(&paths, &valves, steps);
				processed_paths += 1;
				if pressure > max_pressure {
					max_pressure = pressure;
				}
			}
			paths.pop();
			
			for p in paths.iter().rev() {
				if "open" != p.step {
					current_valve = p.step.clone();
					break;
				}
			}
			continue;
		}
		if paths.is_empty() {
			break;
		}
	
		let path_step = paths.get_mut(len - 1).unwrap();
		let current_valve_struct = valves.get(&current_valve).unwrap();
		let mut options: HashSet<String> = current_valve_struct.next_valves.clone();
		
		if path_step.step != "open" 
		&& !path_step.open_valves.contains(&current_valve) 
		&& current_valve_struct.flow_rate != 0
		{				
			options.insert("open".to_string());
		}
		
		options = &options - &path_step.visited_options;
		if options.is_empty() {
			//we are at the end of a path, no more ways to go
			//but we will skip non-productive tails, they do not add to meanungful result
			if paths.last().unwrap().step == "open" {
				let pressure = pressure(&paths, &valves, steps);
				processed_paths += 1;
				if pressure > max_pressure {
					max_pressure = pressure;
				}
			}
			paths.pop();
			if !paths.is_empty() {
				for p in paths.iter().rev() {
					if "open" != p.step {
						current_valve = p.step.clone();
						break;
					}
				}
			}
		} else {
			let option = options.iter().next().unwrap();
			path_step.visited_options.insert(option.clone());
			let mut new_open_valves = path_step.open_valves.clone();
			let mut new_last_open_valve = path_step.last_open_valve.clone();
			let mut new_last_open_valve_index = path_step.last_open_valve_index.clone();
			if option == "open" {
				new_open_valves.insert(current_valve.clone());
				new_last_open_valve = current_valve.clone();
				new_last_open_valve_index = len - 1;
			} else {
				//should check for optimal path
				let left_part = if path_step.last_open_valve == "" {
					"AA".to_string()
				} else {
					path_step.last_open_valve.clone()
				};
				
				let small_path_len = if path_step.last_open_valve == "" {
					len
				} else {
					len - path_step.last_open_valve_index - 1
				};
				
				let shortest_path = *shortest_paths.get(&(left_part.clone() + &option)).unwrap();
				if small_path_len as u32 > shortest_path {
					//do not go to option, not optimal way
					continue;
				}
				
				current_valve = option.clone();
			}
			
			let new_path_step = PathStep{ 
				step: option.clone(), 
				open_valves: new_open_valves,  
				visited_options: HashSet::new(),
				last_open_valve: new_last_open_valve,
				last_open_valve_index: new_last_open_valve_index,
			};
			paths.push(new_path_step);
		}
	}
	println!("processed_paths: {}", processed_paths);
	max_pressure
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut valves: HashMap<String, Valve> = HashMap::new();
    
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let input: Vec<&str> = user_input.trim().split(' ').collect();
            let name = input[1];
            let flow_rate: u32 = input[4].split('=').collect::<Vec<&str>>()[1].replace(';', "").parse().unwrap();
            let mut next_valves: HashSet<String> = HashSet::new();
            for item in input.iter().skip(9) {
				next_valves.insert(item.replace(',', "").to_string());
			}
            valves.insert(name.to_string(), Valve{ name: name.to_string(), flow_rate, next_valves });
            user_input.clear();
        }
    }
    
    let max_pressure = process(30, &valves);
	println!("max_pressure: {}", max_pressure);
}
