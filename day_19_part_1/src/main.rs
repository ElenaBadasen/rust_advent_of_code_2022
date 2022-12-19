use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
struct Factory {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_obsidian: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn possible_robots(ore: u32, clay: u32, obsidian: u32, factory: &Factory) -> HashSet<[u32; 4]> {
    let mut result: HashSet<[u32; 4]> = HashSet::new();
    result.insert([0; 4]);
    if ore / factory.ore_robot_cost > 0 {
        result.insert([1, 0, 0, 0]);
    }

    if ore / factory.clay_robot_cost > 0 {
        result.insert([0, 1, 0, 0]);
    }

    let max_possible_obsidian_robots_by_ore = ore / factory.obsidian_robot_cost_ore;
    let max_possible_obsidian_robots_by_clay = clay / factory.obsidian_robot_cost_clay;
    let obsidian_arr = [
        max_possible_obsidian_robots_by_ore,
        max_possible_obsidian_robots_by_clay,
    ];
    if *obsidian_arr.iter().min().unwrap() > 0 {
        result.insert([0, 0, 1, 0]);
    }

    let max_possible_geode_robots_by_ore = ore / factory.geode_robot_cost_ore;
    let max_possible_geode_robots_by_obsidian = obsidian / factory.geode_robot_cost_obsidian;
    let geode_arr = [
        max_possible_geode_robots_by_ore,
        max_possible_geode_robots_by_obsidian,
    ];
    if *geode_arr.iter().min().unwrap() > 0 {
        result.insert([0, 0, 0, 1]);
    }
    result
}

fn factory_max_geodes(factory: &Factory) -> u32 {
    let start_state = State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };
    let mut current_states: HashSet<State> = HashSet::from_iter(vec![start_state]);
    let mut shortest_paths: HashMap<[u32; 4], u32> = HashMap::new();
    let mut enough_ore_robots_for_everything = false;
    let mut enough_clay_robots_for_everything = false;
    let mut enough_obsidian_robots_for_everything = false;
    for i in 1..=24 {
        //println!("step {}", i);
        let mut new_current_states: HashSet<State> = HashSet::new();
        for state in &current_states {
            for possible_robots_arr in
                possible_robots(state.ore, state.clay, state.obsidian, factory)
            {
                if possible_robots_arr != [0; 4] {
                    let key = [
                        possible_robots_arr[0] + state.ore_robots,
                        possible_robots_arr[1] + state.clay_robots,
                        possible_robots_arr[2] + state.obsidian_robots,
                        possible_robots_arr[3] + state.geode_robots,
                    ];
                    match shortest_paths.get(&key) {
                        None => {
                            //do not save shortest path if resources are depleted
                            if state.ore
                                >= (factory.ore_robot_cost
                                    + factory.clay_robot_cost
                                    + factory.obsidian_robot_cost_ore
                                    + factory.geode_robot_cost_ore)
                                && state.clay >= factory.obsidian_robot_cost_clay
                                && state.obsidian >= factory.geode_robot_cost_obsidian
                            {
                                shortest_paths.insert(key, i);
                            }
                        }
                        Some(shortest_path) => {
                            if *shortest_path < i {
                                continue;
                            }
                        }
                    }
                }

                let mut new_state = state.clone();
                new_state.ore += new_state.ore_robots;
                new_state.clay += new_state.clay_robots;
                new_state.obsidian += new_state.obsidian_robots;
                new_state.geodes += new_state.geode_robots;

                if !enough_ore_robots_for_everything {
                    new_state.ore_robots += possible_robots_arr[0];
                    new_state.ore -= possible_robots_arr[0] * factory.ore_robot_cost;
                }
                if !enough_clay_robots_for_everything {
                    new_state.clay_robots += possible_robots_arr[1];
                    new_state.ore -= possible_robots_arr[1] * factory.clay_robot_cost;
                }

                if !enough_obsidian_robots_for_everything {
                    new_state.obsidian_robots += possible_robots_arr[2];
                    new_state.clay -= possible_robots_arr[2] * factory.obsidian_robot_cost_clay;
                    new_state.ore -= possible_robots_arr[2] * factory.obsidian_robot_cost_ore;
                }

                new_state.obsidian -= possible_robots_arr[3] * factory.geode_robot_cost_obsidian;
                new_state.ore -= possible_robots_arr[3] * factory.geode_robot_cost_ore;
                new_state.geode_robots += possible_robots_arr[3];

                enough_ore_robots_for_everything = enough_ore_robots_for_everything
                    || new_state.ore_robots
                        > (factory.clay_robot_cost
                            + factory.ore_robot_cost
                            + factory.obsidian_robot_cost_ore
                            + factory.geode_robot_cost_ore);

                enough_clay_robots_for_everything = enough_clay_robots_for_everything
                    || new_state.clay_robots > factory.obsidian_robot_cost_clay;

                enough_obsidian_robots_for_everything = enough_obsidian_robots_for_everything
                    || new_state.obsidian_robots > factory.geode_robot_cost_obsidian;

                new_current_states.insert(new_state);
            }
        }
        current_states = new_current_states;
    }
    current_states.iter().map(|s| s.geodes).max().unwrap() * factory.id
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut factories: Vec<Factory> = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let input_split: Vec<&str> = user_input.trim().split(' ').collect();
            let factory = Factory {
                id: input_split[1].replace(':', "").parse().unwrap(),
                ore_robot_cost: input_split[6].parse().unwrap(),
                clay_robot_cost: input_split[12].parse().unwrap(),
                obsidian_robot_cost_ore: input_split[18].parse().unwrap(),
                obsidian_robot_cost_clay: input_split[21].parse().unwrap(),
                geode_robot_cost_ore: input_split[27].parse().unwrap(),
                geode_robot_cost_obsidian: input_split[30].parse().unwrap(),
            };
            factories.push(factory);
            user_input.clear();
        }
    }

    let mut result: u32 = 0;
    for factory in &factories {
        result += factory_max_geodes(factory);
    }

    println!("result part 1: {}", result);
}
