use aoc_helpers::{open_file_into_string};

fn main() {
    let file = open_file_into_string("input");

    println!("Part A: {}", do_day6a(&file));
    println!("Part B: {}", do_day6b(&file));
}


fn do_day6a(input: &str) -> usize {
    let mut idx = 4;
    for s in input.to_string().chars().collect::<Vec<char>>().windows(4) {
        let mut s = s.to_vec();
        s.sort();
        s.dedup();
        if s.len() < 4 {
            idx += 1;
        } else {
            break;
        }
    }
    idx
}

fn do_day6b(input: &str) -> usize {
    let mut idx = 14;
    for s in input.to_string().chars().collect::<Vec<char>>().windows(14) {
        let mut s = s.to_vec();
        s.sort();
        s.dedup();
        if s.len() < 14 {
            idx += 1;
        } else {
            break;
        }
    }
    idx
}

#[test]
fn test_day6a() {
    assert_eq!(do_day6a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(do_day6a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(do_day6a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(do_day6a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(do_day6a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_day6b() {
    assert_eq!(do_day6b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(do_day6b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(do_day6b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(do_day6b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(do_day6b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
