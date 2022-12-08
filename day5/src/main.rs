use aoc_helpers::{open_file_into_string};

fn main() {
    let file = open_file_into_string("input");

    println!("Part A: {}", do_day5a(&file));
    println!("Part B: {}", do_day5b(&file));
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn prepare_stacks_and_instructions(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let num_stacks: u64 = input[0].split('\n').rev().take(1).last().unwrap().split(' ').last().unwrap().trim().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    input[0]
        .split('\n')
        .rev()
        .skip(1)
        .for_each(|row| row
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .enumerate()
            .for_each(|(idx, c)| {
                let c = c
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .to_string();
                if !c.is_empty() {
                    stacks[idx].push(c.chars().take(1).next().unwrap());
                }
            })
        );

    let instuctions: Vec<Instruction> = input[1]
        .split('\n')
        .map(|s| s
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|p| p
                .parse()
                .unwrap())
            .collect::<Vec<usize>>())
        .filter(|v| !v.is_empty())
        .map(|v| Instruction{
            quantity: v[0],
            src: v[1],
            dst: v[2]
        })
        .collect();

    (stacks, instuctions)
}

fn do_day5a(input: &str) -> String {
    let (mut stacks, instructions) = prepare_stacks_and_instructions(input);
    for inst in instructions {
        for _ in 0..inst.quantity {
            let item = stacks[inst.src - 1].pop().unwrap();
            stacks[inst.dst - 1].push(item);
        }
    }

    let mut r = vec![];
    for s in stacks {
        r.push(*s.last().unwrap());
    }
    r.iter().collect()
}

fn do_day5b(input: &str) -> String {
    let (mut stacks, instructions) = prepare_stacks_and_instructions(input);
    for inst in instructions {
        let mut r = vec![];
        for _ in 0..inst.quantity {
            r.push(stacks[inst.src - 1].pop().unwrap());
        }
        stacks[inst.dst - 1].extend(r.iter().rev());
    }

    let mut r = vec![];
    for s in stacks {
        r.push(*s.last().unwrap());
    }
    r.iter().collect()
}

#[test]
fn test_day5a() {
    let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(do_day5a(&input), "CMZ");
}

#[test]
fn test_day5b() {
    let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(do_day5b(&input), "MCD");
}
