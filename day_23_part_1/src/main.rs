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
    let mut left_border: i32 = 0;
    let mut right_border: i32 = 0;
    let mut top_border: i32 = 0;
    let mut bottom_border: i32;

    let mut current_index = 0;
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let trimmed_input = user_input.trim();
            if right_border == 0 {
                right_border = trimmed_input.len() as i32 - 1;
            }
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
    bottom_border = current_index - 1;

    let mut directions_order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut proposed_moves: HashMap<[i32; 2], HashSet<[i32; 2]>> = HashMap::new();
    let mut new_elves: HashSet<[i32; 2]> = HashSet::new();

    for _i in 0..10 {
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

        for (m, moving_elves) in &proposed_moves {
            if moving_elves.len() == 1 {
                new_elves.insert(*m);
                *field.get_mut(moving_elves.iter().next().unwrap()).unwrap() = false;
                *field.get_mut(m).unwrap() = true;

                if m[0] > right_border {
                    right_border = m[0];
                }
                if m[0] < left_border {
                    left_border = m[0];
                }
                if m[1] < top_border {
                    top_border = m[1];
                }
                if m[1] > bottom_border {
                    bottom_border = m[1];
                }
            } else {
                for el in moving_elves {
                    new_elves.insert(*el);
                }
            }
        }
        elves = new_elves.clone();
        new_elves.clear();
        proposed_moves.clear();
        let elem = directions_order.remove(0);
        directions_order.push(elem);
    }

    let result =
        (right_border + 1 - left_border) * (bottom_border + 1 - top_border) - elves.len() as i32;
    println!("result part 1: {}", result);
}
