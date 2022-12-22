use std::io;

#[derive(Debug, Clone)]
enum Terrain {
    Outside,
    Empty,
    Wall,
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn make_move(coordinate: &[usize; 2], direction: &Direction, field: &[Vec<Terrain>]) -> [usize; 2] {
    let possible_coordinate = match direction {
        Direction::Left => [coordinate[0] - 1, coordinate[1]],
        Direction::Up => [coordinate[0], coordinate[1] - 1],
        Direction::Right => [coordinate[0] + 1, coordinate[1]],
        Direction::Down => [coordinate[0], coordinate[1] + 1],
    };
    match field[possible_coordinate[1]][possible_coordinate[0]] {
        Terrain::Empty => possible_coordinate,
        Terrain::Wall => *coordinate,
        Terrain::Outside => {
            let possible_teleported_coordinate = match direction {
                Direction::Left => [
                    field[coordinate[1]].len()
                        - 1
                        - field[coordinate[1]]
                            .iter()
                            .rev()
                            .position(|v| !matches!(v, Terrain::Outside))
                            .unwrap(),
                    coordinate[1],
                ],
                Direction::Up => [
                    coordinate[0],
                    field.len()
                        - 1
                        - field
                            .iter()
                            .rev()
                            .position(|v| !matches!(v[coordinate[0]], Terrain::Outside))
                            .unwrap(),
                ],
                Direction::Right => [
                    field[coordinate[1]]
                        .iter()
                        .position(|v| !matches!(v, Terrain::Outside))
                        .unwrap(),
                    coordinate[1],
                ],
                Direction::Down => [
                    coordinate[0],
                    field
                        .iter()
                        .position(|v| !matches!(v[coordinate[0]], Terrain::Outside))
                        .unwrap(),
                ],
            };
            match field[possible_teleported_coordinate[1]][possible_teleported_coordinate[0]] {
                Terrain::Empty => possible_teleported_coordinate,
                Terrain::Wall => *coordinate,
                Terrain::Outside => *coordinate, //should not happen
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
                    coordinate = make_move(&coordinate, &direction, &field);
                }
            }
        }
    }

    println!(
        "result part 1: {}",
        1000 * coordinate[1] + 4 * coordinate[0] + score(direction)
    );
}
