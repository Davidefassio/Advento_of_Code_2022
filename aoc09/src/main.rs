use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Direction { U, D, L, R, }

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
        }
    }

    fn follow(&mut self, forw: Pos) {
        if (self.x - forw.x).abs() <= 1 && (self.y - forw.y).abs() <= 1 {
            return;  // No need to move
        }
        if self.x != forw.x {
            self.x += if self.x > forw.x { -1 } else { 1 };
        }
        if self.y != forw.y {
            self.y += if self.y > forw.y { -1 } else { 1 };
        }
    }
}

fn tail_visits(snake_len: usize) -> usize {
    let mut snake: Vec<Pos> = vec![Pos::default(); snake_len];
    let mut visits: HashSet<Pos> = HashSet::new();
    visits.insert(Pos::default());

    for line in read_to_string("input.txt").unwrap().lines() {
        let dir = match line.chars().next().unwrap() {
            'U' => Direction::U,
            'D' => Direction::D,
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Unknown direction!"),  
        };

        let steps: i32 = line[2..].parse().unwrap();

        for _ in 0..steps {
            snake[0].mov(dir);
            for i in 1..snake_len {
                let prev = snake[i - 1];
                snake[i].follow(prev);
            }
            visits.insert(*snake.last().unwrap());
        }
    }

    visits.len()
}

fn main() {
    println!("{}", tail_visits(2));
    println!("{}", tail_visits(10));
}
