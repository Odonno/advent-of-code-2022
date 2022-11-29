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

fn main() {
    let day = ask_day_input();

    copy_template(day);
    import_module_on_main(day);
    use_crate_on_import(day);
    add_arm_on_match_import(day);

    println!("Folder src/day{:02} successfully generated!", day);
}

fn ask_day_input() -> usize {
    let mut input = String::new();
    print!("Enter the day: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap()
}

fn copy_template(day: usize) {
    let src = format!("template");
    let dist = format!("src/day{:02}", day);
    copy_dir_all(src, dist).unwrap();
}

fn import_module_on_main(day: usize) {
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

fn use_crate_on_import(day: usize) {
    let import_path = Path::new("src/import.rs");
    let mut import_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(import_path)
        .unwrap();
    let import_content = format!("\nuse crate::day{:02}::run as day{:02};", day, day);
    import_file.write_all(import_content.as_bytes()).unwrap();
}

fn add_arm_on_match_import(day: usize) {
    let import_path = Path::new("src/import.rs");
    
    let import_content = fs::read_to_string(import_path).unwrap();

    let original_text = r#"_ => panic!("Invalid day number. Did you forget to generate this day using the script?"),"#;
    let replaced_text = format!(
        "{} => day{:02}(part),\n\t\t_ => panic!(\"Invalid day number. Did you forget to generate this day using the script?\"),", 
        day, 
        day
    );
    let import_content = str::replace(&import_content, original_text, &replaced_text);

    let mut import_file = std::fs::File::create(import_path).unwrap();
    import_file.write_all(import_content.as_bytes()).unwrap();
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
