use chrono::{Datelike, Utc};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};

/// This script is made to generate folder for each day of Advent of Code.
/// Examples: day02, day03, day04, etc.
pub fn run(day: Option<u8>) {
    let day = match day {
        Some(day) => day as u8,
        None => ask_day_input(),
    };

    check_folder_already_exists(day);
    copy_template(day);
    import_module_on_main(day);
    use_crate_on_puzzle(day);
    add_arm_on_match_puzzle(day);
    update_config_file(day);

    println!("Folder src/day{:02} successfully generated!", day);
}

fn ask_day_input() -> u8 {
    let now = Utc::now();
    let current_day = now.day();

    let day = Input::<u32>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the day")
        .default(current_day)
        .interact()
        .unwrap();

    day as u8
}

fn check_folder_already_exists(day: u8) {
    let path = format!("src/day{:02}", day);
    let path = Path::new(&path);

    if path.exists() {
        println!("Folder src/day{:02} already exists!", day);
        process::exit(1);
    }
}

fn copy_template(day: u8) {
    let src = format!("template");
    let dist = format!("src/day{:02}", day);

    copy_dir_all(src, dist).unwrap();
}

fn import_module_on_main(day: u8) {
    let main_path = Path::new("src/main.rs");
    let mut main_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(main_path)
        .unwrap();

    let main_content = format!("\nmod day{:02};", day);
    main_file.write_all(main_content.as_bytes()).unwrap();
}

fn use_crate_on_puzzle(day: u8) {
    let puzzle_path = Path::new("src/puzzle.rs");
    let mut puzzle_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(puzzle_path)
        .unwrap();

    let puzzle_content = format!("\nuse crate::day{:02}::run as day{:02};", day, day);
    puzzle_file.write_all(puzzle_content.as_bytes()).unwrap();
}

fn add_arm_on_match_puzzle(day: u8) {
    let puzzle_path = Path::new("src/puzzle.rs");
    let puzzle_content = fs::read_to_string(puzzle_path).unwrap();

    let original_text = r#"_ => panic!("Invalid day number. Did you forget to generate this day using the script?"),"#;
    let replaced_text = format!("{} => day{:02}(),\n\t\t{}", day, day, original_text);
    let puzzle_content = str::replace(&puzzle_content, original_text, &replaced_text);

    let mut puzzle_file = File::create(puzzle_path).unwrap();
    puzzle_file.write_all(puzzle_content.as_bytes()).unwrap();
}

fn update_config_file(day: u8) {
    let config_path = Path::new(".cargo/config.toml");
    let config_content = fs::read_to_string(config_path).unwrap();

    // update day number
    let regex = Regex::new(r#"DAY = "\d+""#).unwrap();
    let original_text = regex.find(&config_content).unwrap().as_str();
    let replaced_text = format!("DAY = \"{}\"", day);
    let config_content = str::replace(&config_content, original_text, &replaced_text);

    // update part number
    let regex = Regex::new(r#"PART = "\d+""#).unwrap();
    let original_text = regex.find(&config_content).unwrap().as_str();
    let replaced_text = "PART = \"1\"";
    let config_content = str::replace(&config_content, original_text, &replaced_text);

    // update use sample
    let regex = Regex::new(r#"USE_SAMPLE = "(?:true|false)""#).unwrap();
    let original_text = regex.find(&config_content).unwrap().as_str();
    let replaced_text = "USE_SAMPLE = \"true\"";
    let config_content = str::replace(&config_content, original_text, &replaced_text);

    let mut config_file = File::create(config_path).unwrap();
    config_file.write_all(config_content.as_bytes()).unwrap();
}

fn copy_dir_all(src: impl AsRef<Path>, dist: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dist)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dist.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dist.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
