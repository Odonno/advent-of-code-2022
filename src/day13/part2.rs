use itertools::Itertools;

type Integer = u32;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Value(Integer),
}

pub fn run(input: &str) {
    let lines = input.lines();

    let lines_array = lines.collect::<Vec<_>>();

    let splitted_lines = lines_array.split(|&s| s.is_empty()).collect::<Vec<_>>();

    let input_packets = splitted_lines
        .into_iter()
        .flat_map(|lines| {
            let left = parse_packet(lines[0]);
            let right = parse_packet(lines[1]);

            vec![left, right]
        })
        .collect::<Vec<_>>();

    let packet_with_two = Packet::List(vec![Packet::Value(2)]);
    let packet_with_six = Packet::List(vec![Packet::Value(6)]);

    let addtionnal_packets = [&packet_with_two, &packet_with_six];

    let mut all_packets = input_packets;

    for packet in addtionnal_packets.into_iter() {
        all_packets.push(packet.clone());
    }

    let sorted_packets = all_packets
        .into_iter()
        .sorted_by(|a, b| compare_values(a, b).unwrap_or(false).cmp(&true))
        .rev()
        .collect::<Vec<_>>();

    let index_of_packet_with_two = sorted_packets
        .iter()
        .position(|packet| packet.clone() == packet_with_two)
        .unwrap();

    let index_of_packet_with_six = sorted_packets
        .iter()
        .position(|packet| packet.clone() == packet_with_six)
        .unwrap();

    let decoder_key = (index_of_packet_with_two + 1) * (index_of_packet_with_six + 1);

    println!("{:?}", decoder_key);
}

fn parse_packet(str: &str) -> Packet {
    let start_index = 1;
    let end_index = str.len() - 1;

    let mut packet = Vec::new();

    let mut current_str = String::new();

    let mut nested_level = 0;

    for index in start_index..=end_index {
        let next_char = str.get(index..index + 1).unwrap();
        let is_last_index = index == end_index;

        if next_char == "," || is_last_index {
            if nested_level == 0 {
                if current_str.is_empty() {
                    continue;
                }

                let has_nested_packet = current_str.contains("[");

                let packet_value = match has_nested_packet {
                    true => parse_packet(current_str.as_str()),
                    false => {
                        let value = current_str.parse::<Integer>().unwrap();
                        Packet::Value(value)
                    }
                };

                packet.push(packet_value);

                current_str = String::new();
                continue;
            }
        }
        if next_char == "[" {
            nested_level += 1;
        }
        if next_char == "]" {
            nested_level -= 1;
        }

        current_str += next_char;
    }

    Packet::List(packet)
}

fn compare_values(left_value: &Packet, right_value: &Packet) -> Option<bool> {
    match (left_value, right_value) {
        (Packet::Value(left_value), Packet::Value(right_value)) => {
            if left_value == right_value {
                return None;
            }

            Some(left_value < right_value)
        }
        (Packet::List(left_list), Packet::List(right_list)) => {
            let left_max_index = left_list.len();
            let right_max_index = right_list.len();

            let max_index = [left_max_index, right_max_index].into_iter().max().unwrap();

            for index in 0..max_index {
                let left_value = left_list.get(index);
                let right_value = right_list.get(index);

                let is_right_order = match (left_value, right_value) {
                    (None, None) => None,
                    (None, _) => Some(true),
                    (_, None) => Some(false),
                    (Some(left_value), Some(right_value)) => {
                        compare_values(left_value, right_value)
                    }
                };

                if is_right_order.is_some() {
                    return is_right_order;
                }
            }

            None
        }
        (left_list, Packet::Value(right_value)) => {
            let packet = vec![Packet::Value(right_value.clone())];
            let right_list = Packet::List(packet);

            compare_values(left_list, &right_list)
        }
        (Packet::Value(left_value), right_list) => {
            let packet = vec![Packet::Value(left_value.clone())];
            let left_list = Packet::List(packet);

            compare_values(&left_list, right_list)
        }
    }
}
