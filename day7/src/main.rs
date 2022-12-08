use std::collections::HashMap;
use aoc_helpers::{open_file_into_string};
use crate::Command::ChangeDirectory;
use crate::FsItem::{Directory, File};

fn main() {
    let file = open_file_into_string("input");

    println!("Part A: {}", do_day7a(&file));
    println!("Part B: {}", do_day7b(&file));
}

#[derive(Debug)]
enum FsItem {
    Directory(HashMap<String, FsItem>),
    File(usize),
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List
}

fn string_to_command(input: &str) -> Command {
    let sp: Vec<&str> = input.split(' ').collect();
    match sp[0] {
        "cd" => ChangeDirectory(sp[1].to_string()),
        "ls" => Command::List,
        _ => panic!("Should only have cd and ls"),
    }
}

fn string_to_item(input: &str) -> (String, FsItem) {
    let sp: Vec<&str> = input.split(' ').collect();

    match sp[0].trim() {
        "dir" => (sp[1].trim().to_string(), Directory(HashMap::new())),
        _ => (sp[1].trim().to_string(), File(sp[0].parse().unwrap())),
    }
}

fn prepare_filesystem(input: &str) -> HashMap<String, FsItem> {
    let io: Vec<(Command, Option<Vec<String>>)> = input.split('$').filter_map(|s| {
        let inner: Vec<String> = s.split('\n').filter_map(|s| if !s.is_empty() {Some(s.trim().to_string()) }else{None}).collect();
        match inner.len() {
            0 => None,
            1 => Some ((string_to_command(&inner[0]), None)),
            _ => Some((string_to_command(&inner[0]), Some(inner[1..].to_vec()))),
        }
    }).collect();

    let mut file_system: HashMap<String, FsItem> = HashMap::new();
    let mut current_directory = "".to_string();
    for (input, output) in io {
        match input {
            ChangeDirectory(dir) => {
                if &dir == ".." {
                    let path: Vec<String> = current_directory.split('/').filter_map(|s| if !s.is_empty() {Some(s.to_string())} else{None}).collect();
                    current_directory = path[0..path.len()-1].iter().fold("/".to_string(), |acc, s| format!("{}{}/", acc, s));
                } else if &dir == "/" {
                    current_directory = "/".to_string();
                } else {
                    current_directory = format!("{}{}/", current_directory, dir);
                }
            }
            Command::List => {
                let items: Vec<(String, FsItem)> = output.unwrap().iter().map(|s| string_to_item(s)).collect();
                let mut cursor = &mut file_system;
                let  path_iter = current_directory.split('/').filter_map(|s| if !s.is_empty() {Some(s.to_string())} else{None});
                for p in path_iter {
                    cursor = match cursor.get_mut(&p).unwrap() {
                        Directory(d) => { d }
                        File(_) => { panic!("Huh?");}
                    };
                };


                for i in items {
                    cursor.insert(i.0, i.1);
                }
            }
        }
    }
    file_system
}


fn do_day7a(input: &str) -> usize {
    let file_system = prepare_filesystem(input);
    let mut global_accum = 0;
    calculate_dir_size(&file_system, &mut global_accum);
    global_accum
}

fn calculate_dir_size(input_dir: &HashMap<String, FsItem>, global_accum: &mut usize) -> usize {
    let s = input_dir.iter().fold(0, |acc, item| acc + match item.1 {
        Directory(dir) => calculate_dir_size(dir, global_accum),
        File(size) => *size,
    });
    if s < 100_000 {
        *global_accum += s;
    }
    s
}

fn calculate_dir_size_partb(input_dir: &HashMap<String, FsItem>, dir_sizes: &mut Vec<usize>) -> usize {
    let s = input_dir.iter().fold(0, |acc, item| acc + match item.1 {
        Directory(dir) => calculate_dir_size_partb(dir, dir_sizes),
        File(size) => *size,
    });
    dir_sizes.push(s);
    s
}

fn do_day7b(input: &str) -> usize {
    let file_system = prepare_filesystem(input);
    let mut candidates = vec![];
    let tot = calculate_dir_size_partb(&file_system, &mut candidates);

    let total_space = 70_000_000;
    let free_space = total_space - tot;

    candidates.sort();
    for candidate in candidates {
        if free_space + candidate > 30_000_000 {
            return candidate;
        }
    }
    0
}

#[test]
fn test_day7a() {
    let input = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    assert_eq!(do_day7a(input), 95437);
}

#[test]
fn test_day7b() {
    let input = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    assert_eq!(do_day7b(input), 24933642);
}
