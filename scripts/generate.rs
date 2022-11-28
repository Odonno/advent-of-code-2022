/*
This script is made to generate folder for each day of Advent of Code.
Examples: day02, day03, day04, etc.

Install "rust-script" package in order to execute this script.

cargo install rust-script

Then run this script with:

rust-script scripts/generate.rs
*/
use std::io::{self, Write};
use std::{fs, path::Path};

const DAY: usize = 3;

fn main() {
    // copy folder template to src/dayXX
    let src = format!("template");
    let dist = format!("src/day{:02}", DAY);
    copy_dir_all(src, dist).unwrap();

    // add mod to src/main.rs
    let main_path = Path::new("src/main.rs");
    let mut main_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(main_path)
        .unwrap();
    let main_content = format!("mod day{:02};", DAY);
    main_file.write_all(main_content.as_bytes()).unwrap();

    // use create on src/import.rs file
    let import_path = Path::new("src/import.rs");
    let mut import_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(import_path)
        .unwrap();
    let import_content = format!("use crate::day{:02}::run as day{:02};", DAY, DAY);
    import_file.write_all(import_content.as_bytes()).unwrap();

    // inform user to add arm on src/import.rs file
    println!("Add the following arm to src/import.rs file:");
    println!("{} => day{:02}(part),", DAY, DAY);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
