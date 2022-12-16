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

#[derive(Debug, Clone)]
struct ValveOpening {
    valve: Valve,
    minute: u8,
}

type OpenedValves = HashMap<String, Valve>;
type TunnelCrosses = HashMap<(String, String), u8>;

const TOTAL_MINUTES: u8 = 30;
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

    let current_valve_name = String::from("AA");
    let current_valve = valves
        .iter()
        .find(|valve| valve.name == current_valve_name)
        .unwrap();

    let valve_openings = get_best_valve_openings(
        &tunnel_crosses,
        &valves,
        &current_valve,
        &opened_valves,
        TOTAL_MINUTES,
    );

    let pressure_released = get_total_pressure_released(&valve_openings);

    println!("{:?}", pressure_released);
}

fn get_total_pressure_released(valve_openings: &Vec<ValveOpening>) -> u32 {
    valve_openings
        .iter()
        .map(|valve_opening| {
            let valve = valve_opening.valve.clone();
            let minute = valve_opening.minute;

            let minutes_left = TOTAL_MINUTES - minute;

            valve.flow_rate as u32 * minutes_left as u32
        })
        .sum::<u32>()
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

fn get_best_valve_openings(
    tunnel_crosses: &TunnelCrosses,
    valves: &Vec<Valve>,
    current_valve: &Valve,
    opened_valves: &OpenedValves,
    remaining_minutes: u8,
) -> Vec<ValveOpening> {
    let closed_valves = valves
        .into_iter()
        .filter(|valve| !opened_valves.contains_key(&valve.name))
        .collect::<Vec<_>>();

    let next_possible_valves = closed_valves
        .into_iter()
        .filter(|valve| valve.flow_rate > 0)
        .filter(|valve| valve.name != current_valve.name)
        .collect::<Vec<_>>();

    if next_possible_valves.is_empty() {
        return vec![];
    }

    let best_valve_openings = next_possible_valves
        .into_iter()
        .map(|next_valve| {
            let from = &current_valve.name;
            let to = &next_valve.name;
            let key = (from.clone(), to.clone());

            let minutes_to_traverse = tunnel_crosses.get(&key).unwrap();

            let total_minutes = remaining_minutes as i32
                - *minutes_to_traverse as i32
                - MINUTE_TO_OPEN_VALVE as i32;

            if total_minutes <= 0 {
                return (0, vec![]);
            }

            let valve_opening = ValveOpening {
                valve: next_valve.clone(),
                minute: TOTAL_MINUTES - total_minutes as u8,
            };

            let mut valve_openings = vec![valve_opening];

            let mut next_opened_valves = opened_valves.clone();
            next_opened_valves.insert(next_valve.name.clone(), next_valve.clone());

            let next_valve_openings = get_best_valve_openings(
                tunnel_crosses,
                valves,
                next_valve,
                &next_opened_valves,
                total_minutes as u8,
            );

            for valve_opening in next_valve_openings {
                valve_openings.push(valve_opening);
            }

            let next_total_pressure = get_total_pressure_released(&valve_openings);

            (next_total_pressure, valve_openings)
        })
        .sorted_by(|(a_pressure, _), (b_pressure, _)| b_pressure.cmp(&a_pressure))
        .map(|(_, valve_openings)| valve_openings)
        .nth(0)
        .unwrap();

    best_valve_openings
}
