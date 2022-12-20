use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use regex::Regex;
use std::{ops::Add, str::Lines};

#[derive(Debug, Clone)]
struct Cost {
    ore: Option<u8>,
    clay: Option<u8>,
    obsidian: Option<u8>,
}

impl Add for Cost {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: match (self.ore, other.ore) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            clay: match (self.clay, other.clay) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            obsidian: match (self.obsidian, other.obsidian) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

#[derive(Debug, Clone)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
}

#[derive(Debug, PartialEq)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Action = Vec<ResourceType>;
type Actions = Vec<Action>;

const TOTAL_MINUTES: u8 = 32;

pub fn run(input: &str) {
    let lines = input.lines();

    let blueprints = parse_input(lines);
    let first_three_blueprints = blueprints.into_iter().take(3).collect::<Vec<_>>();

    const INITIAL_ROBOTS: Resources = Resources {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    const INITIAL_RESOURCES: Resources = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    let best_number_of_geodes_per_blueprint = first_three_blueprints
        .iter()
        .map(|blueprint| {
            let max_ore_production = vec![
                blueprint.ore_robot_cost.ore.unwrap_or_default(),
                blueprint.clay_robot_cost.ore.unwrap_or_default(),
                blueprint.obsidian_robot_cost.ore.unwrap_or_default(),
                blueprint.geode_robot_cost.ore.unwrap_or_default(),
            ]
            .into_iter()
            .max()
            .unwrap();

            let max_clay_production = vec![
                blueprint.ore_robot_cost.clay.unwrap_or_default(),
                blueprint.clay_robot_cost.clay.unwrap_or_default(),
                blueprint.obsidian_robot_cost.clay.unwrap_or_default(),
                blueprint.geode_robot_cost.clay.unwrap_or_default(),
            ]
            .into_iter()
            .max()
            .unwrap();

            let max_obsidian_production = vec![
                blueprint.ore_robot_cost.obsidian.unwrap_or_default(),
                blueprint.clay_robot_cost.obsidian.unwrap_or_default(),
                blueprint.obsidian_robot_cost.obsidian.unwrap_or_default(),
                blueprint.geode_robot_cost.obsidian.unwrap_or_default(),
            ]
            .into_iter()
            .max()
            .unwrap();

            let best_number_of_geodes = calculate_best_number_of_geodes(
                blueprint,
                &INITIAL_ROBOTS,
                &INITIAL_RESOURCES,
                max_ore_production,
                max_clay_production,
                max_obsidian_production,
                1,
            );
            best_number_of_geodes
        })
        .collect::<Vec<_>>();

    let total = best_number_of_geodes_per_blueprint
        .into_iter()
        .fold(1, |total, number_of_geodes| {
            total as u16 * number_of_geodes as u16
        });

    println!("{:?}", total);
}

fn calculate_best_number_of_geodes(
    blueprint: &Blueprint,
    robots: &Resources,
    resources: &Resources,
    max_ore_production: u8,
    max_clay_production: u8,
    max_obsidian_production: u8,
    minute: u8,
) -> u8 {
    if minute > TOTAL_MINUTES {
        return resources.geodes;
    }

    const FIRST_QUARTER: u8 = TOTAL_MINUTES / 4;
    const FIRST_HALF: u8 = TOTAL_MINUTES / 2;
    const LAST_QUARTER: u8 = TOTAL_MINUTES - FIRST_QUARTER;

    let mut actions = Actions::new();

    if minute >= LAST_QUARTER && robots.geodes == 0 {
        // nothing
    } else {
        let has_reached_max_ore_production = robots.ore >= max_ore_production;
        let has_reached_max_clay_production = robots.clay >= max_clay_production;
        let has_reached_max_obsidian_production = robots.obsidian >= max_obsidian_production;

        let can_build_ore = can_build(&blueprint.ore_robot_cost, &resources);
        let can_build_clay = can_build(&blueprint.clay_robot_cost, &resources);
        let can_build_obsidian = can_build(&blueprint.obsidian_robot_cost, &resources);
        let can_build_geode = can_build(&blueprint.geode_robot_cost, &resources);

        if !has_reached_max_ore_production && can_build_ore {
            actions.push(vec![ResourceType::Ore]);
        }

        if !has_reached_max_clay_production && can_build_clay {
            actions.push(vec![ResourceType::Clay]);
        }

        if !has_reached_max_obsidian_production && can_build_obsidian {
            actions.push(vec![ResourceType::Obsidian]);
        }

        if can_build_geode {
            actions.push(vec![ResourceType::Geode]);
        }

        let stopped_to_build_ore =
            has_reached_max_ore_production || can_build_ore || minute > FIRST_QUARTER;
        let stopped_to_build_clay =
            has_reached_max_clay_production || can_build_clay || minute > FIRST_HALF;
        let stopped_to_build_obsidian =
            has_reached_max_obsidian_production || can_build_obsidian || minute > LAST_QUARTER;

        if robots.clay == 0 && stopped_to_build_ore && stopped_to_build_clay {
            // nothing
        } else {
            if robots.obsidian == 0
                && stopped_to_build_ore
                && stopped_to_build_clay
                && stopped_to_build_obsidian
            {
                // nothing
            } else {
                if stopped_to_build_ore
                    && stopped_to_build_clay
                    && stopped_to_build_obsidian
                    && can_build_geode
                {
                    // nothing
                } else {
                    actions.push(vec![]);
                }
            }
        }
    }

    let all_moves = actions
        .par_iter()
        .map(|action| {
            let mut robots = robots.clone();
            let mut resources = resources.clone();

            execute_action(blueprint, action, &mut robots, &mut resources);

            (robots, resources)
        })
        .collect::<Vec<_>>();

    if all_moves.is_empty() {
        return 0;
    }

    let most_geodes = all_moves
        .par_iter()
        .map(|(robots, resources)| {
            calculate_best_number_of_geodes(
                blueprint,
                &robots,
                &resources,
                max_ore_production,
                max_clay_production,
                max_obsidian_production,
                minute + 1,
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by(|a, b| b.cmp(a))
        .nth(0)
        .unwrap();

    most_geodes
}

fn execute_action(
    blueprint: &Blueprint,
    action: &Action,
    robots: &mut Resources,
    resources: &mut Resources,
) {
    for resource_type in action {
        match resource_type {
            ResourceType::Ore => {
                let cost = &blueprint.ore_robot_cost;
                resources.ore -= cost.ore.unwrap_or_default();
                resources.clay -= cost.clay.unwrap_or_default();
                resources.obsidian -= cost.obsidian.unwrap_or_default();
            }
            ResourceType::Clay => {
                let cost = &blueprint.clay_robot_cost;
                resources.ore -= cost.ore.unwrap_or_default();
                resources.clay -= cost.clay.unwrap_or_default();
                resources.obsidian -= cost.obsidian.unwrap_or_default();
            }
            ResourceType::Obsidian => {
                let cost = &blueprint.obsidian_robot_cost;
                resources.ore -= cost.ore.unwrap_or_default();
                resources.clay -= cost.clay.unwrap_or_default();
                resources.obsidian -= cost.obsidian.unwrap_or_default();
            }
            ResourceType::Geode => {
                let cost = &blueprint.geode_robot_cost;
                resources.ore -= cost.ore.unwrap_or_default();
                resources.clay -= cost.clay.unwrap_or_default();
                resources.obsidian -= cost.obsidian.unwrap_or_default();
            }
        };
    }

    resources.ore += robots.ore;
    resources.clay += robots.clay;
    resources.obsidian += robots.obsidian;
    resources.geodes += robots.geodes;

    for resource_type in action {
        match resource_type {
            ResourceType::Ore => robots.ore += 1,
            ResourceType::Clay => robots.clay += 1,
            ResourceType::Obsidian => robots.obsidian += 1,
            ResourceType::Geode => robots.geodes += 1,
        }
    }
}

fn can_build(cost: &Cost, resources: &Resources) -> bool {
    let has_enough_ore = match cost.ore {
        None => true,
        Some(ore_cost) => resources.ore >= ore_cost,
    };
    let has_enough_clay = match cost.clay {
        None => true,
        Some(clay_cost) => resources.clay >= clay_cost,
    };
    let has_enough_obsidian = match cost.obsidian {
        None => true,
        Some(obsidian_cost) => resources.obsidian >= obsidian_cost,
    };

    has_enough_ore && has_enough_clay && has_enough_obsidian
}

fn parse_input(lines: Lines) -> Vec<Blueprint> {
    let blueprints = lines.map(
        |line| {
            let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
            let captures = regex.captures(line).unwrap();

            let _id = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();

            let ore_robot_cost = Cost {
                ore: captures.get(2).unwrap().as_str().parse::<u8>().ok(),
                clay: None,
                obsidian: None,
            };

            let clay_robot_cost = Cost {
                ore: captures.get(3).unwrap().as_str().parse::<u8>().ok(),
                clay: None,
                obsidian: None,
            };

            let obsidian_robot_cost = Cost {
                ore: captures.get(4).unwrap().as_str().parse::<u8>().ok(),
                clay: captures.get(5).unwrap().as_str().parse::<u8>().ok(),
                obsidian: None,
            };

            let geode_robot_cost = Cost {
                ore: captures.get(6).unwrap().as_str().parse::<u8>().ok(),
                clay: None,
                obsidian: captures.get(7).unwrap().as_str().parse::<u8>().ok(),
            };

            Blueprint {
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            }
        }
    )
    .collect::<Vec<_>>();

    blueprints
}
