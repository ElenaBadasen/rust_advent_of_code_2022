//this is actually a slow solution, but I decided not to try and redo everything

use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Valve {
	name: u32,
	flow_rate: u32,
	next_valves: HashSet<u32>,
}

#[derive(Clone, Debug)]
struct PathStep {
	step: u32, //open is 1
	next_option_index: usize,
	last_open_valve: u32,
	last_open_valve_index: usize,
}

fn pressure(paths: &[&Vec<PathStep>; 2], valves: &HashMap<u32, Valve>, steps: u32) -> u32 {
	let mut pressure = 0;
	for item in paths.iter().take(2) {
		let mut current_valve_name = ('A' as u32) * 100 + 'A' as u32;
		let mut open_valves_flows_sum = 0;
		for i in 1..(steps + 1) {
			pressure += open_valves_flows_sum;
			match item.get(i as usize) {
				None => {},
				Some(s) => {
					if s.step == 1 {
						open_valves_flows_sum += valves.get(&current_valve_name).unwrap().flow_rate;
					} else {
						current_valve_name = s.step;
					}
				},
			}
		}
	}
	pressure
}

const AA: u32 = ('A' as u32) * 100 + 'A' as u32;

fn process_inner(my_paths: &Vec<PathStep>, 
	valves: &HashMap<u32, Valve>, 
	steps: u32, 
	shortest_paths: &HashMap<[u32; 2], u32>,
	need_to_go_deeper: bool,
	open_valves: &[u32],
	valve_options: &HashMap<u32, Vec<u32>>,
	) -> u32 {
		
	let mut elephant_paths: Vec<PathStep> = vec![PathStep{
		step: AA, 
		next_option_index: 0,
		last_open_valve: 0,
		last_open_valve_index: 0,
	}];
	let mut elephant_current_valve = AA;
	let mut max_pressure: u32 = 0;
	let mut new_open_valves: Vec<u32> = vec![];
	let mut processed_paths_outer = 0;

	loop {
		if need_to_go_deeper && processed_paths_outer % 1000 == 0 {
			println!("processed_paths_outer {}", processed_paths_outer);
		}
		if elephant_paths.is_empty() {
			break;
		}
		let elephant_len = elephant_paths.len();
		
		//total non-zero valves count for elephant 
		//is 15, so we can stop going now
		if new_open_valves.len() + open_valves.len() == 15 {	
			let pressure = if need_to_go_deeper {
				pressure(&[&elephant_paths, &vec![]], valves, steps)
			} else {
				pressure(&[my_paths, &elephant_paths], valves, steps)
			};
			
			if need_to_go_deeper {
				processed_paths_outer += 1;
			}
			
			if pressure > max_pressure {
				max_pressure = pressure;
			}
			let popped_path_step = elephant_paths.pop().unwrap();
			if popped_path_step.step == 1 {
				new_open_valves.pop();
			}
			
			for p in elephant_paths.iter().rev() {
				if 1 != p.step {
					elephant_current_valve = p.step;
					break;
				}
			}
			continue;
		}
		
		if elephant_len > steps as usize {
			//we are at the end of a path, time is out, but 
			//valves count is not met
			if elephant_paths.last().unwrap().step == 1 {		
				let pressure = if need_to_go_deeper {
					process_inner(&elephant_paths, valves, steps, shortest_paths, false, &new_open_valves, valve_options)
				} else {
					pressure(&[my_paths, &elephant_paths], valves, steps)
				};
				if need_to_go_deeper {
					processed_paths_outer += 1;
				}
				
				if pressure > max_pressure {
					max_pressure = pressure;
				}
			}
			let popped_path_step = elephant_paths.pop().unwrap();
			if popped_path_step.step == 1 {
				new_open_valves.pop();
			}

			for p in elephant_paths.iter().rev() {
				if 1 != p.step {
					elephant_current_valve = p.step;
					break;
				}
			}
			continue;
		}
		
		let elephant_path_step = elephant_paths.get_mut(elephant_len - 1).unwrap();
		let elephant_current_valve_struct = valves.get(&elephant_current_valve).unwrap();

		if elephant_path_step.next_option_index >= valve_options.get(&elephant_current_valve).unwrap().len() {
			//we are at the end of a path, no more ways to go
			//but we will skip non-productive tails, they do not add to meanungful result
			if elephant_paths.last().unwrap().step == 1 {
				let pressure = if need_to_go_deeper {
					process_inner(&elephant_paths, valves, steps, shortest_paths, false, &new_open_valves, valve_options)
				} else {
					pressure(&[my_paths, &elephant_paths], valves, steps)
				};
				
				if need_to_go_deeper {
					processed_paths_outer += 1;
				}
				
				if pressure > max_pressure {
					max_pressure = pressure;
				}
			}
		
		
			let popped_path_step = elephant_paths.pop().unwrap();
			if popped_path_step.step == 1 {
				new_open_valves.pop();
			}
			if !elephant_paths.is_empty() {
				for p in elephant_paths.iter().rev() {
					if 1 != p.step {
						elephant_current_valve = p.step;
						break;
					}
				}
			}
		} else {
			let elephant_option = valve_options
				.get(&elephant_current_valve).unwrap()[elephant_path_step.next_option_index];
			elephant_path_step.next_option_index += 1;

			let mut elephant_new_last_open_valve = elephant_path_step.last_open_valve;
			let mut elephant_new_last_open_valve_index = elephant_path_step.last_open_valve_index;
			if elephant_option == 1 {
				if elephant_path_step.step != 1 
				&& !new_open_valves.contains(&elephant_current_valve) 
				&& !open_valves.contains(&elephant_current_valve) 
				&& elephant_current_valve_struct.flow_rate != 0
				{				
					elephant_new_last_open_valve = elephant_current_valve;
					elephant_new_last_open_valve_index = elephant_len - 1;
					new_open_valves.push(elephant_current_valve);
				} else {
					//do not open if not possible, do not add step
					continue;
				}
			} else {
				//should check for optimal path
				let left_part = if elephant_path_step.last_open_valve == 0 {
					AA
				} else {
					elephant_path_step.last_open_valve
				};
				
				let elephant_small_path_len = if elephant_path_step.last_open_valve == 0 {
					elephant_len
				} else {
					elephant_len - elephant_path_step.last_open_valve_index - 1
				};
				
				let elephant_shortest_path = *shortest_paths.get(&[left_part, elephant_option]).unwrap();
				if elephant_small_path_len as u32 > elephant_shortest_path {
					//do not go to option, not optimal way
					continue;
				}
				
				elephant_current_valve = elephant_option;
			}
			
			let elephant_new_path_step = PathStep{ 
				step: elephant_option, 
				next_option_index: 0,
				last_open_valve: elephant_new_last_open_valve,
				last_open_valve_index: elephant_new_last_open_valve_index,
			};
			elephant_paths.push(elephant_new_path_step);
		}
	}
	max_pressure
}

fn find_shortest_path(left_valve: u32, right_valve: u32, valves: &HashMap<u32, Valve>) -> u32 {
	let mut current_valves: HashSet<u32> = HashSet::new();
	let mut visited_valves: HashSet<u32> = HashSet::new();
	current_valves.insert(left_valve);
	let mut current_index = 1;
	let mut result = valves.len();
	loop {
		visited_valves.extend(current_valves.clone());
		let mut new_current_valves = HashSet::new();
		for valve in current_valves {
			new_current_valves.extend(valves.get(&valve).unwrap().next_valves.clone());
		}
		new_current_valves = &new_current_valves - &visited_valves;
		if new_current_valves.contains(&right_valve) {
			result = current_index;
			break;
		} else if new_current_valves.is_empty() {
			break;
		} else {
			current_valves = new_current_valves;
		}
		current_index += 1;
	}
	result as u32
}

fn process(steps: u32, valves: &HashMap<u32, Valve>, valve_options: &HashMap<u32, Vec<u32>>) -> u32 {
	let mut shortest_paths: HashMap<[u32; 2], u32> = HashMap::new();
	for (name1, left_valve) in valves {
		if left_valve.flow_rate == 0 && *name1 != AA {
			continue;
		}
		for name2 in valves.keys() {
			let min_path_len = if name1 == name2 {
				0
			} else {
				find_shortest_path(*name1, *name2, valves)
			};
			shortest_paths.insert([*name1, *name2], min_path_len);
		}
	}
	println!("shortest paths found, {:?}", shortest_paths);

	process_inner(
		&vec![],
		valves,
		steps,
		&shortest_paths,
		true,
		&[],
		valve_options,
	)
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut valves: HashMap<u32, Valve> = HashMap::new();
    
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let input: Vec<&str> = user_input.trim().split(' ').collect();
            let name = input[1];
            let flow_rate: u32 = input[4].split('=').collect::<Vec<&str>>()[1].replace(';', "").parse().unwrap();
            let mut next_valves: HashSet<u32> = HashSet::new();
            for item in input.iter().skip(9) {
				let item = item.replace(',', "");
				let item_number = item.chars().next().unwrap() as u32 * 100 + item.chars().nth(1).unwrap() as u32;
				next_valves.insert(item_number);
			}
			let name_number = name.chars().next().unwrap() as u32 * 100 + name.chars().nth(1).unwrap() as u32;
            valves.insert(name_number, Valve{ name: name_number, flow_rate, next_valves });
            user_input.clear();
        }
	}
		
	let mut valve_options: HashMap<u32, Vec<u32>> = HashMap::new();
	for (name, valve) in &valves {
		let mut options: Vec<u32> = vec![1];
		for v in &valve.next_valves {
			options.push(*v);
		}
		valve_options.insert(*name, options);
	}
		
	let max_pressure_part_2 = process(26, &valves, &valve_options);
	println!("max_pressure part 2: {}", max_pressure_part_2);
}
