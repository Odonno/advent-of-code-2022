use itertools::Itertools;

pub fn run() {
    let input = include_str!("part1.txt");

    let mut index = 0;

    const CHARS_PER_PACKET: u8 = 4;

    loop {
        let mut chars = input.chars().skip(index).take(CHARS_PER_PACKET as usize);

        if chars.all_unique() {
            break;
        }

        index += 1;
    }

    let marker_index = index + CHARS_PER_PACKET as usize;

    println!("{:?}", marker_index);
}
