use regex::Regex;

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

    let distress_beacon_min_x: i32 = 0;
    let distress_beacon_max_x: i32 = if use_sample { 20 } else { 4_000_000 };

    let distress_beacon_min_y: i32 = 0;
    let distress_beacon_max_y: i32 = if use_sample { 20 } else { 4_000_000 };

    let mut distress_beacon = None;

    for y in distress_beacon_min_y..=distress_beacon_max_y {
        distress_beacon =
            detect_distress_beacon(&sensors, y, distress_beacon_min_x, distress_beacon_max_x);

        if distress_beacon.is_some() {
            break;
        }
    }

    let distress_beacon = distress_beacon.unwrap();
    let tuning_frequency = get_tuning_frequency(distress_beacon);

    println!("{:?}", tuning_frequency);
}

fn detect_distress_beacon(
    sensors: &Vec<Sensor>,
    y_search: i32,
    min_x: i32,
    max_x: i32,
) -> Option<Beacon> {
    let mut used_ranges = Vec::new();

    for sensor in sensors.iter() {
        if sensor.y == y_search {
            used_ranges.push(sensor.x..=sensor.x);
        }

        if sensor.closest_beacon.y == y_search {
            used_ranges.push(sensor.closest_beacon.x..=sensor.closest_beacon.x);
        }
    }

    for sensor in sensors.iter() {
        let max_distance =
            (sensor.x - sensor.closest_beacon.x).abs() + (sensor.y - sensor.closest_beacon.y).abs();

        let distance_to_y = (sensor.y - y_search).abs();

        if distance_to_y > max_distance {
            continue;
        }

        let diff = max_distance - distance_to_y;

        let min_x = (sensor.x - diff).clamp(min_x, max_x);
        let max_x = (sensor.x + diff).clamp(min_x, max_x);

        used_ranges.push(min_x..=max_x);
    }

    let mut x = 0;

    loop {
        let range = used_ranges.iter().find(|range| range.contains(&x));
        if range.is_none() {
            return Some(Beacon { x, y: y_search });
        }

        x = range.unwrap().end() + 1;

        if x > max_x {
            break;
        }
    }

    None
}

fn get_tuning_frequency(beacon: Beacon) -> u128 {
    (beacon.x as u128 * 4000000) + beacon.y as u128
}
