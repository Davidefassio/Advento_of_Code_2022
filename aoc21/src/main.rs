use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    A,
    S,
    M,
    D,
}

impl Op {
    fn new(s: &str) -> Op {
        match s {
            "+" => Op::A,
            "-" => Op::S,
            "*" => Op::M,
            "/" => Op::D,
            _ => panic!("Unknown symbol."),
        }
    }

    fn doop(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Op::A => op1 + op2,
            Op::S => op1 - op2,
            Op::M => op1 * op2,
            Op::D => op1 / op2,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Monk {
    Imm(i64),
    Parent(Box<(Monk, Op, Monk)>),
    Human(i64),
}

impl Monk {
    fn from_str(id: &str, known_values: &HashMap<String, String>) -> Self {
        let s = known_values.get(id).unwrap();
        if s.chars().filter(|c| c.is_numeric()).count() > 0 {
            let n = s.parse::<i64>().unwrap();
            if id == "humn" {
                return Monk::Human(n);
            }
            return Monk::Imm(n);
        } else {
            let args = s.splitn(3, ' ').collect::<Vec<_>>();
            return Monk::Parent(Box::new((
                Monk::from_str(args[0], known_values),
                Op::new(args[1]),
                Monk::from_str(args[2], known_values),
            )));
        }
    }

    fn value(&self) -> i64 {
        match self {
            Monk::Imm(n) | Monk::Human(n) => *n,
            Monk::Parent(b) => b.1.doop(b.0.value(), b.2.value()),
        }
    }

    fn into_children(self) -> (Monk, Op, Monk) {
        if let Monk::Parent(b) = self {
            *b
        } else {
            panic!("attempted to get children of a non-parent")
        }
    }

    fn undo_op(self, h: &mut i64) -> Monk {
        let (lhs, op, rhs) = self.into_children();
        match op {
            Op::A => {
                let (cons, var) = if matches!(lhs, Monk::Imm(..)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                *h -= cons.value();
                var
            }
            Op::S => {
                if let Monk::Imm(n) = lhs {
                    *h = n - *h;
                    rhs
                } else if let Monk::Imm(n) = rhs {
                    *h += n;
                    lhs
                } else {
                    unreachable!()
                }
            }
            Op::M => {
                let (cons, var) = if matches!(lhs, Monk::Imm(..)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                *h /= cons.value();
                var
            }
            Op::D => {
                if let Monk::Imm(n) = lhs {
                    *h = n / *h;
                    rhs
                } else if let Monk::Imm(n) = rhs {
                    *h *= n;
                    lhs
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn make_immediates(&mut self) {
        if let Monk::Parent(b) = self {
            let (lhs, op, rhs) = b.as_mut();
            lhs.make_immediates();
            rhs.make_immediates();
            if let (Monk::Imm(a), Monk::Imm(b)) = (lhs, rhs) {
                *self = Monk::Imm(op.doop(*a, *b));
            }
        }
    }

    fn solve_for_humn(mut self) -> i64 {
        self.make_immediates();
        let (lhs, _, rhs) = self.into_children();
        let (constant_side, mut human_side) = if matches!(lhs, Monk::Imm(..)) {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        let mut h = constant_side.value();
        while !matches!(human_side, Monk::Human(..)) {
            human_side = human_side.undo_op(&mut h);
        }
        h
    }
}

fn main() {
    let mut known_values: HashMap<String, String> = HashMap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let (id, monk_str) = line.split_once(": ").unwrap();
        known_values.insert(id.to_string(), monk_str.to_string());
    }

    let root = Monk::from_str("root", &known_values);
    println!("{}", root.value());
    println!("{}", root.solve_for_humn());
}
