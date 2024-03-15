use std::fs::read_to_string;

fn split_half(line: &str) -> (&[u8], &[u8]) {
    let (s1, s2) = line.split_at(line.len() / 2);
    (s1.as_bytes(), s2.as_bytes())
}

fn ch_to_int(c: u8) -> i32 {
    if c > 96 && c < 123 {
        (c as i32) - 96
    }
    else {
        (c as i32) - 38
    }
}

fn compute_red(line: &str) -> i32 {
    let (v1, v2) = split_half(line);

    for c1 in v1 {
        for c2 in v2 {
            if c1 == c2 {
                return ch_to_int(*c1);
            }
        }
    }

    0
}

fn part1() {
    let mut cnt = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        cnt += compute_red(line);
    }

    println!("{}", cnt);
}

fn find_shared(vecs: &[Vec<u8>]) -> i32 {
    let mut found: bool;
    for c0 in &vecs[0] {
        found = false;
        for c1 in &vecs[1] {
            if c0 == c1 {
                found = true;
                break;
            }
        }

        if found {
            for c2 in &vecs[2] {
                if c0 == c2 {
                    return ch_to_int(*c0);
                }
            }
        }
    }

    0
}

fn part2() {
    let mut cnt = 0;
    let mut three_lines: Vec<Vec<u8>> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        three_lines.push(line.as_bytes().to_vec());

        if three_lines.len() == 3 {
            cnt += find_shared(&three_lines);
            three_lines.clear();
        }
    }

    println!("{}", cnt);
}

fn main() {
    part1();
    part2();
}
