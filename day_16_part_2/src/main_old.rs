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
	step: [String; 2],
	no_options: [bool; 2],
	open_valves: HashSet<String>,
	visited_options: HashSet<[String; 2]>,
	last_open_valve: [String; 2],
	last_open_valve_index: [usize; 2],
}

fn pressure(paths: &[PathStep], valves: &HashMap<String, Valve>, steps: u32) -> u32 {
	let mut pressure = 0;
	let mut current_valve_names = vec!["AA".to_string(); 2];
	let mut open_valves_flows_sum = 0;
	for i in 1..(steps + 1) {
		pressure += open_valves_flows_sum;
		match paths.get(i as usize) {
			None => {},
			Some(s) => {
				for (j, s_inner) in s.step.iter().enumerate() {
					if s_inner == "open" {
						open_valves_flows_sum += valves.get(&current_valve_names[j]).unwrap().flow_rate;
					} else if s_inner != "" {
						current_valve_names[j] = s_inner.clone();
					}
				}
			},
		}
	}
	pressure
}

fn process(players: usize, steps: u32, valves: &HashMap<String, Valve>) -> u32 {
	let mut paths: Vec<PathStep> = vec![PathStep{
		step: ["AA".to_string(), "AA".to_string()], 
		open_valves: HashSet::new(), 
		visited_options: HashSet::new(),
		last_open_valve: ["".to_string(), "".to_string()],
		last_open_valve_index: [0; 2],
	}];
    let mut current_valves = ["AA".to_string(),"AA".to_string()];
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
			let start_len = if name1 == name2 {
				0
			} else {
				valves_len as u32
			};
			shortest_paths.insert(name1.clone() + &name2, start_len);
		}
	}
    
    let mut processed_paths = 0;
    loop {
		if processed_paths % 100000 == 0 {
			println!("processed_paths {}", processed_paths);
		}
		if paths.is_empty() {
			break;
		}
		let len = paths.len();
		if len > steps as usize || paths.last().unwrap().is_finished = [true, true] {
			//we are at the end of a path, time is out; or we ran out of options for this valve
			let pressure = pressure(&paths, &valves, steps);
			if pressure > max_pressure {
				max_pressure = pressure;
			}
			processed_paths += 1;
			paths.pop();
			let step = paths.last().unwrap().step.clone();
			
			for j in 0..step.len() {
				for p in paths.iter().rev() {
					if !(["".to_string(), "open".to_string()].contains(&p.step[j])) {
						current_valves[j] = p.step[j].clone();
						break;
					}
				}
			}
			continue;
		}
		
		//total non-zero valves count is 15, so we can stop going now
		if paths.last().unwrap().open_valves.len() == 15 {
			let pressure = pressure(&paths, &valves, steps);
			if pressure > max_pressure {
				max_pressure = pressure;
			}
			processed_paths += 1;
			paths.pop();
			let step = paths.last().unwrap().step.clone();
			
			for j in 0..step.len() {
				for p in paths.iter().rev() {
					if !(["".to_string(), "open".to_string()].contains(&p.step[j])) {
						current_valves[j] = p.step[j].clone();
						break;
					}
				}
			}
			continue;
		}
	
		let path_step = paths.get_mut(len - 1).unwrap();
		let mut current_valve_structs = vec![];
		for j in 0..players {
			current_valve_structs.push(valves.get(&current_valves[j]).unwrap());
		}
		let mut options: HashSet<[String; 2]> = HashSet::new();
		
		if players == 2 {
			for v in &current_valve_structs[0].next_valves {
				for w in &current_valve_structs[1].next_valves {
					options.insert([v.clone(), w.clone()]);
				}
				//options.insert([v.clone(), "".to_string()]);
			}
			//for w in &current_valve_structs[1].next_valves {
				//options.insert(["".to_string(), w.clone()]);
			//}
			if path_step.is_finished[0] {
				if path_step.is_finished[1] {
					//finish this path
					continue;
				} else {
					for w in &current_valve_structs[1].next_valves {
						options.insert(["".to_string(), w.clone()]);
					}
				}
			} else if path_step.is_finished[1] {
				for v in &current_valve_structs[0].next_valves {
					options.insert([v.clone(), "".to_string()]);
				}
			}
		} else {
			println!("NOT AN OPTION");
			return 0;
		}
		
		for j in 0..players {
			if path_step.step[j] != "open" 
			&& !path_step.open_valves.contains(&current_valves[j]) 
			&& current_valve_structs[j].flow_rate != 0
			{				
				if players == 2 {
					if j == 0 {
						if path_step.step[1] != "open" 
						&& !path_step.open_valves.contains(&current_valves[1]) 
						&& current_valve_structs[1].flow_rate != 0
						{
							options.insert(["open".to_string(), "open".to_string()]);
						}
						for w in &current_valve_structs[1].next_valves {
							options.insert(["open".to_string(), w.clone()]);
						}
					} else { 
						for v in &current_valve_structs[0].next_valves {
							options.insert([v.clone(), "open".to_string()]);
						}
					}
				} else {
					println!("NOT AN OPTION");
					return 0;
				}
			}
		}
		options = &options - &path_step.visited_options;
		if options.is_empty() {
			//we are at the end of a path, no more ways to go
			let pressure = pressure(&paths, &valves, steps);
			if pressure > max_pressure {
				max_pressure = pressure;
			}
			processed_paths += 1;
			paths.pop();
			if !paths.is_empty() {
				for j in 0..players {
					for p in paths.iter().rev() {
						if !(["".to_string(), "open".to_string()].contains(&p.step[j])) {
							current_valves[j] = p.step[j].clone();
							break;
						}
					}
				}
			}
		} else {
			let option = options.iter().next().unwrap();
			path_step.visited_options.insert(option.clone());
			let mut new_open_valves = path_step.open_valves.clone();
			let mut new_last_open_valve = path_step.last_open_valve.clone();
			let mut new_last_open_valve_index = path_step.last_open_valve_index.clone();
			for j in 0..players {
				if option[j] == "open" {
					new_open_valves.insert(current_valves[j].clone());
					new_last_open_valve[j] = current_valves[j].clone();
					new_last_open_valve_index[j] = len - 1;
				} else if option[j] == "" {
					//do nothing, this player doesn't move
				} else {
					//should check for optimal path
					let left_part = if path_step.last_open_valve[j] == "" {
						"AA".to_string()
					} else {
						path_step.last_open_valve[j].clone()
					};
					
					let small_path_len = if path_step.last_open_valve[j] == "" {
						len
					} else {
						len - path_step.last_open_valve_index[j]
					};
					
					let shortest_path = *shortest_paths.get(&(left_part.clone() + &option[j])).unwrap();
					if small_path_len as u32 > shortest_path {
						//do not go to option, not optimal way
						continue;
					} else if (small_path_len as u32) < shortest_path {
						shortest_paths.insert(left_part + &option[j], small_path_len as u32);
					}
					
					current_valves[j] = option[j].clone();
				}
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
    
    //let max_pressure = process(1, 30, &valves);
	//println!("max_pressure: {}", max_pressure);
	
	let max_pressure_part_2 = process(2, 26, &valves);
	println!("max_pressure part 2: {}", max_pressure_part_2);
	
}
