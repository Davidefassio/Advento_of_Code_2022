use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn extremes(sensors: &[Point], beacons: &[Point], row: i32) -> Vec<Point> {
    let distances: Vec<i32> = sensors.iter().zip(beacons.iter()).map(|(s, b)| (s.x - b.x).abs() + (s.y - b.y).abs()).collect();

    let mut ssimp: Vec<Point> = Vec::new();
    for (&s, &d) in sensors.iter().zip(distances.iter()) {
        let a = (s.y - row).abs();
        if a <= d {
            ssimp.push(Point { x: s.x - d + a, y: s.x + d - a })
        }
    }
    ssimp
}

fn clamp_extremes(extr: Vec<Point>, min: i32, max: i32) -> Vec<Point> {
    extr.iter().map(|p| Point { x: p.x.clamp(min, max), y: p.y.clamp(min, max)}).collect()
}

fn len_blocked(extr: &[Point]) -> i32 {
    let mut sorted_segments = extr.to_owned();
    sorted_segments.sort_by_key(|&p| p.x);

    let mut total_length = 0;
    let mut last_end = i32::MIN;

    for &p in &sorted_segments {
        if p.x > last_end {
            total_length += p.y - p.x + 1;
            last_end = p.y;
        } 
        else if p.y > last_end {
            total_length += p.y - last_end;
            last_end = p.y;
        }
    }

    total_length - 1
}

fn find_hole(extr: &[Point]) -> i32 {
    let mut sorted_segments = extr.to_owned();
    sorted_segments.sort_by_key(|&p| p.x);

    let mut curr_y = 0;
    for &p in &sorted_segments {
        if p.x > curr_y + 1 {
            return curr_y + 1;
        }
        if p.y > curr_y {
            curr_y = p.y;
        }
    }
    unreachable!();
}

fn main() {
    let mut sensors: Vec<Point> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let lineb = line.as_bytes();
        let mut i = 0;
        let mut s: usize = 0;
        let mut x: i32 = 0;
                
        for (n, &c) in lineb.iter().enumerate() {
            if i % 2 == 0 {
                if c == b'=' {
                    s = n + 1;
                    i += 1;
                }
            }
            else if i == 1 || i == 5 {
                if c == b',' {
                    x = line[s..n].parse().unwrap();
                    i += 1;
                }
            }
            else {
                if c == b':' {
                    sensors.push(Point { x, y: line[s..n].parse::<i32>().unwrap() });
                    i += 1;
                }
            }
        }
        beacons.push(Point { x, y: line[s..].parse::<i32>().unwrap() });
    }
    
    println!("{}", len_blocked(&extremes(&sensors, &beacons, 2000000)));

    let mi = 0;
    let ma = 4000000;
    for i in mi..ma + 1 {
        let c = clamp_extremes(extremes(&sensors, &beacons, i), mi, ma);

        if len_blocked(&c) != ma {
            println!("{}", (find_hole(&c) as i64) * (ma as i64) + (i as i64));
            break;
        }
    }
}
