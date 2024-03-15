use std::fs::read_to_string;

fn tokenize(line: &str) -> (i32, i32, i32, i32) {
    let mut retval: Vec<i32> = Vec::new();
    let mut start = 0;

    for (i, ch) in line.as_bytes().iter().enumerate() {
        if *ch == b'-' || *ch == b',' {
            retval.push((&line[start..i]).parse().unwrap());
            start = i + 1;
        }
    }
    retval.push((&line[start..line.len()]).parse().unwrap());

    (retval[0], retval[1], retval[2], retval[3])
}

fn main() {
    let mut cnt1 = 0;
    let mut cnt2 = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let (s1, e1, s2, e2) = tokenize(line);
        
        if (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1) {
            cnt1 += 1;
        }

        if (s1 <= s2 && e1 >= s2) || 
            (s1 <= e2 && e1 >= e2) ||
            (s2 <= s1 && e2 >= s1) ||
            (s2 <= e1 && e2 >= e1) {
            cnt2 += 1;
        }

    }

    println!("{}", cnt1);
    println!("{}", cnt2);
}
