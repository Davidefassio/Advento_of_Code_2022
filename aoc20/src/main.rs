use std::fs::read_to_string;

fn find_id(arr: &[(usize, i64)], id: usize) -> usize {
    for (n, i) in arr.iter().enumerate() {
        if i.0 == id {
            return n;
        }
    }
    panic!("Cannot find index.");
}

fn find_num(arr: &[(usize, i64)], num: i64) -> usize {
    for (n, i) in arr.iter().enumerate() {
        if i.1 == num {
            return n;
        }
    }
    panic!("Cannot find number.");
}

fn update_id(id: usize, d: i64, length: usize) -> (usize, bool) {
    if d > 0 && id == length - 1 {
        (0, true)
    } else if d < 0 && id == 0 {
        (length - 1, true)
    } else {
        ((id as i64 + d) as usize, false)
    }
}

fn move_id(arr: &mut [(usize, i64)], id: usize) {
    let d: i64 = if arr[id].1 >= 0 { 1 } else { -1 };
    let mut v: i64 = arr[id].1.abs() % (arr.len() as i64 - 1);

    let mut curr_id = id;
    let tmpt = update_id(curr_id, d, arr.len());
    let mut next_id = tmpt.0;
    let mut flag;

    while v > 0 {
        arr.swap(curr_id, next_id);

        curr_id = next_id;
        (next_id, flag) = update_id(curr_id, d, arr.len());
        if flag {
            if curr_id == 0 {
                arr.rotate_left(1);
                curr_id = arr.len() - 1;
            } else {
                arr.rotate_right(1);
                curr_id = 0;
            }

            (next_id, _) = update_id(curr_id, d, arr.len());
        }

        v -= 1;
    }
}

fn result(arr: &[(usize, i64)]) -> i64 {
    let id0 = find_num(arr, 0);

    arr[(id0 + 1000) % arr.len()].1
        + arr[(id0 + 2000) % arr.len()].1
        + arr[(id0 + 3000) % arr.len()].1
}

fn compute(arr: &mut [(usize, i64)], it: usize) {
    for _ in 0..it {
        for i in 0..arr.len() {
            move_id(arr, find_id(arr, i));
        }
    }

    println!("{}", result(arr));
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut work1: Vec<(usize, i64)> = file
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .enumerate()
        .map(|c| (c.0, c.1.to_owned()))
        .collect();

    let key = 811589153;
    let mut work2: Vec<(usize, i64)> = work1.iter().map(|p| (p.0, p.1 * key)).collect();

    compute(&mut work1, 1);
    compute(&mut work2, 10);
}
