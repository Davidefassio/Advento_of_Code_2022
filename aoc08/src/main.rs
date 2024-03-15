use std::fs::read_to_string;

fn is_visible(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> i32 {
    let mut i: i32;
    let mut vis: bool;
    // Up
    i = row as i32 - 1;
    vis = true;
    while i >= 0 {
        if grid[i as usize][col] >= grid[row][col] {
            vis = false;
            break;
        }
        i -= 1;
    }
    if vis {
        return 1;
    }
    // Down
    i = row as i32 + 1;
    vis = true;
    while i < grid.len() as i32 {
        if grid[i as usize][col] >= grid[row][col] {
            vis = false;
            break;
        }
        i += 1;
    }
    if vis {
        return 1;
    }
    // Left
    i = col as i32 - 1;
    vis = true;
    while i >= 0 {
        if grid[row][i as usize] >= grid[row][col] {
            vis = false;
            break;
        }
        i -= 1;
    }
    if vis {
        return 1;
    }
    // Right
    i = col as i32 + 1;
    vis = true;
    while i < grid[row].len() as i32 {
        if grid[row][i as usize] >= grid[row][col] {
            vis = false;
            break;
        }
        i += 1;
    }
    if vis {
        return 1;
    }

    0
}

fn outside_visible(grid: &Vec<Vec<u8>>) -> i32 {
    let mut cnt = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            cnt += is_visible(grid, r, c);
        }
    }
    cnt
}

fn scenic_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> i32 {
    let mut i: i32;
    let mut cnt: i32;
    let mut ss = 1;

    // Up
    i = row as i32 - 1;
    cnt = 0;
    while i - cnt >= 0 {
        cnt += 1;
        if grid[(i - cnt + 1) as usize][col] >= grid[row][col] {
            break;
        }
    }
    ss *= cnt;

    // Down
    i = row as i32 + 1;
    cnt = 0;
    while i + cnt < grid.len() as i32 {
        cnt += 1;
        if grid[(i + cnt - 1) as usize][col] >= grid[row][col] {
            break;
        }
    }
    ss *= cnt;

    // Left
    i = col as i32 - 1;
    cnt = 0;
    while i - cnt >= 0 {
        cnt += 1;
        if grid[row][(i - cnt + 1) as usize] >= grid[row][col] {
            break;
        }
    }
    ss *= cnt;

    // Right
    i = col as i32 + 1;
    cnt = 0;
    while i + cnt < grid[row].len() as i32 {
        cnt += 1;
        if grid[row][(i + cnt - 1) as usize] >= grid[row][col] {
            break;
        }
    }
    ss *= cnt;

    ss
}

fn max_scenic_score(grid: &Vec<Vec<u8>>) -> i32 {
    let mut max = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let ss = scenic_score(grid, r, c);
            if ss > max {
                max = ss;
            }
        }
    }
    max
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    
    for (id, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        grid.push(Vec::new());

        let lineb = line.as_bytes();

        for b in lineb {
            grid[id].push(*b - 48);
        }
    }

    println!("{}", outside_visible(&grid));
    println!("{}", max_scenic_score(&grid));
}
