use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point{ x, y }
    }
}

fn max_y(points: &[Vec<Point>]) -> i32 {
    points.iter().map(|v| v.iter().max_by(|a, b| (a.y).cmp(&b.y)).unwrap()).max_by(|a, b| (a.y).cmp(&b.y)).unwrap().y
}

#[derive(Debug, Clone, Copy)]
enum SimRes {
    STOP,
    FALL,
    BLOCK,
}

fn fill_map(map: &mut Vec<Vec<u8>>, points: &[Vec<Point>]) {
    for v in points {
        for i in 1..v.len() {
            if v[i].x == v[i - 1].x {
                if v[i].y > v[i - 1].y {
                    for n in 0..(v[i].y - v[i - 1].y + 1) {
                        map[(v[i - 1].y + n) as usize][(v[i].x) as usize] = 1;
                    }
                }
                else {
                    for n in 0..(v[i - 1].y - v[i].y + 1) {
                        map[(v[i].y + n) as usize][(v[i].x) as usize] = 1;
                    }
                }
            }
            else {
                if v[i].x > v[i - 1].x {
                    for n in 0..(v[i].x - v[i - 1].x + 1) {
                        map[(v[i].y) as usize][(v[i - 1].x + n) as usize] = 1;
                    }
                }
                else {
                    for n in 0..(v[i - 1].x - v[i].x + 1) {
                        map[(v[i].y) as usize][(v[i].x + n) as usize] = 1;
                    }
                }
            }
        }
    }
}

fn simulate(map: &mut Vec<Vec<u8>>) -> SimRes {
    let mut sand = Point::new(500, 0);

    loop {
        if sand.y >= map.len() as i32 - 2 {
            map[(sand.y) as usize][(sand.x) as usize] = 1;
            return SimRes::FALL;
        }

        if map[(sand.y + 1) as usize][(sand.x) as usize] == 0 {  // Down
            sand.y += 1;
        }
        else if map[(sand.y + 1) as usize][(sand.x - 1) as usize] == 0 {  // DL
            sand.y += 1;
            sand.x -= 1;
        }
        else if map[(sand.y + 1) as usize][(sand.x + 1) as usize] == 0 {  // DR
            sand.y += 1;
            sand.x += 1;
        }
        else {
            if sand == Point::new(500, 0) {
                return SimRes::BLOCK;
            }

            map[(sand.y) as usize][(sand.x) as usize] = 1;
            return SimRes::STOP;
        }
    }
}

fn print_part1(n: i32, flag: bool) -> bool {
    if flag {
        println!("{}", n)
    }
    false
}

fn main() {
    let mut tmp: Vec<Vec<Point>> = Vec::new();

    for (n, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        let lineb = line.as_bytes();

        tmp.push(Vec::new());

        let mut p1: i32 = 0;
        let mut s = 0;

        for (i, &c) in lineb.iter().enumerate() {
            if c == b',' {
                p1 = line[s..i].parse().unwrap();
                s = i + 1;
            }
            else if c == b' ' {
                if lineb[i - 1] != b'>' {
                    let p2 = line[s..i].parse::<i32>().unwrap();
                    tmp[n].push(Point::new(p1, p2));
                }
                s = i + 1;
            } 
        }
        let p2 = line[s..lineb.len()].parse::<i32>().unwrap();
        tmp[n].push(Point::new(p1, p2));
    }

    let maxy = max_y(&tmp);

    tmp.push(vec![Point::new(0, maxy + 2),Point::new(1000, maxy + 2)]);

    let mut map: Vec<Vec<u8>> = vec![vec![0; 1001]; maxy as usize + 3];
    fill_map(&mut map, &tmp);

    let mut cnt = 0;
    let mut flag = true;
    loop {
        match simulate(&mut map) {
            SimRes::STOP => (),
            SimRes::FALL => flag = print_part1(cnt, flag),
            SimRes::BLOCK => break,
        }
        
        cnt += 1;
    }

    println!("{}", cnt + 1);
}
