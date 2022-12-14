use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
enum Terrain {
    Air,
    Stone,
    Sand,
}

fn process_sand(
    sand_pos: [u32; 2],
    mut terrain_map: HashMap<u32, HashMap<u32, Terrain>>,
    lowest_stone: u32,
) -> (Option<[u32; 2]>, HashMap<u32, HashMap<u32, Terrain>>) {
    let mut sand_rested = false;
    let mut new_sand_pos = sand_pos;
    while !sand_rested {
        if new_sand_pos[1] > lowest_stone {
            return (None, terrain_map);
        }
        match terrain_map
            .entry(new_sand_pos[1] + 1)
            .or_default()
            .entry(new_sand_pos[0])
            .or_insert(Terrain::Air)
        {
            Terrain::Air => {
                new_sand_pos = [new_sand_pos[0], new_sand_pos[1] + 1];
            }
            _default => {
                match terrain_map
                    .entry(new_sand_pos[1] + 1)
                    .or_default()
                    .entry(new_sand_pos[0] - 1)
                    .or_insert(Terrain::Air)
                {
                    Terrain::Air => {
                        new_sand_pos = [new_sand_pos[0] - 1, new_sand_pos[1] + 1];
                    }
                    _default => {
                        match terrain_map
                            .entry(new_sand_pos[1] + 1)
                            .or_default()
                            .entry(new_sand_pos[0] + 1)
                            .or_insert(Terrain::Air)
                        {
                            Terrain::Air => {
                                new_sand_pos = [new_sand_pos[0] + 1, new_sand_pos[1] + 1];
                            }
                            _default => {
                                sand_rested = true;
                            }
                        }
                    }
                }
            }
        }
    }
    (Some(new_sand_pos), terrain_map)
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut terrain_map: HashMap<u32, HashMap<u32, Terrain>> = HashMap::new();
    let mut lowest_stone = 0;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let stone_path: Vec<&str> = user_input.trim().split(" -> ").collect();
            let mut prev_x = 0;
            let mut prev_y = 0;
            for (index, value) in stone_path.iter().enumerate() {
                let xy: Vec<u32> = value
                    .split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect();
                let x = xy[0];
                let y = xy[1];
                if index == 0 {
                    prev_x = x;
                    prev_y = y;
                }
                if prev_x == x {
                    //vertical line
                    let ys = [prev_y, y];
                    let left_y = ys.iter().min().unwrap();
                    let right_y = ys.iter().max().unwrap();
                    for j in *left_y..(right_y + 1) {
                        let row = terrain_map.entry(j).or_default();
                        if j > lowest_stone {
                            lowest_stone = j;
                        }
                        row.insert(x, Terrain::Stone);
                    }
                } else {
                    //horizontal line
                    if y > lowest_stone {
                        lowest_stone = y;
                    }
                    let xs = [prev_x, x];
                    let top_x = xs.iter().min().unwrap();
                    let bottom_x = xs.iter().max().unwrap();
                    for i in *top_x..(bottom_x + 1) {
                        let row = terrain_map.entry(y).or_default();
                        row.insert(i, Terrain::Stone);
                    }
                }
                prev_x = x;
                prev_y = y;
            }
            user_input.clear();
        }
    }

    let mut terrain_map_part_2 = terrain_map.clone();

    let mut sand_in_free_fall = false;
    let mut rested_units = 0;
    while !sand_in_free_fall {
        //sand appears at 500,0
        let sand_pos = [500, 0];

        let (sand_pos_option, new_terrain_map) = process_sand(sand_pos, terrain_map, lowest_stone);
        terrain_map = new_terrain_map;
        match sand_pos_option {
            None => {
                sand_in_free_fall = true;
            }
            Some(pos) => {
                rested_units += 1;
                terrain_map
                    .entry(pos[1])
                    .or_default()
                    .insert(pos[0], Terrain::Sand);
            }
        }
    }

    println!("result part 1: {}", rested_units);

    let new_lowest_stone = lowest_stone + 2;

    for i in (500 - new_lowest_stone * 3)..(500 + new_lowest_stone * 3) {
        terrain_map_part_2
            .entry(lowest_stone + 2)
            .or_default()
            .insert(i, Terrain::Stone);
    }

    let mut rested_units_part_2 = 0;
    let mut sand_blocked = false;
    while !sand_blocked {
        //sand appears at 500,0
        let sand_pos = [500, 0];

        let (sand_pos_option, new_terrain_map) =
            process_sand(sand_pos, terrain_map_part_2, new_lowest_stone);
        terrain_map_part_2 = new_terrain_map;
        match sand_pos_option {
            None => {
                //should not happen here
                break;
            }
            Some(pos) => {
                rested_units_part_2 += 1;
                terrain_map_part_2
                    .entry(pos[1])
                    .or_default()
                    .insert(pos[0], Terrain::Sand);
                if pos == [500, 0] {
                    sand_blocked = true;
                }
            }
        }
    }

    println!("result part 2: {}", rested_units_part_2);
}
