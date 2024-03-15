use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut tmp = 0;

    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            result.push(tmp);
            tmp = 0;
        }
        else {
            tmp += line.to_string().parse::<i32>().unwrap();
        }
    }
    result.push(tmp);
    result
}


pub fn main() {
    let mut sums = read_lines("input.txt");
    sums.sort();

    let max = sums.last().unwrap();
    println!("{}", max);

    let n = 3;
    let top_n: i32 = sums.iter().skip(sums.len() - n).sum();
    println!("{}", top_n);
}