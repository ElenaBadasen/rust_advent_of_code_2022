use std::collections::HashSet;
use std::io;

#[derive(Clone, Debug)]
enum Terrain {
    Lava,
    UnchartedWater,
    AirPocket,
    FreeWater,
}

//get air pocket, starting with given coordinate
//gives set of cubes and info, is it free air/water or a pocket.
fn search(coordinate: &[usize; 3], space: &[Vec<Vec<Terrain>>]) -> (HashSet<[usize; 3]>, Terrain) {
    let mut visited_cubes: HashSet<[usize; 3]> = HashSet::from_iter(vec![*coordinate]);
    let mut visited_water_cubes: HashSet<[usize; 3]> = HashSet::from_iter(vec![*coordinate]);
    let mut current_cubes: HashSet<[usize; 3]> = HashSet::from_iter(vec![*coordinate]);
    while !current_cubes.is_empty() {
        let mut new_current_cubes: HashSet<[usize; 3]> = HashSet::new();
        for cube in &current_cubes {
            for neighbour in &[
                [cube[0] + 1, cube[1], cube[2]],
                [cube[0] - 1, cube[1], cube[2]],
                [cube[0], cube[1] + 1, cube[2]],
                [cube[0], cube[1] - 1, cube[2]],
                [cube[0], cube[1], cube[2] + 1],
                [cube[0], cube[1], cube[2] - 1],
            ] {
                if !visited_cubes.contains(neighbour) {
                    visited_cubes.insert(*neighbour);
                    match space[neighbour[2]][neighbour[1]][neighbour[0]] {
                        Terrain::Lava => {
                            //do nothing
                        }
                        Terrain::UnchartedWater => {
                            visited_water_cubes.insert(*neighbour);
                            new_current_cubes.insert(*neighbour);
                        }
                        Terrain::AirPocket => {
                            //finished the search
                            return (visited_water_cubes, Terrain::AirPocket);
                        }
                        Terrain::FreeWater => {
                            //finished the search
                            return (visited_water_cubes, Terrain::FreeWater);
                        }
                    }
                }
            }
        }
        current_cubes = new_current_cubes;
    }
    //we haven't met free water, that means we are in an air pocket
    (visited_water_cubes, Terrain::AirPocket)
}

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
    let mut space: Vec<Vec<Vec<Terrain>>> =
        vec![vec![vec![Terrain::UnchartedWater; max_x + 2]; max_y + 2]; max_z + 2];
    for cube in &cubes {
        space[cube[2]][cube[1]][cube[0]] = Terrain::Lava;
    }

    //mark edge cubes as Terrain::FreeWater
    for z in [0, max_z + 1] {
        for y in 0..(max_y + 2) {
            for x in 0..(max_x + 2) {
                space[z][y][x] = Terrain::FreeWater;
            }
        }
    }

    for y in [0, max_y + 1] {
        for z in 0..(max_z + 2) {
            for x in 0..(max_x + 2) {
                space[z][y][x] = Terrain::FreeWater;
            }
        }
    }

    for x in [0, max_x + 1] {
        for y in 0..(max_y + 2) {
            for z in 0..(max_z + 2) {
                space[z][y][x] = Terrain::FreeWater;
            }
        }
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
            match space[neighbour[2]][neighbour[1]][neighbour[0]] {
                Terrain::Lava => {}
                Terrain::FreeWater => {
                    free_flats += 1;
                }
                Terrain::AirPocket => {}
                Terrain::UnchartedWater => {
                    let (water_cubes, water_type) = search(neighbour, &space);
                    if matches!(water_type, Terrain::FreeWater) {
                        free_flats += 1;
                    }
                    for inner_cube in &water_cubes {
                        space[inner_cube[2]][inner_cube[1]][inner_cube[0]] = water_type.clone();
                    }
                }
            }
        }
    }

    println!("result part 2: {}", free_flats);
}
