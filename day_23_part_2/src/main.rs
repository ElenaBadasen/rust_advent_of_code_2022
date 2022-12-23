use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut field: HashMap<[i32; 2], bool> = HashMap::new();
    let mut elves: HashSet<[i32; 2]> = HashSet::new();

    let mut current_index = 0;
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let trimmed_input = user_input.trim();
            for (inner_index, c) in trimmed_input.chars().enumerate() {
                if c == '#' {
                    field.insert([inner_index as i32, current_index], true);
                    elves.insert([inner_index as i32, current_index]);
                } else {
                    field.insert([inner_index as i32, current_index], false);
                }
            }
            current_index += 1;
            user_input.clear();
        }
    }

    let mut directions_order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut proposed_moves: HashMap<[i32; 2], HashSet<[i32; 2]>> = HashMap::new();
    let mut new_elves: HashSet<[i32; 2]> = HashSet::new();

    let mut result_part_2 = 0;
    for i in 0.. {
        for elf in &elves {
            let mut move_proposed = false;
            if *field.entry([elf[0] - 1, elf[1] - 1]).or_insert(false)
                || *field.entry([elf[0], elf[1] - 1]).or_insert(false)
                || *field.entry([elf[0] + 1, elf[1] - 1]).or_insert(false)
                || *field.entry([elf[0] - 1, elf[1] + 1]).or_insert(false)
                || *field.entry([elf[0], elf[1] + 1]).or_insert(false)
                || *field.entry([elf[0] + 1, elf[1] + 1]).or_insert(false)
                || *field.entry([elf[0] - 1, elf[1]]).or_insert(false)
                || *field.entry([elf[0] + 1, elf[1]]).or_insert(false)
            {
                for direction in &directions_order {
                    match direction {
                        Direction::North => {
                            if !*field.entry([elf[0] - 1, elf[1] - 1]).or_insert(false)
                                && !*field.entry([elf[0], elf[1] - 1]).or_insert(false)
                                && !*field.entry([elf[0] + 1, elf[1] - 1]).or_insert(false)
                            {
                                proposed_moves
                                    .entry([elf[0], elf[1] - 1])
                                    .or_default()
                                    .insert(*elf);
                                move_proposed = true;
                                break;
                            }
                        }
                        Direction::South => {
                            if !*field.entry([elf[0] - 1, elf[1] + 1]).or_insert(false)
                                && !*field.entry([elf[0], elf[1] + 1]).or_insert(false)
                                && !*field.entry([elf[0] + 1, elf[1] + 1]).or_insert(false)
                            {
                                proposed_moves
                                    .entry([elf[0], elf[1] + 1])
                                    .or_default()
                                    .insert(*elf);
                                move_proposed = true;
                                break;
                            }
                        }
                        Direction::West => {
                            if !*field.entry([elf[0] - 1, elf[1] - 1]).or_insert(false)
                                && !*field.entry([elf[0] - 1, elf[1]]).or_insert(false)
                                && !*field.entry([elf[0] - 1, elf[1] + 1]).or_insert(false)
                            {
                                proposed_moves
                                    .entry([elf[0] - 1, elf[1]])
                                    .or_default()
                                    .insert(*elf);
                                move_proposed = true;
                                break;
                            }
                        }
                        Direction::East => {
                            if !*field.entry([elf[0] + 1, elf[1] - 1]).or_insert(false)
                                && !*field.entry([elf[0] + 1, elf[1]]).or_insert(false)
                                && !*field.entry([elf[0] + 1, elf[1] + 1]).or_insert(false)
                            {
                                proposed_moves
                                    .entry([elf[0] + 1, elf[1]])
                                    .or_default()
                                    .insert(*elf);
                                move_proposed = true;
                                break;
                            }
                        }
                    }
                }
            }
            if !move_proposed {
                new_elves.insert(*elf);
            }
        }

        let mut any_elves_moved = false;
        for (m, moving_elves) in &proposed_moves {
            if moving_elves.len() == 1 {
                new_elves.insert(*m);
                *field.get_mut(moving_elves.iter().next().unwrap()).unwrap() = false;
                *field.get_mut(m).unwrap() = true;
                if !any_elves_moved {
                    any_elves_moved = true;
                }
            } else {
                for el in moving_elves {
                    new_elves.insert(*el);
                }
            }
        }
        if !any_elves_moved {
            result_part_2 = i + 1;
            break;
        }
        elves = new_elves.clone();
        new_elves.clear();
        proposed_moves.clear();
        let elem = directions_order.remove(0);
        directions_order.push(elem);
    }

    println!("result part 2: {}", result_part_2);
}
