use crate::GameResult::{Draw, Loss, Win};
use crate::Move::{Paper, Rock, Scissors};
use aoc_helpers::open_file_into_string;

fn main() {
    let file = open_file_into_string("input");
    let input: Vec<(char, char)> = file
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let s: Vec<char> = s
                .split(' ')
                .map(|s| s.chars().collect::<Vec<char>>()[0])
                .collect();
            (s[0], s[1])
        })
        .collect();

    println!("Part A: {}", do_day2a(&input));
    println!("Part B: {}", do_day2b(&input));
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

fn char_to_move(c: char) -> Move {
    match c {
        'A' => Rock,
        'X' => Rock,
        'B' => Paper,
        'Y' => Paper,
        'C' => Scissors,
        'Z' => Scissors,
        _ => panic!("nope"),
    }
}

fn char_to_result(c: char) -> GameResult {
    match c {
        'X' => Loss,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!("nope"),
    }
}

fn do_day2a(games: &[(char, char)]) -> u64 {
    games
        .iter()
        .map(|s| (char_to_move(s.0), char_to_move(s.1)))
        .fold(0, |acc, game| {
            let shape_score = match game.1 {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };

            let result_score = match game {
                (Rock, Rock) => 3,
                (Rock, Paper) => 6,
                (Rock, Scissors) => 0,
                (Paper, Rock) => 0,
                (Paper, Paper) => 3,
                (Paper, Scissors) => 6,
                (Scissors, Rock) => 6,
                (Scissors, Paper) => 0,
                (Scissors, Scissors) => 3,
            };

            acc + (shape_score + result_score)
        })
}

fn do_day2b(games: &[(char, char)]) -> u64 {
    games
        .iter()
        .map(|s| (char_to_move(s.0), char_to_result(s.1)))
        .fold(0, |acc, game| {
            let my_move = match game {
                (Rock, Loss) => Scissors,
                (Rock, Draw) => Rock,
                (Rock, Win) => Paper,
                (Paper, Loss) => Rock,
                (Paper, Draw) => Paper,
                (Paper, Win) => Scissors,
                (Scissors, Loss) => Paper,
                (Scissors, Draw) => Scissors,
                (Scissors, Win) => Rock,
            };

            let shape_score = match my_move {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };

            let result_score = match (game.0, my_move) {
                (Rock, Rock) => 3,
                (Rock, Paper) => 6,
                (Rock, Scissors) => 0,
                (Paper, Rock) => 0,
                (Paper, Paper) => 3,
                (Paper, Scissors) => 6,
                (Scissors, Rock) => 6,
                (Scissors, Paper) => 0,
                (Scissors, Scissors) => 3,
            };

            acc + (shape_score + result_score)
        })
}

#[test]
fn test2_a() {
    let input = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];

    assert_eq!(do_day2a(&input), 15);
}

#[test]
fn test2_b() {
    let input = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];

    assert_eq!(do_day2b(&input), 12);
}
