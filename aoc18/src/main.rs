use std::fs::read_to_string;

const N: usize = 22;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

fn covered_faces(grid: &[[[u8; N]; N]; N], cubes: &[Cube]) -> usize {
    let mut covered = 0;
    for &c in cubes {
        assert!(grid[c.x + 1][c.y + 1][c.z + 1] == 1);
        covered += ((grid[c.x][c.y + 1][c.z + 1] == 1) as usize)
            + ((grid[c.x + 2][c.y + 1][c.z + 1] == 1) as usize)
            + ((grid[c.x + 1][c.y][c.z + 1] == 1) as usize)
            + ((grid[c.x + 1][c.y + 2][c.z + 1] == 1) as usize)
            + ((grid[c.x + 1][c.y + 1][c.z] == 1) as usize)
            + ((grid[c.x + 1][c.y + 1][c.z + 2] == 1) as usize);
    }
    covered
}

fn flood_fill(grid: &mut [[[u8; N]; N]; N]) -> bool {
    let mut change: bool = false;

    grid[1][1][1] = 2;
    for x in 1..N - 1 {
        for y in 1..N - 1 {
            for z in 1..N - 1 {
                if grid[x][y][z] == 0
                    && (grid[x - 1][y][z] == 2
                        || grid[x + 1][y][z] == 2
                        || grid[x][y - 1][z] == 2
                        || grid[x][y + 1][z] == 2
                        || grid[x][y][z - 1] == 2
                        || grid[x][y][z + 1] == 2)
                {
                    grid[x][y][z] = 2;
                    change = true;
                }
            }
        }
    }
    change
}

fn fill_interior(grid: &mut [[[u8; N]; N]; N], cubes: &mut Vec<Cube>) {
    for x in 1..N - 1 {
        for y in 1..N - 1 {
            for z in 1..N - 1 {
                if grid[x][y][z] == 0 {
                    grid[x][y][z] = 1;
                    cubes.push(Cube {
                        x: x - 1,
                        y: y - 1,
                        z: z - 1,
                    });
                }
            }
        }
    }
}

fn main() {
    let mut cubes: Vec<Cube> = Vec::new();
    let mut grid: [[[u8; N]; N]; N] = [[[0; N]; N]; N];

    for line in read_to_string("input.txt").unwrap().lines() {
        let tokens: Vec<usize> = line.split(',').map(|t| t.parse().unwrap()).collect();
        cubes.push(Cube {
            x: tokens[0],
            y: tokens[1],
            z: tokens[2],
        });
    }

    for &c in &cubes {
        grid[c.x + 1][c.y + 1][c.z + 1] = 1;
    }

    println!("{}", 6 * cubes.len() - covered_faces(&grid, &cubes));

    while flood_fill(&mut grid) {}
    fill_interior(&mut grid, &mut cubes);

    println!("{}", 6 * cubes.len() - covered_faces(&grid, &cubes));
}
