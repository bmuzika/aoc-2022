use aoc_helpers::{open_file_into_string, split_by_newline};

fn main() {
    let file = open_file_into_string("input");
    let input: Vec<String> = split_by_newline(&file);

    println!("Part A: {}", do_day4a(&input));
    println!("Part B: {}", do_day4b(&input));
}

fn str_to_tuple(s: &str) -> (u64, u64) {
    let a: Vec<u64> = s.split('-').map(|a| a.parse().expect("")).take(2).collect();
    (a[0], a[1])
}

fn do_day4a(elf_pairs: &[String]) -> u64 {
    elf_pairs
        .iter()
        .map(|s| s.split(',').map(str_to_tuple).collect::<Vec<(u64, u64)>>())
        .fold(0, |acc, elf_pair| {
            let elf1 = elf_pair[0];
            let elf2 = elf_pair[1];

            if (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1) {
                1 + acc
            } else {
                acc
            }
        })
}

fn do_day4b(elf_pairs: &[String]) -> u64 {
    elf_pairs
        .iter()
        .map(|s| s.split(',').map(str_to_tuple).collect::<Vec<(u64, u64)>>())
        .fold(0, |acc, elf_pair| {
            let elf1 = elf_pair[0];
            let elf2 = elf_pair[1];

            if (elf1.0 <= elf2.0 && elf1.1 >= elf2.0) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.0) {
                1 + acc
            } else {
                acc
            }
        })
}

#[test]
fn test_day4a() {
    let input: Vec<String> = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(do_day4a(&input), 2);
}

#[test]
fn test_day4b() {
    let input: Vec<String> = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(do_day4b(&input), 4);
}
