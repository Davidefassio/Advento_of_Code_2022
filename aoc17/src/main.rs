use std::collections::HashMap;
use std::fs::read_to_string;

static WIDTH: usize = 7;

#[derive(Debug, Clone, Copy, Default)]
enum Dir {
    #[default]
    L,
    R,
}

impl Dir {
    fn new(d: u8) -> Self {
        match d {
            b'<' => Dir::L,
            b'>' => Dir::R,
            _ => panic!("Direction not found"),
        }
    }
}

#[derive(Debug, Clone)]
struct Piece {
    pos: Vec<usize>,
    shape: Vec<Vec<u8>>,
    side: usize,
}

impl Piece {
    fn oline() -> Piece {
        Piece {
            pos: vec![0, 0],
            shape: vec![
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            side: 4,
        }
    }

    fn cross() -> Piece {
        Piece {
            pos: vec![0, 0],
            shape: vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 0],
            ],
            side: 3,
        }
    }

    fn angle() -> Piece {
        Piece {
            pos: vec![0, 0],
            shape: vec![
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0],
            ],
            side: 3,
        }
    }

    fn vline() -> Piece {
        Piece {
            pos: vec![0, 0],
            shape: vec![
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            side: 1,
        }
    }

    fn block() -> Piece {
        Piece {
            pos: vec![0, 0],
            shape: vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            side: 2,
        }
    }

    fn reset_pos(&mut self, row: usize, col: usize) {
        self.pos[0] = row;
        self.pos[1] = col;
    }

    fn check_move_lat(&self, d: Dir) -> bool {
        match d {
            Dir::L => self.pos[1] != 0,
            Dir::R => self.pos[1] + self.side != WIDTH,
        }
    }

    fn move_lat(&mut self, d: Dir) {
        match d {
            Dir::L => self.pos[1] -= 1,
            Dir::R => self.pos[1] += 1,
        };
    }

    fn undo_move_lat(&mut self, d: Dir) {
        match d {
            Dir::L => self.pos[1] += 1,
            Dir::R => self.pos[1] -= 1,
        };
    }

    fn check_move_down(&self) -> bool {
        self.pos[0] != 0
    }

    fn check_collision(&self, grid: &[Vec<u8>]) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if self.shape[r][c] == 1 && grid[r + self.pos[0]][c + self.pos[1]] == 1 {
                    return true;
                }
            }
        }
        false
    }

    fn print_on_grid(&self, grid: &mut Vec<Vec<u8>>) {
        for r in 0..4 {
            for c in 0..4 {
                if self.shape[r][c] == 1 {
                    grid[r + self.pos[0]][c + self.pos[1]] = 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
struct State {
    heights: [usize; 7], // height of the tops relative to the highest
    piece_id: usize,
    dir_id: usize,
}

impl State {
    fn new(grid: &[Vec<u8>], highest: usize, piece_id: usize, dir_id: usize) -> State {
        let mut heights: [usize; 7] = [0; 7];
        let mut h = highest;
        let mut n = 0;

        loop {
            for i in 0..7 {
                if heights[i] == 0 && grid[h][i] == 1 {
                    heights[i] = highest - h;
                    n += 1;
                }
            }
            
            if n == 7 || h == 0 {
                break;
            }
            h -= 1;
        }

        State {
            heights,
            piece_id,
            dir_id,
        }
    }
}

fn update_heighest(grid: &[Vec<u8>], old: usize) -> usize {
    for r in old..grid.len() {
        if grid[r].iter().all(|&x| x == 0) {
            return r;
        }
    }
    panic!("Grid full!");
}

fn simulate(directions: &[Dir], pieces: &[Piece], total_p: usize) {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 7]; 6000];
    let mut highest: usize = 0;
    let mut piece_id: usize = 0;
    let mut dir_id: usize = 0;
    let mut n_pieces: usize = 0;
    let mut states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut cycle_found: bool = false;
    let mut height_gain_in_cycle: usize = 0;
    let mut skipped_cycles: usize = 0;

    while n_pieces < total_p {
        // Spawn a piece
        highest = update_heighest(&grid, highest);
        let mut curr_p = pieces[piece_id].clone();
        curr_p.reset_pos(highest + 3, 2);

        loop {
            // Apply jet direction
            if curr_p.check_move_lat(directions[dir_id]) {
                curr_p.move_lat(directions[dir_id]);
                if curr_p.check_collision(&grid) {
                    curr_p.undo_move_lat(directions[dir_id]);
                }
            }
            dir_id = (dir_id + 1) % directions.len();

            // Apply gravity
            if curr_p.check_move_down() {
                curr_p.pos[0] -= 1;
                if curr_p.check_collision(&grid) {
                    curr_p.pos[0] += 1;
                    curr_p.print_on_grid(&mut grid);
                    break;
                }
            } else {
                curr_p.print_on_grid(&mut grid);
                break;
            }
        }

        n_pieces += 1;
        highest = update_heighest(&grid, highest);

        if !cycle_found {
            let curr_state = State::new(&grid, highest, piece_id, dir_id);

            if let Some(&v) = states.get(&curr_state) {
                // Cycle found!
                height_gain_in_cycle = highest - v.0;
                let rocks_in_cycle = n_pieces - v.1;
                skipped_cycles = (total_p - n_pieces) / rocks_in_cycle;
                n_pieces += skipped_cycles * rocks_in_cycle;

                cycle_found = true;
            } else {
                // New state found
                states.insert(curr_state, (highest, n_pieces));
            }
        }

        piece_id = (piece_id + 1) % pieces.len();
    }

    println!("{}", highest + (skipped_cycles * height_gain_in_cycle));
}

fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let line = binding.lines().next().unwrap();
    let directions: Vec<Dir> = line.as_bytes().iter().map(|&d| Dir::new(d)).collect();

    let pieces: Vec<Piece> = vec![
        Piece::oline(),
        Piece::cross(),
        Piece::angle(),
        Piece::vline(),
        Piece::block(),
    ];

    simulate(&directions, &pieces, 2022);
    simulate(&directions, &pieces, 1000000000000);
}
