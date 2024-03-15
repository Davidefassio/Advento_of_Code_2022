struct Node {
    name: String,
    value: i64,
    childs: Vec<usize>,
    father: Option<usize>,
}

impl Node {
    fn new(name: &str, father: Option<usize>) -> Node {
        Node {
            value: 0,
            name: String::from(name),
            childs: Vec::new(),
            father,
        }
    }

    fn add_value(&mut self, val: i64) {
        self.value += val;
    }
}

pub struct SumTree {
    arena: Vec<Node>,
    head: usize,
    curr: usize,
}

impl SumTree {
    pub fn new() -> SumTree {
        let mut a = Vec::new();
        a.push(Node::new("/", None));
        SumTree {
            arena: a,
            head: 0,
            curr: 0,
        }
    }

    pub fn goto_root(&mut self) {
        self.curr = self.head;
    }

    pub fn goto_father(&mut self) {
        if let Some(f) = self.arena[self.curr].father {
            self.curr = f;
        }
    }

    pub fn goto_child(&mut self, name: &str) {
        for c in &self.arena[self.curr].childs {
            if self.arena[*c].name == name {
                self.curr = *c;
                return;
            }
        }
    }

    pub fn add_value_rec(&mut self, value: i64) {
        self.arena[self.curr].add_value(value);

        let mut cf = self.curr;
        while let Some(f) = self.arena[cf].father {
            cf = f;
            self.arena[cf].add_value(value);
        }
    }

    pub fn make_child(&mut self, name: &str) {
        let l = self.arena.len();
        self.arena.push(Node::new(name, Some(self.curr)));
        self.arena[self.curr].childs.push(l);
    }

    pub fn get_result_part1(&self) -> i64 {
        let mut retval = 0;
        for n in &self.arena {
            if n.value < 100000 {
                retval += n.value;
            }
        }
        retval
    }

    pub fn get_result_part2(&self) -> i64 {
        let total_size: i64 = 70000000;
        let update: i64 = 30000000;
        let free = total_size - self.arena[0].value;
        let needed = update - free;
        let mut min: i64 = 100000000;

        for n in &self.arena {
            if n.value > needed  && n.value < min{
                min = n.value;
            }
        }
        min
    }
}
