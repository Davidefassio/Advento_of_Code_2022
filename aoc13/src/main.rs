use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum NestedList {
    Number(i32),
    List(Vec<NestedList>),
}

impl NestedList {
    fn from_string(slc: &str) -> Self {
        let bytes = slc.as_bytes();
        Self::_from_str_rec(&slc[1..], &bytes[1..]).0
    }

    fn _from_str_rec(slc: &str, vu8: &[u8]) -> (Self, usize) {
        let mut s = 0;
        let mut i = s;
        let mut retval: Vec<NestedList> = Vec::new();

        while i < vu8.len() {
            if vu8[i] == b',' || vu8[i] == b']' {
                if s < i {
                    retval.push(Self::Number(slc[s..i].parse().unwrap()));
                }
                if vu8[i] == b']' {
                    break;
                }
                s = i + 1;
            }
            else if vu8[i] == b'[' {
                let (nl, idx) = Self::_from_str_rec(&slc[i + 1..], &vu8[i + 1..]);
                
                retval.push(nl);
                i += idx;
                s = i + 1;
            }
            i += 1;
        }

        (Self::List(retval), i + 1)
    }
}

impl PartialEq for NestedList {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::Number(a), Self::List(b)) => {
                let nl_tmp = &vec![Self::Number(*a)];
                nl_tmp == b
            },
            (Self::List(a), Self::Number(b)) => {
                let nl_tmp = &vec![Self::Number(*b)];
                a == nl_tmp
            },
            (Self::List(a), Self::List(b)) => {
                if a.len() == b.len() {
                    for (ea, eb) in a.iter().zip(b.iter()) {
                        if ea != eb {
                            return false;
                        }
                    }
                    true
                }
                else {
                    false
                }

            },
        }
    }
}

impl Eq for NestedList {}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::Number(a), Self::List(b)) => {
                let nl_tmp = &vec![Self::Number(*a)];
                nl_tmp.cmp(b)
            },
            (Self::List(a), Self::Number(b)) => {
                let nl_tmp = &vec![Self::Number(*b)];
                a.cmp(nl_tmp)
            },
            (Self::List(a), Self::List(b)) => {
                for i in 0..std::cmp::min(a.len(), b.len()) {                    
                    if a[i] != b[i] {
                        return a[i].cmp(&b[i]);
                    }
                }
                a.len().cmp(&b.len())
            },
        }
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut coll: Vec<NestedList> = Vec::new();
    let mut cnt = 0;

    for (n, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        if n % 3 == 0 {
            coll.push(NestedList::from_string(line));
        }
        else if n % 3 == 1 {
            coll.push(NestedList::from_string(line));
            
            if coll[coll.len() - 2] < coll[coll.len() - 1] {
                cnt += (n / 3) + 1;
            }
        }
    }

    println!("{}", cnt);

    let i1 = coll.iter().map(|p| if p < &NestedList::from_string("[[2]]") {1} else {0}).sum::<usize>() + 1;
    let i2 = coll.iter().map(|p| if p < &NestedList::from_string("[[6]]") {1} else {0}).sum::<usize>() + 2;

    println!("{}", i1 * i2);
}
