use aoc_helpers::{open_file_into_string, split_by_newline};

fn main() {
    let file = open_file_into_string("input");
    let input = split_by_newline(&file);

    println!("Part A: {}", do_day3a(&input));
    println!("Part B: {}", do_day3b(&input));
}

fn char_to_priority(c: &char) -> u64 {
    match c {
        'a'..='z' => {1 + *c as u64 - b'a' as u64},
        'A'..='Z' => {27 + *c as u64 - b'A' as u64},
        _ => 0
    }
}

fn do_day3a(packs: &[String]) -> u64 {
    packs.iter().fold(0, |acc, pack| {
        let (first, last) = pack.split_at(pack.len()/2);
        acc + first.chars().find_map(|c| {
            last.chars().find_map(|m| if m == c {Some(char_to_priority(&c))} else {None})
        }).expect("No match found")
    })
}

fn do_day3b(packs: &[String]) -> u64 {
    packs.chunks(3).fold(0, |acc, group| {
        let mut a: Vec<char> = group[0].chars().collect();
        let mut b: Vec<char> = group[1].chars().collect();
        let mut c: Vec<char> = group[2].chars().collect();

        a.sort();
        a.dedup();
        b.sort();
        b.dedup();
        c.sort();
        c.dedup();
        let badge = find_triple_match(&a, &b, &c);
        acc + char_to_priority(&badge)
    })
}

fn find_triple_match(a: &[char], b: &[char], c: &[char]) -> char {
    for ch_a in a {
        for ch_b in b {
            if ch_a == ch_b {
                for ch_c in c {
                    if ch_c == ch_b {
                        return *ch_c;
                    }
                }
            }
        }
    }
    ' '
}

#[test]
fn test3_a() {
    let input: Vec<String> = vec![
    "vJrwpWtwJgWrhcsFMMfFFhFp",
    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    "PmmdzqPrVvPwwTWBwg",
    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    "ttgJtRGJQctTZtZT",
    "CrZsJsPPZsGzwwsLwLmpwMDw",
    ].iter().map(|s| s.to_string()).collect();

    assert_eq!(do_day3a(&input), 157);
}

#[test]
fn test3_b() {
    let input: Vec<String> = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ].iter().map(|s| s.to_string()).collect();

    assert_eq!(do_day3b(&input), 70);
}
