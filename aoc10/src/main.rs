use std::fs::read_to_string;

fn addx(reg: &mut Vec<i32>, value: i32) {
    reg.push(reg[reg.len() - 1]);
    reg.push(reg[reg.len() - 2] + value);
}

fn noop(reg: &mut Vec<i32>) {
    reg.push(*reg.last().unwrap())
}

fn part1(reg: &[i32]) {
    let idx: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let somm: i32 = idx.iter().map(|i| reg[*i] * (*i as i32)).sum();
    println!("{}", somm);
}

fn part2(reg: &[i32]) {
    let row = 6;
    let col = 40;

    for r in 0..row {
        for c in 0..col {
            print!("{}", if (reg[r * col + c + 1] - (c as i32)).abs() <= 1 {"#"} else {"."});
        }
        println!();
    }
}

fn main() {
    let mut reg: Vec<i32> = vec![0, 1];

    for line in read_to_string("input.txt").unwrap().lines() {
        match &line[..4] {
            "addx" => addx(&mut reg, line[5..].parse().unwrap()),
            "noop" => noop(&mut reg),
            _ => panic!("Invalid instruction,")
        }
    }

    part1(&reg);
    part2(&reg);
}
