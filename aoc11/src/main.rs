use std::fs::read_to_string;

#[derive(Debug)]
enum Operator {
    A,
    S,
    M,
    D,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op1: Option<i64>,
    op2: Option<i64>,
    op: Operator,
    test: i64,
    throw: Vec<i64>,
    business: i64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            op1: None,
            op2: None,
            op: Operator::A,
            test: 0,
            throw: Vec::new(),
            business: 0,
        }
    }

    fn set_start_items(&mut self, input: &str) {
        for n in input.split(',') {
            self.items.push(n.trim().parse().unwrap());
        }
    }

    fn set_op(&mut self, input: &str) {
        let mut tokens = input.split_whitespace();

        let t1 = tokens.next().unwrap();
        if t1 != "old" {
            self.op1 = Some(t1.parse().unwrap());
        }

        let t2 = tokens.next().unwrap();
        match t2 {
            "+" => self.op = Operator::A,
            "-" => self.op = Operator::S,
            "*" => self.op = Operator::M,
            "/" => self.op = Operator::D,
            _ => (),
        }

        let t3 = tokens.next().unwrap();
        if t3 != "old" {
            self.op2 = Some(t3.parse().unwrap());
        }
    }

    fn set_test(&mut self, input: &str) {
        self.test = input.parse().unwrap();
    }

    fn set_throw_true(&mut self, input: &str) {
        self.throw.push(input.parse().unwrap());
    }

    fn set_throw_false(&mut self, input: &str) {
        self.throw.push(input.parse().unwrap());
    }

    fn do_op(&self, lvl: i64) -> i64 {
        let a_op1 = match self.op1 {
            Some(v) => v,
            None => lvl,
        };

        let a_op2 = match self.op2 {
            Some(v) => v,
            None => lvl,
        };

        match self.op {
            Operator::A => a_op1 + a_op2,
            Operator::S => a_op1 - a_op2,
            Operator::M => a_op1 * a_op2,
            Operator::D => a_op1 / a_op2,
        }
    }

    fn do_test(&self, lvl: i64) -> usize {
        (if lvl % self.test == 0 { self.throw[0] } else { self.throw[1] }) as usize
    }
}

fn compute_monk_business(monkeys: &mut [Monkey], rounds: i32, limit: i64, divide_by_3: bool) {
    let nmonk = monkeys.len();

    for _ in 0..rounds {
        for mid in 0..nmonk {
            let nitems = monkeys[mid].items.len();
            monkeys[mid].business += nitems as i64;
            for iid in 0..nitems {
                let it = monkeys[mid].items[iid];

                let mut nlvl = monkeys[mid].do_op(it);
                if divide_by_3 {
                    nlvl /= 3;
                }
                nlvl %= limit;

                let tid = monkeys[mid].do_test(nlvl);
                monkeys[tid].items.push(nlvl);
            }
            monkeys[mid].items.clear();
        }
    }

    let mut businesss: Vec<i64> = monkeys
        .iter()
        .map(|m| m.business)
        .collect();
    businesss.sort_by(|a, b| b.cmp(a));

    println!("{}", businesss[0] * businesss[1]);

}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_id: i32 = -1;
    let mut monkey_line: usize = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let lineb = line.as_bytes();

        if lineb.is_empty() {}
        else if lineb[0] == b'M' {
            monkey_line = 0;
            monkey_id += 1;
            monkeys.push(Monkey::new());
        }
        else {
            let i = monkey_id as usize;
            match monkey_line {
                1 => monkeys[i].set_start_items(&line[18..]),
                2 => monkeys[i].set_op(&line[19..]),
                3 => monkeys[i].set_test(&line[21..]),
                4 => monkeys[i].set_throw_true(&line[29..]),
                5 => monkeys[i].set_throw_false(&line[30..]),
                6 => (),
                _ => panic!("Never reached!"),
            }
        }

        monkey_line += 1;
    }

    let glob_mod = monkeys
        .iter()
        .map(|m| m.test)
        .product();

    if false {
        compute_monk_business(&mut monkeys, 20, glob_mod, true);
    } 
    else {
        compute_monk_business(&mut monkeys, 10000, glob_mod, false);
    }

}
