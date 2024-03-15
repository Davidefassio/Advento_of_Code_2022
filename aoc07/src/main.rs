pub mod sumtree;

use std::fs::read_to_string;

fn main() {
    let mut tree = sumtree::SumTree::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let lineb = line.as_bytes();

        if lineb[0] == b'$' {
            if lineb[2] == b'c' {
                // cd
                if lineb[5] == b'/' {
                    tree.goto_root();
                }
                else if lineb[5] == b'.' {
                    tree.goto_father();
                }
                else {
                    tree.goto_child(&line[5..]);
                }
            }
            else {
                // ls
            }
        }
        else if lineb[0] == b'd' {
            // dir
            tree.make_child(&line[4..]);
        }
        else {
            // file
            let ws = line.find(' ').unwrap();
            let slc = &line[..ws];
            let v: i64 = slc.parse().unwrap();
            tree.add_value_rec(v);
        }
    }

    println!("{}", tree.get_result_part1());
    println!("{}", tree.get_result_part2());
}
