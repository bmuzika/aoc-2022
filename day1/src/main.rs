use aoc_helpers::{open_file_into_string, string_into_vec_of_uint};

fn main() {
    let file = open_file_into_string("input");
    let input: Vec<Vec<u64>> = file
        .split("\n\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(string_into_vec_of_uint)
        .collect();

    println!("Part A: {}", do_day1a(&input));
    println!("Part B: {}", do_day1b(&input));
}

fn find_food_carried_by_elves(elves: &[Vec<u64>]) -> Vec<u64> {
    elves.iter().map(|items| items.iter().sum()).collect()
}

fn do_day1a(elves: &[Vec<u64>]) -> u64 {
    find_food_carried_by_elves(elves)
        .into_iter()
        .max()
        .unwrap_or(0)
}

fn do_day1b(elves: &[Vec<u64>]) -> u64 {
    let mut food = find_food_carried_by_elves(elves);
    food.sort();
    food.reverse();

    food[0] + food[1] + food[2]
}

#[test]
fn test1_a() {
    let input = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    assert_eq!(do_day1a(&input), 24000);
}

#[test]
fn test1_b() {
    let input = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    assert_eq!(do_day1b(&input), 45000);
}
