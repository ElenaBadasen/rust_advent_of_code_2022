use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i16,
    y: i64,
}

#[derive(Clone, Debug, PartialEq)]
enum RockType {
    Horizontal,
    Cross,
    L,
    Vertical,
    Square,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Terrain {
    Air,
    Rock,
}

fn clashes(tile: &Coordinate, field: &VecDeque<[Terrain; 7]>, offset: i64) -> bool {
    if tile.x < 0 || tile.x >= 7 || tile.y < 0 {
        return true;
    }
    match field[tile.y as usize - offset as usize][tile.x as usize] {
        Terrain::Air => false,
        Terrain::Rock => true,
    }
}

fn move_rock(
    start_coordinate: &Coordinate,
    rock_type: &RockType,
    direction: &Direction,
    field: &VecDeque<[Terrain; 7]>,
    offset: i64,
) -> (Coordinate, bool) {
    let potential_coordinate: Coordinate = match direction {
        Direction::Left => Coordinate {
            x: start_coordinate.x - 1,
            y: start_coordinate.y,
        },
        Direction::Right => Coordinate {
            x: start_coordinate.x + 1,
            y: start_coordinate.y,
        },
        Direction::Down => {
            if start_coordinate.y == 0 {
                return (
                    Coordinate {
                        x: start_coordinate.x,
                        y: start_coordinate.y,
                    },
                    true,
                );
            }
            Coordinate {
                x: start_coordinate.x,
                y: start_coordinate.y - 1,
            }
        }
    };

    for tile in rock_tiles(rock_type, &potential_coordinate) {
        if clashes(&tile, field, offset) {
            let result = matches!(direction, Direction::Down);
            return (
                Coordinate {
                    x: start_coordinate.x,
                    y: start_coordinate.y,
                },
                result,
            );
        }
    }
    (
        Coordinate {
            x: potential_coordinate.x as i16,
            y: potential_coordinate.y as i64,
        },
        false,
    )
}

fn rock_tiles(rock_type: &RockType, coordinate: &Coordinate) -> HashSet<Coordinate> {
    match rock_type {
        RockType::Horizontal => vec![
            Coordinate {
                x: coordinate.x,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x + 2,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x + 3,
                y: coordinate.y,
            },
        ]
        .into_iter()
        .collect::<HashSet<Coordinate>>(),
        RockType::Cross => vec![
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x + 2,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y - 2,
            },
        ]
        .into_iter()
        .collect::<HashSet<Coordinate>>(),
        RockType::L => vec![
            Coordinate {
                x: coordinate.x + 2,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x + 2,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x + 2,
                y: coordinate.y - 2,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 2,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y - 2,
            },
        ]
        .into_iter()
        .collect::<HashSet<Coordinate>>(),
        RockType::Vertical => vec![
            Coordinate {
                x: coordinate.x,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 2,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 3,
            },
        ]
        .into_iter()
        .collect::<HashSet<Coordinate>>(),
        RockType::Square => vec![
            Coordinate {
                x: coordinate.x,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y - 1,
            },
        ]
        .into_iter()
        .collect::<HashSet<Coordinate>>(),
    }
}

fn next_rock_type(rock_type: RockType) -> RockType {
    match rock_type {
        RockType::Horizontal => RockType::Cross,
        RockType::Cross => RockType::L,
        RockType::L => RockType::Vertical,
        RockType::Vertical => RockType::Square,
        RockType::Square => RockType::Horizontal,
    }
}

fn rock_extra_height(rock_type: &RockType) -> u32 {
    match rock_type {
        RockType::Horizontal => 0,
        RockType::Cross => 2,
        RockType::L => 2,
        RockType::Vertical => 3,
        RockType::Square => 1,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();

    let mut field: VecDeque<[Terrain; 7]> = VecDeque::new();
    for _i in 0..4 {
        field.push_back([Terrain::Air; 7]);
    }
    let mut highest_rock_level: i64 = 0;
    let mut rock_coordinate: Coordinate = Coordinate { x: 2, y: 3 };
    let mut rock_rested: bool;
    let mut rock_type = RockType::Horizontal;
    let mut rocks_fallen: i64 = 0;
    let mut end_reached = false;

    let mut offset = 0;

    let rules: Vec<bool> = user_input.trim().chars().map(|u| u == '<').collect();
    let rules_arr: [bool; 10091] = rules.try_into().unwrap();

    let mut is_first_flat_surface = true;
    let mut saved_rule_index: u64 = 0;
    let mut saved_settled_rock_type = RockType::Horizontal;
    let mut saved_highest_rock_level: u64 = 0;
    let mut saved_rocks_fallen: u64 = 0;

    while !end_reached {
        for (rule_index, rule) in rules_arr.iter().enumerate() {
            //try to move sideways
            let direction = if *rule {
                Direction::Left
            } else {
                Direction::Right
            };
            let (new_coordinate, _) =
                move_rock(&rock_coordinate, &rock_type, &direction, &field, offset);
            rock_coordinate = new_coordinate;

            //try to move downwards
            let (new_coordinate, rested) = move_rock(
                &rock_coordinate,
                &rock_type,
                &Direction::Down,
                &field,
                offset,
            );
            rock_coordinate = new_coordinate;
            rock_rested = rested;
            if rock_rested {
                rocks_fallen += 1;

                //refresh field
                let mut max_split_at_candidate = 0;
                for tile in rock_tiles(
                    &rock_type,
                    &Coordinate {
                        x: rock_coordinate.x,
                        y: rock_coordinate.y,
                    },
                ) {
                    if tile.y > highest_rock_level {
                        //refresh highest_rock_level
                        highest_rock_level = tile.y;
                    }
                    let row = field.get_mut(tile.y as usize - offset as usize).unwrap();
                    row[tile.x as usize] = Terrain::Rock;
                    if *row == [Terrain::Rock; 7] && tile.y > max_split_at_candidate {
                        max_split_at_candidate = tile.y;
                    }
                }
                if max_split_at_candidate > 0 {
                    if max_split_at_candidate == highest_rock_level {
                        //we make a suggestion that flat surfaces on top appear at all,
                        //I checked it on my input, and they indeed appear often
                        if is_first_flat_surface {
                            //set up the variables
                            saved_rule_index = rule_index as u64;
                            saved_settled_rock_type = rock_type.clone();
                            saved_highest_rock_level = highest_rock_level as u64;
                            saved_rocks_fallen = rocks_fallen as u64;

                            is_first_flat_surface = false;
                        } else if saved_rule_index == rule_index as u64
                            && saved_settled_rock_type == rock_type
                        {
                            //we make a suggestion that a cycle can be found at reasonable time
                            //because the set of combinations of rule indices, rock types and
                            //surface shapes seems not too big
                            let cycle_height_change: u64 =
                                highest_rock_level as u64 - saved_highest_rock_level;
                            let cycle_rocks_change: u64 = rocks_fallen as u64 - saved_rocks_fallen;
                            let steps_to_skip: u64 =
                                (1000000000000_u64 - rocks_fallen as u64) / cycle_rocks_change;
                            highest_rock_level += cycle_height_change as i64 * steps_to_skip as i64;
                            offset += cycle_height_change as i64 * steps_to_skip as i64;
                            max_split_at_candidate +=
                                cycle_height_change as i64 * steps_to_skip as i64;
                            rocks_fallen += cycle_rocks_change as i64 * steps_to_skip as i64;
                        }
                    }
                    for _ in 0..(max_split_at_candidate as usize - offset as usize) {
                        field.pop_front();
                    }
                    offset = max_split_at_candidate;
                }
                for j in (highest_rock_level as usize + 1)..=(highest_rock_level as usize + 7) {
                    if j - offset as usize > field.len() - 1 {
                        field.push_back([Terrain::Air; 7]);
                    }
                }

                //if rocks_fallen == 2022 {
                if rocks_fallen == 1000000000000 {
                    end_reached = true;
                    break;
                }

                //generate new rock
                rock_type = next_rock_type(rock_type);
                rock_coordinate = Coordinate {
                    x: 2,
                    y: highest_rock_level + 4 + rock_extra_height(&rock_type) as i64,
                };
            }
        }
    }

    let result = highest_rock_level + 1;
    println!("result part 2: {}", result);
}
