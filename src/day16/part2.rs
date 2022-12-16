use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    name: String,
    flow_rate: u8,
}

#[derive(Debug)]
struct Tunnel {
    from_valve: String,
    to_valve: String,
}

#[derive(Debug, Clone, PartialEq)]
enum Worker {
    Me,
    Elephant,
}

#[derive(Debug, Clone)]
struct ValveOpener {
    current_valve_name: String,
    worker: Worker,
    occupied_until_minute: u8,
}

type OpenedValves = HashMap<String, Valve>;
type TunnelCrosses = HashMap<(String, String), u8>;

const TOTAL_MINUTES: u8 = 26;
const MINUTE_TO_OPEN_VALVE: u8 = 1;

pub fn run(input: &str) {
    let lines = input.lines();

    let mut valves = Vec::new();
    let mut tunnels = Vec::new();

    for line in lines {
        let mut parts = line.split("; ");

        let valve_part = parts.next().unwrap();

        let regex = Regex::new(r"Valve (.+) has flow rate=(\d+)").unwrap();
        let captures = regex.captures(valve_part).unwrap();
        let valve_name = captures.get(1).unwrap().as_str();
        let valve_flow_rate = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();

        let valve = Valve {
            name: valve_name.to_string(),
            flow_rate: valve_flow_rate,
        };

        valves.push(valve);

        let tunnel_part = parts.next().unwrap();

        let regex = Regex::new(r"tunnel[s]? lead[s]? to valve[s]? (.+)").unwrap();
        let captures = regex.captures(tunnel_part).unwrap();
        let tunnel_to_valves = captures.get(1).unwrap().as_str().to_string();

        let to_valves = tunnel_to_valves.split(", ").collect::<Vec<_>>();

        for to_valve in to_valves {
            let tunnel = Tunnel {
                from_valve: valve_name.to_string(),
                to_valve: to_valve.to_string(),
            };

            tunnels.push(tunnel);
        }
    }

    let tunnel_crosses = extract_tunnel_crosses(tunnels, &valves);

    let opened_valves = OpenedValves::new();

    let valve_openers = vec![
        ValveOpener {
            current_valve_name: String::from("AA"),
            worker: Worker::Me,
            occupied_until_minute: 1,
        },
        ValveOpener {
            current_valve_name: String::from("AA"),
            worker: Worker::Elephant,
            occupied_until_minute: 1,
        },
    ];

    let openable_valves = valves
        .into_iter()
        .filter(|valve| valve.flow_rate > 0)
        .collect::<Vec<_>>();

    let pressure_released = get_best_total_pressure(
        &tunnel_crosses,
        &openable_valves,
        &opened_valves,
        &valve_openers,
    );

    println!("{:?}", pressure_released);
}

fn calculate_pressure_released(valve: &Valve, from_minute: u8) -> u32 {
    let minutes_left = (TOTAL_MINUTES + 1) - from_minute;
    valve.flow_rate as u32 * minutes_left as u32
}

fn extract_tunnel_crosses(tunnels: Vec<Tunnel>, valves: &Vec<Valve>) -> TunnelCrosses {
    let mut tunnel_crosses = TunnelCrosses::new();

    for tunnel in tunnels.iter() {
        let key = (tunnel.from_valve.clone(), tunnel.to_valve.clone());
        let inverted_key = (tunnel.to_valve.clone(), tunnel.from_valve.clone());

        tunnel_crosses.insert(key, 1);
        tunnel_crosses.insert(inverted_key, 1);
    }

    loop {
        let mut has_remaining = false;

        for from_valve in valves.iter() {
            for to_valve in valves.iter() {
                if from_valve == to_valve {
                    continue;
                }

                let key = (from_valve.name.clone(), to_valve.name.clone());
                let inverted_key = (to_valve.name.clone(), from_valve.name.clone());

                if tunnel_crosses.contains_key(&key) || tunnel_crosses.contains_key(&inverted_key) {
                    continue;
                }

                let number_of_cross =
                    calculate_number_of_crosses(&tunnel_crosses, from_valve, to_valve);
                if number_of_cross.is_some() {
                    let number_of_cross = number_of_cross.unwrap();

                    tunnel_crosses.insert(key, number_of_cross);
                    tunnel_crosses.insert(inverted_key, number_of_cross);

                    has_remaining = true;
                }
            }
        }

        if !has_remaining {
            break;
        }
    }

    tunnel_crosses
}

fn calculate_number_of_crosses(
    tunnel_crosses: &TunnelCrosses,
    from_valve: &Valve,
    to_valve: &Valve,
) -> Option<u8> {
    let crosses_from_valve = tunnel_crosses
        .into_iter()
        .filter(|(key, _)| key.0 == from_valve.name)
        .collect::<Vec<_>>();

    let crosses_to_valve = tunnel_crosses
        .into_iter()
        .filter(|(key, _)| key.1 == to_valve.name)
        .collect::<Vec<_>>();

    let mut min_number_of_crosses = None;

    for cross_from_valve in crosses_from_valve.iter() {
        for cross_to_valve in crosses_to_valve.iter() {
            if cross_from_valve.0 .1 == cross_to_valve.0 .0 {
                let number_of_crosses = cross_from_valve.1 + cross_to_valve.1;

                match min_number_of_crosses {
                    Some(min) if number_of_crosses < min => {
                        min_number_of_crosses = Some(number_of_crosses);
                    }
                    None => {
                        min_number_of_crosses = Some(number_of_crosses);
                    }
                    _ => {}
                }
            }
        }
    }

    min_number_of_crosses
}

fn get_best_total_pressure(
    tunnel_crosses: &TunnelCrosses,
    openable_valves: &Vec<Valve>,
    opened_valves: &OpenedValves,
    valve_openers: &Vec<ValveOpener>,
) -> u32 {
    if openable_valves.is_empty() {
        return 0;
    }

    let next_opener =
        match valve_openers[0].occupied_until_minute <= valve_openers[1].occupied_until_minute {
            true => &valve_openers[0],
            _ => &valve_openers[1],
        };

    let best_total_pressure = openable_valves
        .into_iter()
        .map(|next_valve| {
            let from = &next_opener.current_valve_name;
            let to = &next_valve.name;
            let key = (from.clone(), to.clone());

            let minutes_to_traverse = tunnel_crosses.get(&key).unwrap();

            let occupied_until_minute = (next_opener.occupied_until_minute
                + *minutes_to_traverse
                + MINUTE_TO_OPEN_VALVE) as u8;

            if occupied_until_minute >= TOTAL_MINUTES {
                return 0;
            }

            let next_valve_openers = valve_openers
                .iter()
                .map(|valve_opener| {
                    if valve_opener.worker == next_opener.worker {
                        return ValveOpener {
                            current_valve_name: next_valve.name.clone(),
                            worker: valve_opener.worker.clone(),
                            occupied_until_minute,
                        };
                    }

                    valve_opener.clone()
                })
                .collect::<Vec<_>>();

            let mut next_opened_valves = opened_valves.clone();
            next_opened_valves.insert(next_valve.name.clone(), next_valve.clone());

            let valve_pressure_released =
                calculate_pressure_released(&next_valve, occupied_until_minute);

            let next_openable_valves = openable_valves
                .iter()
                .filter(|valve| valve.name != next_valve.name)
                .map(|valve| valve.clone())
                .collect::<Vec<_>>();

            let next_total_pressure = get_best_total_pressure(
                tunnel_crosses,
                &next_openable_valves,
                &next_opened_valves,
                &next_valve_openers,
            );

            let total_pressure = valve_pressure_released + next_total_pressure;
            total_pressure
        })
        .sorted_by(|a, b| b.cmp(&a))
        .nth(0)
        .unwrap();

    best_total_pressure
}
