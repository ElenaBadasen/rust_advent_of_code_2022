use std::io;

fn get_new_max_elves_loads(max_elves_loads: &Vec<u32>, elf_load_to_insert: u32) -> Vec<u32> {
	let mut new_max_elves_loads = vec![];
	let mut current_elf_load_found_its_place = false;
	for value in max_elves_loads {
		if !current_elf_load_found_its_place && elf_load_to_insert > *value {
			new_max_elves_loads.push(elf_load_to_insert);
			current_elf_load_found_its_place = true;
		}
		if new_max_elves_loads.len() < max_elves_loads.len() {
			new_max_elves_loads.push(*value);
		}
	}
	new_max_elves_loads
}

fn main() {   
	let max_load_elves_to_find = 3;
	
    let stdin = io::stdin();
    let mut user_input = String::new();
    
    let mut current_elf_load = 0;
    let mut max_elves_loads = vec![0; max_load_elves_to_find];
       
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
		if bytes == 0 {
			break;
		} else if user_input == "\n" {
			if current_elf_load > *max_elves_loads.last().unwrap() {
				max_elves_loads = get_new_max_elves_loads(&max_elves_loads, current_elf_load);
			}
			current_elf_load = 0;
		} else {
			let value: u32 = user_input.trim().parse().expect("Wrong input!");
			current_elf_load += value;
		}
		user_input = String::new();
	}
	if current_elf_load > *max_elves_loads.last().unwrap() {
		max_elves_loads = get_new_max_elves_loads(&max_elves_loads, current_elf_load);
	}
	
	println!("result part 1: {}", *max_elves_loads.first().unwrap());
	println!("result part 2: {}", max_elves_loads.iter().sum::<u32>());
}
