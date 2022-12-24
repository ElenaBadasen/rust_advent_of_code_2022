use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Blizzard {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Terrain {
    Snow(BlizzardSet),
    Wall,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BlizzardSet {
    set: Vec<Blizzard>,
}

fn refresh_field(
    field: HashMap<[i32; 2], Terrain>,
    field_width: i32,
    field_height: i32,
) -> HashMap<[i32; 2], Terrain> {
    let mut new_field: HashMap<[i32; 2], Terrain> = HashMap::new();
    for (coord, elem) in &field {
        match elem {
            Terrain::Empty => {}
            Terrain::Wall => {
                new_field.insert(*coord, Terrain::Wall);
            }
            Terrain::Snow(blizzard_set) => {
                for blizzard in &blizzard_set.set {
                    let actual_coordinate: [i32; 2] = match blizzard {
                        Blizzard::Left => {
                            let possible_coordinate = [coord[0] - 1, coord[1]];
                            if !matches!(field.get(&possible_coordinate), Some(Terrain::Wall)) {
                                possible_coordinate
                            } else {
                                [field_width - 2, coord[1]]
                            }
                        }
                        Blizzard::Up => {
                            let possible_coordinate = [coord[0], coord[1] - 1];
                            if !matches!(field.get(&possible_coordinate), Some(Terrain::Wall)) {
                                possible_coordinate
                            } else {
                                [coord[0], field_height - 2]
                            }
                        }
                        Blizzard::Right => {
                            let possible_coordinate = [coord[0] + 1, coord[1]];
                            if !matches!(field.get(&possible_coordinate), Some(Terrain::Wall)) {
                                possible_coordinate
                            } else {
                                [1, coord[1]]
                            }
                        }
                        Blizzard::Down => {
                            let possible_coordinate = [coord[0], coord[1] + 1];
                            if !matches!(field.get(&possible_coordinate), Some(Terrain::Wall)) {
                                possible_coordinate
                            } else {
                                [coord[0], 1]
                            }
                        }
                    };
                    let entry = new_field.entry(actual_coordinate).or_insert(Terrain::Empty);
                    match entry {
                        Terrain::Snow(inner_blizzard_set) => {
                            inner_blizzard_set.set.push((*blizzard).clone());
                        }
                        _ => {
                            *entry = Terrain::Snow(BlizzardSet {
                                set: vec![(*blizzard).clone()],
                            });
                        }
                    }
                }
            }
        }
    }
    new_field
}

fn run(
    start_coordinate: [i32; 2],
    end_coordinate: [i32; 2],
    input_field: &HashMap<[i32; 2], Terrain>,
    field_width: i32,
    field_height: i32,
) -> (i32, HashMap<[i32; 2], Terrain>) {
    let mut current_coordinates: HashSet<[i32; 2]> = HashSet::new();
    current_coordinates.insert(start_coordinate);
    let mut result = 0;
    let mut field = input_field.clone();

    for i in 1.. {
        if result > 0 {
            break;
        }
        field = refresh_field(field, field_width, field_height);

        let mut new_current_coordinates: HashSet<[i32; 2]> = HashSet::new();
        for coordinate in current_coordinates {
            let candidate_coordinates = [
                coordinate,
                [coordinate[0], coordinate[1] - 1],
                [coordinate[0], coordinate[1] + 1],
                [coordinate[0] - 1, coordinate[1]],
                [coordinate[0] + 1, coordinate[1]],
            ];
            for candidate in candidate_coordinates {
                if candidate == end_coordinate {
                    result = i;
                    break;
                }
                if matches!(
                    *field.entry(candidate).or_insert(Terrain::Empty),
                    Terrain::Empty
                ) {
                    new_current_coordinates.insert(candidate);
                }
            }
        }
        current_coordinates = new_current_coordinates.clone();
    }
    (result, field)
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut field: HashMap<[i32; 2], Terrain> = HashMap::new();

    let mut current_index = 0;
    let mut field_width: i32 = 0;
    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            if current_index == 0 {
                field_width = user_input.trim().len() as i32;
            }
            for (inner_index, c) in user_input.trim().chars().enumerate() {
                let mut terrain = Terrain::Empty;
                match c {
                    '#' => {
                        terrain = Terrain::Wall;
                    }
                    '.' => terrain = Terrain::Empty,
                    '<' => {
                        terrain = Terrain::Snow(BlizzardSet {
                            set: vec![Blizzard::Left],
                        })
                    }
                    '^' => {
                        terrain = Terrain::Snow(BlizzardSet {
                            set: vec![Blizzard::Up],
                        })
                    }
                    '>' => {
                        terrain = Terrain::Snow(BlizzardSet {
                            set: vec![Blizzard::Right],
                        })
                    }
                    'v' => {
                        terrain = Terrain::Snow(BlizzardSet {
                            set: vec![Blizzard::Down],
                        })
                    }
                    _ => {}
                }
                field.insert([inner_index as i32, current_index], terrain);
            }
            user_input.clear();
            current_index += 1;
        }
    }
    let field_height: i32 = current_index;
    field.insert([1, -1], Terrain::Wall);
    field.insert([field_width - 2, field_height], Terrain::Wall);

    let (steps_1, field_1) = run(
        [1, 0],
        [field_width - 2, field_height - 1],
        &field,
        field_width,
        field_height,
    );
    let (steps_2, field_2) = run(
        [field_width - 2, field_height - 1],
        [1, 0],
        &field_1,
        field_width,
        field_height,
    );
    let (steps_3, _field_3) = run(
        [1, 0],
        [field_width - 2, field_height - 1],
        &field_2,
        field_width,
        field_height,
    );

    println!("result part 2: {}", steps_1 + steps_2 + steps_3);
}
