use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: (u8, u8),
    geode_robot: (u8, u8),
}

impl Blueprint {
    fn max_ore_cost(&self) -> u8 {
        self.ore_robot.max(self.clay_robot).max(self.obsidian_robot.0).max(self.geode_robot.0)
    }

    fn max_clay_cost(&self) -> u8 {
        self.obsidian_robot.1
    }

    fn max_obsidian_cost(&self) -> u8 {
        self.geode_robot.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    minute: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl State {
    fn new() -> Self {
        State {
            minute: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn can_build_ore_robot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.ore_robot
    }

    fn can_build_clay_robot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.clay_robot
    }

    fn can_build_obsidian_robot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.obsidian_robot.0 && self.clay >= bp.obsidian_robot.1
    }

    fn can_build_geode_robot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.geode_robot.0 && self.obsidian >= bp.geode_robot.1
    }

    fn build_ore_robot(mut self, bp: &Blueprint) -> Self {
        self.ore -= bp.ore_robot;
        self.ore_robot += 1;
        self
    }

    fn build_clay_robot(mut self, bp: &Blueprint) -> Self {
        self.ore -= bp.clay_robot;
        self.clay_robot += 1;
        self
    }

    fn build_obsidian_robot(mut self, bp: &Blueprint) -> Self {
        self.ore -= bp.obsidian_robot.0;
        self.clay -= bp.obsidian_robot.1;
        self.obsidian_robot += 1;
        self
    }

    fn build_geode_robot(mut self, bp: &Blueprint) -> Self {
        self.ore -= bp.geode_robot.0;
        self.obsidian -= bp.geode_robot.1;
        self.geode_robot += 1;
        self
    }

    fn step(mut self) -> Self {
        self.minute += 1;
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
        self
    }
}

fn solution(
    state: State,
    bp: &Blueprint,
    limit: u8,
    max_result: &mut u8,
    can_ore: bool,
    can_clay: bool,
    can_obsidian: bool,
) {
    if state.minute == limit {
        *max_result = (*max_result).max(state.geode);
        return;
    }

    let remaining = (limit - state.minute) as u32;
    let max_yield = remaining * (state.geode_robot as u32) + remaining * (remaining - 1) / 2;
    if state.geode as u32 + max_yield <= *max_result as u32 {
        return;
    }

    if state.can_build_geode_robot(bp) {
        solution(state.step().build_geode_robot(bp), bp, limit, max_result, true, true, true);
    } 
    else {
        let mut new_can_obsidian = true;
        if state.can_build_obsidian_robot(bp) {
            new_can_obsidian = false;

            if can_obsidian && state.obsidian_robot < bp.max_obsidian_cost() {
                solution(state.step().build_obsidian_robot(bp), bp, limit, max_result, true, true, true);
            }
        }

        let mut new_can_clay = true;
        if state.can_build_clay_robot(bp) {
            new_can_clay = false;

            if can_clay && state.clay_robot < bp.max_clay_cost() {
                solution(state.step().build_clay_robot(bp), bp, limit, max_result, true, true, true);
            }
        }

        let mut new_can_ore = true;
        if state.can_build_ore_robot(bp) {
            new_can_ore = false;

            if can_ore && state.ore_robot < bp.max_ore_cost() {
                solution(state.step().build_ore_robot(bp), bp, limit, max_result, true, true, true);
            }
        }

        solution(state.step(), bp, limit, max_result, new_can_ore, new_can_clay, new_can_obsidian);
    }
}

fn solve(bp: &Blueprint, limit: u8) -> u32 {
    let mut max_result = 0;
    solution(State::new(), bp, limit, &mut max_result, true, true, true);
    max_result as u32
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 1;

    for (n, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        let line = line.replace(':', "");
        let line = line.replace('.', "");
        let tokens: Vec<&str> = line.split(' ').collect();

        let bp = Blueprint {
            ore_robot: tokens[6].parse().unwrap(),
            clay_robot: tokens[12].parse().unwrap(),
            obsidian_robot: (tokens[18].parse().unwrap(), tokens[21].parse().unwrap()),
            geode_robot: (tokens[27].parse().unwrap(), tokens[30].parse().unwrap()),
        };

        part1 += (n + 1) as u32 * solve(&bp, 24);

        if n < 3 {
            part2 *= solve(&bp, 32);
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
