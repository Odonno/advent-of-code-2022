use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: Beacon,
}

#[derive(Debug, PartialEq)]
enum Object {
    None,
    Sensor,
    Beacon,
}

type Map = HashMap<(i32, i32), Object>;

pub fn run(input: &str, use_sample: bool) {
    let lines = input.lines();

    let sensors = lines
        .map(|line| {
            let regex = Regex::new(
                r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$",
            )
            .unwrap();

            let captures = regex.captures(line).unwrap();
            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let closest_beacon_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let closest_beacon_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

            Sensor {
                x,
                y,
                closest_beacon: Beacon {
                    x: closest_beacon_x,
                    y: closest_beacon_y,
                },
            }
        })
        .collect::<Vec<_>>();

    let mut map: Map = Map::new();

    for sensor in sensors.iter() {
        map.insert((sensor.x, sensor.y), Object::Sensor);
        map.insert(
            (sensor.closest_beacon.x, sensor.closest_beacon.y),
            Object::Beacon,
        );
    }

    let y_search: i32 = if use_sample { 10 } else { 2_000_000 };

    for sensor in sensors.iter() {
        let max_distance =
            (sensor.x - sensor.closest_beacon.x).abs() + (sensor.y - sensor.closest_beacon.y).abs();

        let distance_to_y = (sensor.y - y_search).abs();

        if distance_to_y > max_distance {
            continue;
        }

        let diff = max_distance - distance_to_y;

        let min_x = sensor.x - diff;
        let max_x = sensor.x + diff;

        for x in min_x..=max_x {
            if map.get(&(x, y_search)).is_none() {
                map.insert((x, y_search), Object::None);
            }
        }
    }

    let total_none_on_y = map
        .into_iter()
        .filter(|((_, y), _)| *y == y_search)
        .filter(|(_, object)| *object != Object::Beacon)
        .count();

    println!("{:?}", total_none_on_y);
}
