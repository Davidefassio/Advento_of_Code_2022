use std::fs::read_to_string;

struct Dock {
    data: Vec<Vec<u8>>,
}

impl Dock {
    fn new(nvec: usize) -> Dock {
        let mut vecs = Vec::new();
        for _ in 0..nvec {
            vecs.push(Vec::new());
        }

        Dock {
            data: vecs,
        }
    }

    fn insert(&mut self, id: usize, val: u8) {
        self.data[id].insert(0, val);
    }

    fn move_once(&mut self, s: usize, e: usize) {
        let val = self.data[s].pop().unwrap();
        self.data[e].push(val);
    }

    fn move_n (&mut self, s: usize, e: usize, n: usize) {
        for _ in 0..n {
            self.move_once(s, e);
        }
    }

    fn move_n_inorder(&mut self, s: usize, e: usize, n: usize) {
        let ls = self.data[s].len();
        let ss = ls - n;
        let mut slice = Vec::from(&self.data[s][ss..ls]);
        self.data[e].append(&mut slice);

        self.data[s].truncate(ss);
    }

    fn get_tops(&self) -> String {
        let mut retval = String::new();

        for i in 0..self.data.len() {
            let l = self.data[i].len();
            retval.push(self.data[i][l - 1] as char);
        }

        retval
    }
}

#[derive(Debug, Copy, Clone)]
enum Part {
    One,
    Two,
}

fn main() {
    let part = Part::Two;

    let mut dock: Dock = Dock::new(0);
    let mut dock_init = false;

    for line in read_to_string("input.txt").unwrap().lines() {
        let chs = line.as_bytes();

        if chs.is_empty() {
            continue;
        }

        if chs[0] == b'[' {
            if !dock_init {
                dock = Dock::new((chs.len() + 1) / 4);
                dock_init = true;
            }
            
            let mut i = 1;
            while i < chs.len() {
                if chs[i] != b' ' {
                    dock.insert((i - 1) / 4, chs[i]);
                }

                i += 4;
            }
        }
        else if chs[0] == b' ' {
            continue;
        }
        else if chs[0] == b'm' {
            let mut vstr: Vec<String> = Vec::new();
            let mut v: Vec<usize> = Vec::new();
            let mut flag = false;
            let mut cnt = 0;

            for c in chs {
                if *c > 47 && *c < 58 {
                    if !flag {
                        vstr.push(String::new());
                    }
                    vstr[cnt].push(*c as char);

                    flag = true;
                }
                else {
                    if flag {
                        v.push(vstr[cnt].parse::<usize>().unwrap());
                        cnt += 1;
                    }
                    
                    flag = false;
                }
            }
            v.push(vstr[cnt].parse::<usize>().unwrap());

            match part {
                Part::One => dock.move_n(v[1] - 1, v[2] - 1, v[0]),
                Part::Two => dock.move_n_inorder(v[1] - 1, v[2] - 1, v[0]),
            }
        }
        else {
            unreachable!("All cases should be coverde!");
        }
    }

    println!("{}", dock.get_tops());

}
