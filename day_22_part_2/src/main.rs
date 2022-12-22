use std::io;

#[derive(Debug, Clone)]
enum Terrain {
    Outside,
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

//layout for my input specifically
fn teleported_coordinate(
    coordinate: &[usize; 2],
    direction: &Direction,
) -> ([usize; 2], Direction) {
    match coordinate[1] {
        0..=50 => {
            match coordinate[0] {
                51..=100 => {
                    match direction {
                        Direction::Left => ([1, 151 - coordinate[1]], Direction::Right),
                        Direction::Up => ([1, 150 + (coordinate[0] - 50)], Direction::Right),
                        Direction::Right => (*coordinate, (*direction).clone()), //should not happen
                        Direction::Down => (*coordinate, (*direction).clone()),  //should not happen
                    }
                }
                101..=150 => {
                    match direction {
                        Direction::Left => (*coordinate, (*direction).clone()), //should not happen
                        Direction::Up => ([coordinate[0] - 100, 200], Direction::Up),
                        Direction::Right => ([100, 151 - coordinate[1]], Direction::Left),
                        Direction::Down => ([100, 50 + (coordinate[0] - 100)], Direction::Left),
                    }
                }
                _ => (*coordinate, (*direction).clone()),
            }
        }
        51..=100 => {
            match direction {
                Direction::Left => ([coordinate[1] - 50, 101], Direction::Down),
                Direction::Up => (*coordinate, (*direction).clone()), //should not happen
                Direction::Right => ([100 + (coordinate[1] - 50), 50], Direction::Up),
                Direction::Down => (*coordinate, (*direction).clone()), //should not happen
            }
        }
        101..=150 => {
            match coordinate[0] {
                0..=50 => {
                    match direction {
                        Direction::Left => ([51, 51 - (coordinate[1] - 100)], Direction::Right),
                        Direction::Up => ([51, 50 + coordinate[0]], Direction::Right),
                        Direction::Right => (*coordinate, (*direction).clone()), //should not happen
                        Direction::Down => (*coordinate, (*direction).clone()),  //should not happen
                    }
                }
                51..=100 => {
                    match direction {
                        Direction::Left => (*coordinate, (*direction).clone()), //should not happen
                        Direction::Up => (*coordinate, (*direction).clone()),   //should not happen
                        Direction::Right => ([150, 51 - (coordinate[1] - 100)], Direction::Left),
                        Direction::Down => ([50, 150 + (coordinate[0] - 50)], Direction::Left),
                    }
                }
                _ => (*coordinate, (*direction).clone()),
            }
        }
        151..=200 => {
            match direction {
                Direction::Left => ([50 + (coordinate[1] - 150), 1], Direction::Down),
                Direction::Up => (*coordinate, (*direction).clone()), //should not happen
                Direction::Right => ([50 + (coordinate[1] - 150), 150], Direction::Up),
                Direction::Down => ([100 + coordinate[0], 1], Direction::Down),
            }
        }
        _ => (*coordinate, (*direction).clone()),
    }
}

fn make_move(
    coordinate: &[usize; 2],
    direction: &Direction,
    field: &[Vec<Terrain>],
) -> ([usize; 2], Direction) {
    let possible_coordinate = match direction {
        Direction::Left => [coordinate[0] - 1, coordinate[1]],
        Direction::Up => [coordinate[0], coordinate[1] - 1],
        Direction::Right => [coordinate[0] + 1, coordinate[1]],
        Direction::Down => [coordinate[0], coordinate[1] + 1],
    };
    match field[possible_coordinate[1]][possible_coordinate[0]] {
        Terrain::Empty => (possible_coordinate, (*direction).clone()),
        Terrain::Wall => (*coordinate, (*direction).clone()),
        Terrain::Outside => {
            let (possible_teleported_coordinate, possible_direction) =
                teleported_coordinate(coordinate, direction);
            match field[possible_teleported_coordinate[1]][possible_teleported_coordinate[0]] {
                Terrain::Empty => (possible_teleported_coordinate, possible_direction),
                Terrain::Wall => (*coordinate, (*direction).clone()),
                Terrain::Outside => (*coordinate, (*direction).clone()), //should not happen
            }
        }
    }
}

fn r(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

fn l(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
    }
}

fn score(direction: Direction) -> usize {
    match direction {
        Direction::Left => 2,
        Direction::Up => 3,
        Direction::Right => 0,
        Direction::Down => 1,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut field: Vec<Vec<Terrain>> = vec![];
    let mut path: Vec<String> = vec![];
    let mut is_first = true;
    let mut width = 0;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            if user_input != "\n" {
                if [' ', '.', '#'].contains(&user_input.chars().next().unwrap()) {
                    let mut row: Vec<Terrain> = vec![];
                    for c in user_input.chars() {
                        match c {
                            ' ' => row.push(Terrain::Outside),
                            '.' => row.push(Terrain::Empty),
                            '#' => row.push(Terrain::Wall),
                            _default => {}
                        }
                    }
                    row.insert(0, Terrain::Outside);
                    row.push(Terrain::Outside);
                    if is_first {
                        width = row.len();
                        is_first = false;
                    } else {
                        while row.len() < width {
                            row.push(Terrain::Outside);
                        }
                    }
                    field.push(row);
                } else {
                    let mut current_number_string = "".to_string();
                    for c in user_input.trim().chars() {
                        if ['L', 'R'].contains(&c) {
                            if !current_number_string.is_empty() {
                                path.push(current_number_string.clone());
                                current_number_string.clear();
                            }
                            path.push(c.to_string());
                        } else {
                            current_number_string.push(c);
                        }
                    }
                    if !current_number_string.is_empty() {
                        path.push(current_number_string.clone());
                    }
                }
            }
            user_input.clear();
        }
    }

    field.insert(0, vec![Terrain::Outside; field[0].len()]);
    field.push(vec![Terrain::Outside; field[0].len()]);

    let mut coordinate: [usize; 2] = [
        field[1]
            .iter()
            .position(|v| matches!(*v, Terrain::Empty))
            .unwrap(),
        1,
    ];
    let mut direction = Direction::Right;
    for c in &path {
        match c.as_str() {
            "R" => direction = r(direction),
            "L" => direction = l(direction),
            _default => {
                let cycles: u32 = c.parse().unwrap();
                for _i in 0..cycles {
                    let (new_coordinate, new_direction) =
                        make_move(&coordinate, &direction, &field);
                    coordinate = new_coordinate;
                    direction = new_direction;
                }
            }
        }
    }

    println!(
        "result part 2: {}",
        1000 * coordinate[1] + 4 * coordinate[0] + score(direction)
    );
}
