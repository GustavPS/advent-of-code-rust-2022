use std::cell::RefCell;
use std::char::decode_utf16;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::{io, usize};
use std::io::BufRead;
use std::mem::discriminant;
use std::path::Path;
use std::rc::{Rc, Weak};
use regex::Regex;
use crate::command::{Command, COMMAND_STARTING_CHARACTER, handle_command};
use crate::directory::Directory;
use crate::file_obj::FileObj;
use crate::file_system::FileSystem;

mod directory;
mod command;
mod file_obj;
mod file_system;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day7/src/data.txt") {
        let root = Directory::new("/");
        let mut system = FileSystem::new(root, 70000000);

        let mut current_path = "".to_string();

        for line in lines {
            if let Ok(line) = line {
                if line.starts_with(COMMAND_STARTING_CHARACTER) {
                    match handle_command(&line) {
                        Command::Cd(path) => {
                            if !path.contains('/') && !current_path.ends_with('/') {
                                current_path += "/";
                            }
                            current_path += &path;
                            system.create_directory(&current_path);
                        },
                        Command::GoBack => {
                            let splitted = current_path.rsplit_once('/').unwrap();
                            current_path = splitted.0.to_string();
                        }
                        Command::Ls => {
                        }
                        Command::Unknown => {}
                    }
                } else {
                    // Assume handling LS
                    let re = Regex::new(r"(?<size>\d+) (?<name>.+)").unwrap();
                    let Some(caps) = re.captures(&line) else {
                        // Should be a dir if we couldn't parse it
                        continue;
                    };
                    let size = caps["size"].parse::<usize>().unwrap();
                    let name = &caps["name"];
                    system.add_file(&current_path, name, size);
                }
            }
        }

        let result = get_total_sizes_below_amount(&system, 100000);
        let folder_to_delete = find_folder_to_delete_to_fit_file(&system, 30000000).unwrap();
        println!("result: {}", result);
        println!("unused space: {}", system.unused_space());
        println!("Folder to delete: {}, size: {}", folder_to_delete.name, folder_to_delete.get_size());
    }
}

fn find_folder_to_delete_to_fit_file(system: &FileSystem, file_size: usize) -> Option<&Directory> {
    let size_to_delete = i32::abs(system.unused_space() as i32 - file_size as i32) as usize;
    let folders = system.get_all_folders();
    if folders.len() == 0 {
        return None;
    }
    let mut candidate = folders.get(0).unwrap().clone();
    let mut candidate_size = candidate.get_size();

    for folder in folders {
        let folder_size = folder.get_size();
        if folder_size >= size_to_delete && folder_size < candidate_size {
            candidate = &folder;
            candidate_size = folder_size;
        }
    }

    Some(candidate)
}

fn get_total_sizes_below_amount(system: &FileSystem, amount: usize) -> usize {
    let mut result = 0;
    let folders = system.get_all_folders();

    for folder in folders {
        let folder_size = folder.get_size();
        if folder_size < amount {
            result += folder_size;
        }
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
