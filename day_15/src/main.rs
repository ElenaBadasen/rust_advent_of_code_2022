use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut sensors: HashMap<[i64; 2], [i64; 2]> = HashMap::new();
    let mut beacons: Vec<[i64; 2]> = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let info: Vec<&str> = user_input.trim().split(' ').collect();
            let sensor_x: i64 = info[2].replace(['x', '=', ','], "").parse().unwrap();
            let sensor_y: i64 = info[3].replace(['y', '=', ':'], "").parse().unwrap();
            let beacon_x: i64 = info[8].replace(['x', '=', ','], "").parse().unwrap();
            let beacon_y: i64 = info[9].replace(['y', '='], "").parse().unwrap();
            sensors.insert([sensor_x, sensor_y], [beacon_x, beacon_y]);
            beacons.push([beacon_x, beacon_y]);
            user_input.clear();
        }
    }

    //let the_y = 10;
    let the_y = 2000000;
    let mut no_beacon_here: HashSet<[i64; 2]> = HashSet::new();

    for (sensor, beacon) in &sensors {
        let distance_to_beacon = (sensor[0] - beacon[0]).abs() + (sensor[1] - beacon[1]).abs();
        let distance_to_line = (sensor[1] - the_y).abs();
        for x in (sensor[0] - (distance_to_beacon - distance_to_line))
            ..(sensor[0] + (distance_to_beacon - distance_to_line) + 1)
        {
            no_beacon_here.insert([x, the_y]);
        }
    }
    for beacon in &beacons {
        if no_beacon_here.contains(beacon) {
            no_beacon_here.remove(beacon);
        }
    }

    println!("result part 1: {}", no_beacon_here.len());

    //for part 2 we know that if distress beacon position in unique, it should be on distance
    //distance_to_beacon + 1 from at least one sensor
    for (sensor, beacon) in &sensors {
        let distance_to_beacon = (sensor[0] - beacon[0]).abs() + (sensor[1] - beacon[1]).abs();
        for y in 0..4000001 {
            let x1 = distance_to_beacon + 1 - (y - sensor[1]).abs() + sensor[0];
            let x2 = -(distance_to_beacon + 1 - (y - sensor[1]).abs()) + sensor[0];
            for x in [x1, x2] {
                if (0..=4000000).contains(&x) {
                    let mut found_place = true;
                    for (sensor_inner, beacon_inner) in &sensors {
                        let distance_to_beacon_inner = (sensor_inner[0] - beacon_inner[0]).abs()
                            + (sensor_inner[1] - beacon_inner[1]).abs();
                        if (x - sensor_inner[0]).abs() + (y - sensor_inner[1]).abs()
                            <= distance_to_beacon_inner
                        {
                            found_place = false;
                            break;
                        }
                    }
                    if found_place {
                        println!("result part 2: {}", x * 4000000 + y);
                        return;
                    }
                }
            }
        }
    }
}
