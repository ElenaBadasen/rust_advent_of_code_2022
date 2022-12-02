use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    
    let mut score = 0;
    let mut score_part_2 = 0;
    
    let single_scores = HashMap::from([
		('X', 1),
		('Y', 2),
		('Z', 3),
	]);
	
	//part 1 table
	let equal_moves = vec![
		('X', 'A'),
		('Y', 'B'),
		('Z', 'C'),
	];
	
	//part 1 table
	let winning_moves = vec![
		('X', 'C'),
		('Y', 'A'),
		('Z', 'B'),
	];
	
	//part 2 table
	let losing_moves_cost_part_2 = HashMap::from([
		('A', 3),
		('B', 1),
		('C', 2),
	]);
	
	//part 2 table
	let draw_moves_cost_part_2 = HashMap::from([
		('A', 1),
		('B', 2),
		('C', 3),
	]);
	
	//part 2 table
	let winning_moves_cost_part_2 = HashMap::from([
		('A', 2),
		('B', 3),
		('C', 1),
	]);
	
	
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
		if bytes == 0 {
			break;	
		} else {
			let mut chars = user_input.trim().chars();
			let opponent_move = chars.next().unwrap();
			chars.next();
			let my_move = chars.next().unwrap();
			
			//part 1
			score += single_scores.get(&my_move).unwrap();
			if equal_moves.contains(&(my_move, opponent_move)) {
				score += 3;
			} else if winning_moves.contains(&(my_move, opponent_move)) {
				score += 6;
			}
			
			//part 2
			match my_move {
				'X' => {
					score_part_2 += losing_moves_cost_part_2.get(&opponent_move).unwrap();
				},
				'Y' => {
					score_part_2 += 3;
					score_part_2 += draw_moves_cost_part_2.get(&opponent_move).unwrap();
				},
				'Z' => {
					score_part_2 += 6;
					score_part_2 += winning_moves_cost_part_2.get(&opponent_move).unwrap();
				},
				_other => {},
			}
		}
		user_input = String::new();
	}
    
    print!("result part 1: {}\n", score);
    print!("result part 2: {}\n", score_part_2);
}
