pub mod part1;

pub fn run() {
    let day = env!("DAY").parse::<u8>().unwrap();
    let part = env!("PART").parse::<u8>().unwrap();
    let use_sample = env!("USE_SAMPLE").parse::<bool>().unwrap();

    let input = if use_sample {
        include_str!("sample.txt")
    } else {
        include_str!("input.txt")
    };

    match part {
        1 => {
            display_info(day, part, use_sample);
            part1::run(input);
        }
        _ => panic!("Invalid part number"),
    }
}

fn display_info(day: u8, part: u8, use_sample: bool) {
    println!("====  Day {}  ====", day);
    println!("====  Part {}  ====", part);

    if use_sample {
        println!(r"/!\ Sample data /!\");
    }
}
