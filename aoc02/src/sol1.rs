use core::panic;
use std::fs::read_to_string;

fn ch_to_int(ch: u8) -> i32 {
    let c = ch as char;

    if c == 'A' || c == 'X' {
        0
    }
    else if  c == 'B' || c == 'Y' {
        1
    }
    else {
        2
    }
}

fn rps_score(p1: i32, p2: i32) -> i32 {
    if p1 == p2 {
        3
    }
    else if (p1 + 1) % 3 == p2 {
        6
    }
    else {
        0
    }
}

fn part1() {
    let mut cnt = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let p1 = ch_to_int(line.as_bytes()[0]);
        let p2 = ch_to_int(line.as_bytes()[2]);
        
        cnt += p2 + 1;
        cnt += rps_score(p1, p2);
    }

    println!("Score {}", cnt);
}

fn fixed_score(p1: i32, outcome: i32) -> i32 {
    let s = match outcome {
        0 => if p1 - 1 < 0 {p1 + 2} else {p1 - 1},
        1 => p1,
        2 => (p1 + 1) % 3,
        _ => panic!(),
    };

    s + 1
}

fn part2() {
    let mut cnt = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let p1 = ch_to_int(line.as_bytes()[0]);
        let outcome = ch_to_int(line.as_bytes()[2]);
        
        cnt += outcome * 3;
        cnt += fixed_score(p1, outcome);
    }

    println!("Score {}", cnt);
}

pub fn main() {
    part1();
    part2();
 }
