use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut cubes: Vec<[usize; 3]> = vec![];
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut max_z: usize = 0;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            //make +1 to all coordinates to simplify calculations with substractions,
            //to not go negative
            let arr: [usize; 3] = user_input
                .trim()
                .split(',')
                .map(|u| u.parse::<usize>().unwrap() + 1)
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            if arr[0] > max_x {
                max_x = arr[0];
            }
            if arr[1] > max_y {
                max_y = arr[1];
            }
            if arr[2] > max_z {
                max_z = arr[2];
            }
            cubes.push(arr);
            user_input.clear();
        }
    }
    let mut space: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_x + 2]; max_y + 2]; max_z + 2];
    for cube in &cubes {
        space[cube[2]][cube[1]][cube[0]] = true;
    }
    let mut free_flats = 0;
    for cube in &cubes {
        for neighbour in &[
            [cube[0] + 1, cube[1], cube[2]],
            [cube[0] - 1, cube[1], cube[2]],
            [cube[0], cube[1] + 1, cube[2]],
            [cube[0], cube[1] - 1, cube[2]],
            [cube[0], cube[1], cube[2] + 1],
            [cube[0], cube[1], cube[2] - 1],
        ] {
            if !space[neighbour[2]][neighbour[1]][neighbour[0]] {
                free_flats += 1;
            }
        }
    }

    println!("result part 1: {}", free_flats);
}
