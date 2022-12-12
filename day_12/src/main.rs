use std::collections::HashMap;
use std::io;

fn path_len(
    heights: &[Vec<char>],
    start_point: [usize; 2],
    end_point: [usize; 2],
    ends_with_char: bool,
    end_char: char,
) -> usize {
    let mut paths: Vec<Vec<[usize; 2]>> = vec![vec![start_point]];
    let mut winner_path: Vec<[usize; 2]> = vec![];
    let mut result_found = false;
    let mut best_points_paths: HashMap<[usize; 2], Vec<[usize; 2]>> = HashMap::new();

    loop {
        if result_found {
            break;
        }
        if paths.is_empty() {
            //this is for debug only, shouldn't happen with correct input and correct algorythm
            println!("EMPTY PATHS");
            break;
        }
        let mut new_paths: Vec<Vec<[usize; 2]>> = vec![];
        for i in 0..paths.len() {
            if result_found {
                break;
            }
            let path = paths.get(i).unwrap();
            let current_point = path.last().unwrap();
            let current_point_neighbours = [
                [current_point[0] as i32 - 1, current_point[1] as i32], //left
                [current_point[0] as i32, current_point[1] as i32 - 1], //top
                [current_point[0] as i32 + 1, current_point[1] as i32], //right
                [current_point[0] as i32, current_point[1] as i32 + 1], //bottom
            ];
            for neighbour in current_point_neighbours {
                if !(neighbour[1] < 0
                    || neighbour[0] < 0
                    || path.contains(&[neighbour[0] as usize, neighbour[1] as usize]))
                {
                    if let Some(neighbour_row) = heights.get(neighbour[1] as usize) {
                        if let Some(neighbour_height) = neighbour_row.get(neighbour[0] as usize) {
                            if *neighbour_height as i32
                                - heights[current_point[1]][current_point[0]] as i32
                                >= -1
                            {
                                if !ends_with_char
                                    && [neighbour[0] as usize, neighbour[1] as usize] == end_point
                                    || ends_with_char && *neighbour_height == end_char
                                {
                                    winner_path = paths.get(i).unwrap().clone();
                                    result_found = true;
                                    break;
                                } else {
                                    //add path to point only if it it better than some old one
                                    //and if there is some path, it is shorter
                                    //or equal than new one, becaus with cycles paths only get longer
                                    if let std::collections::hash_map::Entry::Vacant(e) =
                                        best_points_paths
                                            .entry([neighbour[1] as usize, neighbour[0] as usize])
                                    {
                                        e.insert(path.clone());
                                        let mut new_path = path.clone();
                                        new_path
                                            .push([neighbour[0] as usize, neighbour[1] as usize]);
                                        new_paths.push(new_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        paths = new_paths.clone();
    }
    winner_path.len()
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut heights: Vec<Vec<char>> = vec![];
    let mut start_point: [usize; 2] = [0, 0];
    let mut end_point: [usize; 2] = [0, 0];
    let mut current_y = 0;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let mut row = vec![];
            for (current_x, ch) in user_input.trim().chars().enumerate() {
                let v = if ch == 'S' {
                    start_point = [current_x, current_y];
                    'a'
                } else if ch == 'E' {
                    end_point = [current_x, current_y];
                    'z'
                } else {
                    ch
                };
                row.push(v);
            }
            heights.push(row);
            current_y += 1;

            user_input.clear();
        }
    }

    //look for the path backwards
    println!(
        "result part 1: {}",
        path_len(&heights, end_point, start_point, false, 'a')
    );
    println!(
        "result part 2: {}",
        path_len(&heights, end_point, end_point, true, 'a')
    );
}
