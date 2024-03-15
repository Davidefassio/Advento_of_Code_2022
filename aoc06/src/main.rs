use std::fs::read_to_string;
use std::collections::HashSet;

// Standard hash set
fn main() {
    let header_size = 4;  // Part1 = 4, Part2 = 14
    let mut set: HashSet<u8> = HashSet::new();

    let line = read_to_string("input.txt").unwrap();
    let lineb = line.as_bytes();
    
    for i in (header_size - 1)..lineb.len() {
        for x in 0..header_size {
            set.insert(lineb[i - x]);
        }

        if set.len() == header_size {
            println!("{}", i + 1);
            break;
        }

        set.clear();
    }
}

/*
// Custom set, implemented using an array
fn main() {
    let header_size = 14;  // Part1 = 4, Part2 = 14
    let mut arrmap: [u8; 26] = [0; 26];

    let line = read_to_string("input.txt").unwrap();
    let lineb = line.as_bytes();
    
    for i in (header_size - 1)..lineb.len() {
        for x in 0..header_size {
            arrmap[(lineb[i - x] - 97) as usize] += 1;
        }

        if *(arrmap.iter().max().unwrap()) < 2 {
            println!("{}", i + 1);
            break;
        }

        arrmap.fill(0);
    }
}
*/
