use std::fs::read_to_string;

type Pos = (usize, usize);

fn follow(grid: &[Vec<i32>], visited: &mut Vec<Vec<i32>>, pos: Pos, end: Pos, steps: i32, max_s: Option<i32>) {
    if let Some(s) = max_s  { if steps >= s { return; } }  // Max step allowed reached

    visited[pos.0][pos.1] = steps;
    
    if pos == end { return; }  // Destination reached

    let new_steps = steps + 1;
    if pos.0 > 0 && grid[pos.0 - 1][pos.1] < grid[pos.0][pos.1] + 2 && visited[pos.0 - 1][pos.1] > new_steps {
            follow(grid, visited, (pos.0 - 1, pos.1), end, new_steps, max_s);  // U
    }
    if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1] < grid[pos.0][pos.1] + 2 && visited[pos.0 + 1][pos.1] > new_steps {
            follow(grid, visited, (pos.0 + 1, pos.1), end, new_steps, max_s);  // D
    }
    if pos.1 > 0 && grid[pos.0][pos.1 - 1] < grid[pos.0][pos.1] + 2 && visited[pos.0][pos.1 - 1] > new_steps {
            follow(grid, visited, (pos.0, pos.1 - 1), end, new_steps, max_s);  // L
    }
    if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1] < grid[pos.0][pos.1] + 2 && visited[pos.0][pos.1 + 1] > new_steps {
            follow(grid, visited, (pos.0, pos.1 + 1), end, new_steps, max_s);  // R
    }
}

fn part1(grid: &[Vec<i32>], start: Pos, end: Pos) {
    let mut visited: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

    follow(grid, &mut visited, start, end, 0, None);

    println!("{}", visited[end.0][end.1]);
}

fn part2(grid: &[Vec<i32>], end: Pos) {
    let row = grid.len();
    let col = grid[0].len();
    let mut visited: Vec<Vec<i32>>;
    let mut min_steps: i32 = i32::MAX;

    for i in 0..row {
        for j in 0..col {
            if grid[i][j] == 0 {
                visited = vec![vec![i32::MAX; col]; row];
                follow(grid, &mut visited, (i, j), end, 0, Some(min_steps));

                if visited[end.0][end.1] != i32::MAX && visited[end.0][end.1] < min_steps{
                    min_steps = visited[end.0][end.1];
                }
            }
        }
    }

    println!("{}", min_steps);
}

fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (cnt, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        let lineb = line.as_bytes();

        grid.push(Vec::new());

        for &c in lineb {
            if c == b'S' {
                start = (cnt, grid[cnt].len());
                grid[cnt].push(0);
            }
            else if c == b'E' {
                end = (cnt, grid[cnt].len());
                grid[cnt].push(25);
            }
            else {
                grid[cnt].push((c - 97u8) as i32);
            }
        }
    }

    part1(&grid, start, end);
    part2(&grid, end);
}
