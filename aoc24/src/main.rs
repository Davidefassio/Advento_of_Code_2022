use std::fs::read_to_string;
use std::collections::{BinaryHeap, HashSet};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

const DIRECTIONS: [(i64, i64); 5] = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

fn conv_dir(d: i64) -> (i64, i64) {
    match d {
        62 => (0, 1),  // >
        60 => (0, -1), // <
        94 => (-1, 0), // ^
        118 => (1, 0), // v
        _ => panic!("Unknown direction!"),
    }
}

fn next_blizz_grid(
    cbg: &HashSet<(i64, i64, i64)>,
    rows: i64,
    cols: i64,
) -> HashSet<(i64, i64, i64)> {
    let mut nb: HashSet<(i64, i64, i64)> = HashSet::new();

    for (r, c, dir) in cbg.iter() {
        let m = conv_dir(*dir);
        let mut r_new = *r + m.0;
        let mut c_new = *c + m.1;
        if r_new == rows - 1 {
            r_new = 1;
        };
        if r_new == 0 {
            r_new = rows - 2;
        };
        if c_new == cols - 1 {
            c_new = 1;
        };
        if c_new == 0 {
            c_new = cols - 2;
        };
        nb.insert((r_new, c_new, *dir));
    }

    nb
}

fn get_neightbors(
    grids: &Vec<HashSet<(i64, i64)>>,
    states: i64,
    r: i64,
    c: i64,
    t: i64,
    rows: i64,
    cols: i64,
) -> Vec<(i64, i64, i64)> {
    let mut nb: Vec<(i64, i64, i64)> = Vec::new();
    for (dis_r, dis_c) in DIRECTIONS {
        let r1 = r + dis_r;
        let c1 = c + dis_c;

        if (((1 <= r1 && r1 < rows - 1) && (1 <= c1 && c1 < cols - 1))
            || ((r1, c1) == (0, 1))
            || ((r1, c1) == (26, 120)))
            && !grids[(t % states) as usize].contains(&(r1, c1))
        {
            nb.push((r1, c1, t));
        }
    }

    nb
}

fn get_steps(
    start: (i64, i64),
    end: (i64, i64),
    t_start: i64,
    rows: i64,
    cols: i64,
    grids: &Vec<HashSet<(i64, i64)>>,
    states: i64,
) -> i64 {
    let mut visited = HashSet::new();
    let mut prio_queue = BinaryHeap::new();
    prio_queue.push(std::cmp::Reverse((t_start, start)));

    while let Some(std::cmp::Reverse((t, (r, c)))) = prio_queue.pop() {
        let s = t % states;
        if !visited.contains(&(r, c, s)) {
            visited.insert((r, c, s));
            if (r, c) == end {
                return t - t_start;
            }
            for (rnew, cnew, t) in get_neightbors(grids, states, r, c, t, rows, cols) {
                prio_queue.push(std::cmp::Reverse((t + 1, (rnew, cnew))));
            }
        }
    }
    0
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        grid.push(line.bytes().collect());
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let states = lcm(rows - 2, cols - 2) as i64;

    let mut curr_blizz_grid: HashSet<(i64, i64, i64)> = HashSet::new();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != b'#' && grid[r][c] != b'.' {
                curr_blizz_grid.insert((r as i64, c as i64, grid[r][c] as i64));
            }
        }
    }

    let mut grids: Vec<HashSet<(i64, i64)>> = Vec::new();
    for _ in 0..=states {
        curr_blizz_grid = next_blizz_grid(&curr_blizz_grid, rows as i64, cols as i64);
        grids.push(curr_blizz_grid.iter().map(|&(r, c, _)| (r, c)).collect());
    }

    let start = (0, 1);
    let end = (rows as i64 - 1, cols as i64 - 2);

    let mut t = get_steps(start, end, 0, rows as i64, cols as i64, &grids, states);
    println!("{}", t);

    t += get_steps(end, start, t, rows as i64, cols as i64, &grids, states);
    t += get_steps(start, end, t, rows as i64, cols as i64, &grids, states);
    println!("{}", t);
}
