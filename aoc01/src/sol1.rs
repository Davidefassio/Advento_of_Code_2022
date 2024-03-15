use std::fs;

fn file_to_lines(file: String) -> Vec<String> {
    let mut retval: Vec<String> = Vec::new();
    let mut tmp = String::new();

    for c in file.chars() {
        if c == '\n' {
            retval.push(tmp.clone());
            tmp.clear();
        }
        else {
            tmp.push(c);
        }
    }

    retval.push(tmp.clone());
    retval
}

fn blocks_to_vec(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut retval: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();

    for l in lines {
        if l.is_empty() {
            retval.push(tmp.clone());
            tmp.clear();
        }
        else {
            tmp.push(l.parse().unwrap());
        }
    }

    retval.push(tmp.clone());
    retval
}

fn sum_vecs(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    vecs.iter().map(|x| x.iter().sum()).collect()
}

pub fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("File not found");

    let lines = file_to_lines(input.clone());
    let vecs = blocks_to_vec(lines);

    let mut sums = sum_vecs(vecs);
    sums.sort();

    let max = sums.last().unwrap();
    println!("{}", max);

    let n = 3;
    let top_n: i32 = sums.iter().skip(sums.len() - n).sum();
    println!("{}", top_n);
}