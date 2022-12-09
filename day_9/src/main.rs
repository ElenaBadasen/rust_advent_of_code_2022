use std::collections::HashSet;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut positions_set: HashSet<[i32; 2]> = HashSet::new();
    let mut positions_set_part_2: HashSet<[i32; 2]> = HashSet::new();
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    positions_set.insert(tail);
    positions_set_part_2.insert(tail);
    let mut rope_part_2: [[i32; 2]; 10] = [[0, 0]; 10];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let command: Vec<&str> = user_input.trim().split(' ').collect();
            for _i in 0..command[1].parse().unwrap() {
                match command[0] {
                    "R" => {
                        head[0] += 1;
                        rope_part_2[9][0] += 1;
                    }
                    "U" => {
                        head[1] += 1;
                        rope_part_2[9][1] += 1;
                    }
                    "L" => {
                        head[0] -= 1;
                        rope_part_2[9][0] -= 1;
                    }
                    "D" => {
                        head[1] -= 1;
                        rope_part_2[9][1] -= 1;
                    }
                    _default => {}
                }

                //part 1
                if (head[0] - tail[0]).abs() <= 1 && (head[1] - tail[1]).abs() <= 1 {
                    //tail doesn't move
                } else if (head[0] - tail[0]) * (head[1] - tail[1]) == 0 {
                    //tail and head in the same column or row
                    if (head[0] - tail[0]).abs() > 0 {
                        if head[0] > tail[0] {
                            tail[0] += 1;
                        } else {
                            tail[0] -= 1;
                        }
                    } else if head[1] > tail[1] {
                        tail[1] += 1;
                    } else {
                        tail[1] -= 1;
                    }
                    positions_set.insert(tail);
                } else {
                    let tail_x_move = if head[0] > tail[0] { 1 } else { -1 };
                    let tail_y_move = if head[1] > tail[1] { 1 } else { -1 };
                    tail[0] += tail_x_move;
                    tail[1] += tail_y_move;
                    positions_set.insert(tail);
                }

                //part 2
                for j in (1..rope_part_2.len()).rev() {
                    if (rope_part_2[j][0] - rope_part_2[j - 1][0]).abs() <= 1
                        && (rope_part_2[j][1] - rope_part_2[j - 1][1]).abs() <= 1
                    {
                        //tail doesn't move
                    } else if (rope_part_2[j][0] - rope_part_2[j - 1][0])
                        * (rope_part_2[j][1] - rope_part_2[j - 1][1])
                        == 0
                    {
                        //tail and head in the same column or row
                        if (rope_part_2[j][0] - rope_part_2[j - 1][0]).abs() > 0 {
                            if rope_part_2[j][0] > rope_part_2[j - 1][0] {
                                rope_part_2[j - 1][0] += 1;
                            } else {
                                rope_part_2[j - 1][0] -= 1;
                            }
                        } else if rope_part_2[j][1] > rope_part_2[j - 1][1] {
                            rope_part_2[j - 1][1] += 1;
                        } else {
                            rope_part_2[j - 1][1] -= 1;
                        }
                        if j == 1 {
                            positions_set_part_2.insert(rope_part_2[j - 1]);
                        }
                    } else {
                        let tail_x_move = if rope_part_2[j][0] > rope_part_2[j - 1][0] {
                            1
                        } else {
                            -1
                        };
                        let tail_y_move = if rope_part_2[j][1] > rope_part_2[j - 1][1] {
                            1
                        } else {
                            -1
                        };
                        rope_part_2[j - 1][0] += tail_x_move;
                        rope_part_2[j - 1][1] += tail_y_move;
                        if j == 1 {
                            positions_set_part_2.insert(rope_part_2[j - 1]);
                        }
                    }
                }
            }
            user_input.clear();
        }
    }

    let result = positions_set.len();
    let result2 = positions_set_part_2.len();
    println!("result part 1: {}", result);
    println!("result part 2: {}", result2);
}
