use std::collections::BTreeMap;

#[derive(Debug)]
struct File {
    size: u32,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();

    let mut current_dir_name = String::from("");

    const ROOT_DIRECTORY_NAME: &str = "/";

    directories.insert(String::from(ROOT_DIRECTORY_NAME), vec![]);

    for line in lines {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        let is_command = args[0] == "$";

        match is_command {
            true => {
                let command_name = args[1];

                if command_name == "cd" {
                    if args[2] == "/" {
                        current_dir_name = String::from(ROOT_DIRECTORY_NAME);
                    } else if args[2] == ".." {
                        current_dir_name = current_dir_name
                            .split("/")
                            .take(current_dir_name.split("/").count() - 1)
                            .collect::<Vec<&str>>()
                            .join("/");

                        if current_dir_name.is_empty() {
                            current_dir_name = String::from(ROOT_DIRECTORY_NAME);
                        }
                    } else {
                        if current_dir_name.ends_with("/") {
                            current_dir_name += args[2];
                        } else {
                            current_dir_name += format!("/{}", args[2]).as_str();
                        }
                    }
                }
            }
            false => {
                if args[0] == "dir" {
                    let name = args[1].to_string();

                    let mut dir_name = current_dir_name.clone();
                    if dir_name == "/" {
                        dir_name = dir_name + &name;
                    } else {
                        dir_name = current_dir_name.clone() + "/" + &name;
                    }

                    directories.insert(dir_name, vec![]);
                } else {
                    let size = args[0].parse::<u32>().unwrap();

                    directories
                        .entry(current_dir_name.clone())
                        .or_default()
                        .push(File { size });
                }
            }
        }
    }

    let directories_total_sizes = directories.into_iter().map(|(dir_name, files)| {
        let total_size = files.iter().map(|file| file.size).sum::<u32>();

        (dir_name, total_size)
    });
    let directories_total_sizes = BTreeMap::from_iter(directories_total_sizes);

    let directories_with_nested_dirs_total_sizes = directories_total_sizes
        .iter()
        .map(|(dir_name, _)| {
            let total_size = directories_total_sizes
                .iter()
                .filter(|(nested_dir_name, _)| nested_dir_name.starts_with(&dir_name.clone()))
                .map(|(_, nested_dir_total_size)| nested_dir_total_size)
                .sum::<u32>();

            (dir_name, total_size)
        })
        .collect::<Vec<_>>();

    let total_used_space = directories_with_nested_dirs_total_sizes
        .iter()
        .find(|(dir_name, _)| *dir_name == "/")
        .unwrap()
        .1;

    let directories_with_one_dir_deleted =
        directories_with_nested_dirs_total_sizes
            .iter()
            .map(|(dir_name, total_size)| {
                let total_size_after_deletion = total_used_space - total_size;

                (dir_name, total_size, total_size_after_deletion)
            });

    const TOTAL_DISK_SPACE: u32 = 70000000;
    const TOTAL_SPACE_TO_UPDATE: u32 = 30000000;

    const FREE_SPACE_REQUIRED: u32 = TOTAL_DISK_SPACE - TOTAL_SPACE_TO_UPDATE;

    let smaller_dir_to_delete = directories_with_one_dir_deleted
        .filter(|(_, _, total_size_after_deletion)| {
            *total_size_after_deletion <= FREE_SPACE_REQUIRED
        })
        .rev()
        .nth(0)
        .unwrap();

    let smaller_space_to_free = smaller_dir_to_delete.1;

    println!("{}", smaller_space_to_free)
}
