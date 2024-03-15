use std::fs::read_to_string;

struct Valve {
    id: String,
    flow_rate: i32,
    tunnel: Vec<String>,
}

#[derive(Debug, Clone)]
struct ValveOpt {
    flow_rate: i32,
    is_open: bool,
    tunnel: Vec<usize>,
}

fn hyp_pres(closed_valv: &[i32], mut curr_time: i32) -> i32 {
    let mut retval = 0;
    let mut i = 0;
    while i < closed_valv.len() && curr_time > 0 {
        retval += closed_valv[i] * curr_time;
        i += 1;
        curr_time -= 2;
    }
    retval
}

fn remove_n(closed_valv: &[i32], n: i32) -> Vec<i32>{
    let mut i = 0;
    let mut retval: Vec<i32> = closed_valv.to_vec();
    while i < retval.len() {
        if retval[i] == n {
            retval.remove(i);
            break;
        }
        i += 1;
    }
    retval
}

fn optimize_rec(valves: &mut [ValveOpt], curr_id: usize, mut time: i32, curr_p: i32, mut min_value: i32, closed_valv: Vec<i32>) -> i32 {
    if time == 0 {
        return curr_p;
    }
    time -= 1;

    if curr_p + hyp_pres(&closed_valv, time) <= min_value {
        return 0;
    }

    let mut retval = i32::MIN;
    if !valves[curr_id].is_open && valves[curr_id].flow_rate > 0 {
        valves[curr_id].is_open = true;
        retval = optimize_rec(valves, curr_id, time, curr_p + valves[curr_id].flow_rate * time, 
            min_value, remove_n(&closed_valv, valves[curr_id].flow_rate));
        valves[curr_id].is_open = false;

        if retval > min_value {
            min_value = retval;
        }
    }

    for i in 0..valves[curr_id].tunnel.len() {
        let r = optimize_rec(valves, valves[curr_id].tunnel[i], time, curr_p, min_value, closed_valv.clone());
        if r > retval {
            retval = r;
        }

        if retval > min_value {
            min_value = retval;
        }
    }

    retval
}

fn optimize(valves: &[ValveOpt], start_id: usize) -> i32 {
    let mut closed_valv: Vec<i32> = valves.iter().map(|v| v.flow_rate).collect();
    closed_valv.sort();
    closed_valv.reverse();
    optimize_rec(&mut valves.to_vec(), start_id, 30, 0, i32::MIN, closed_valv)
}

fn remove_n_2(closed_valv: &[i32], n1: i32, n2: i32) -> Vec<i32>{
    remove_n(&remove_n(&closed_valv, n1), n2)
}

fn optimize_rec_2(valves: &mut [ValveOpt], id1: usize, id2: usize, mut time: i32, curr_p: i32, mut min_value: i32, closed_valv: Vec<i32>) -> i32 {
    if time == 0 {
        return curr_p;
    }
    time -= 1;

    if curr_p + hyp_pres(&closed_valv, time) <= min_value {
        return 0;
    }

    let mut retval = i32::MIN;
    if !valves[id1].is_open && valves[id1].flow_rate > 0 {
        valves[id1].is_open = true;

        if !valves[id2].is_open && valves[id2].flow_rate > 0 {
            valves[id2].is_open = true;
            retval = optimize_rec_2(valves, id1, id2, time, curr_p + (valves[id1].flow_rate + valves[id2].flow_rate) * time, 
                min_value, remove_n_2(&closed_valv, valves[id1].flow_rate, valves[id2].flow_rate));
            valves[id2].is_open = false;

            if retval > min_value {
                min_value = retval;
            }
        }

        for i in 0..valves[id2].tunnel.len() {
            let r = optimize_rec_2(valves, id1, valves[id2].tunnel[i], time, 
                curr_p + valves[id1].flow_rate * time, min_value, remove_n(&closed_valv, valves[id1].flow_rate));
            if r > retval {
                retval = r;
            }
    
            if retval > min_value {
                min_value = retval;
            }
        }

        valves[id1].is_open = false;
    }

    for i in 0..valves[id1].tunnel.len() {
        if !valves[id2].is_open && valves[id2].flow_rate > 0 {
            valves[id2].is_open = true;
            let r = optimize_rec_2(valves, valves[id1].tunnel[i], id2, time, curr_p + valves[id2].flow_rate * time, 
                min_value, remove_n(&closed_valv, valves[id2].flow_rate));
            valves[id2].is_open = false;

            if r > retval {
                retval = r;
            }

            if retval > min_value {
                min_value = retval;
            }
        }

        for j in 0..valves[id2].tunnel.len() {
            let r = optimize_rec_2(valves, valves[id1].tunnel[i], valves[id2].tunnel[j], time, 
                curr_p, min_value, closed_valv.clone());
            if r > retval {
                retval = r;
            }
    
            if retval > min_value {
                min_value = retval;
            }
        }
    }

    retval
}

fn optimize_2(valves: &[ValveOpt], start_id: usize) -> i32 {
    let mut closed_valv: Vec<i32> = valves.iter().map(|v| v.flow_rate).collect();
    closed_valv.sort();
    closed_valv.reverse();
    optimize_rec_2(&mut valves.to_vec(), start_id, start_id, 26, 0, i32::MIN, closed_valv)
}

fn main() {
    let mut valve_tmp: Vec<Valve> = Vec::new();
    let mut start: usize = 0;

    for (n, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        let lineb = line.as_bytes();
        let id = line[6..8].to_string();
        if id == "AA" {
            start = n; 
        }
        
        let mut sc = 0;
        for i in 23.. {
            if lineb[i] == b';' {
                sc = i;
                break;
            }
        }
        let flow_rate: i32 = line[23..sc].parse().unwrap();
        let mut tunnel: Vec<String> = Vec::new();
        let mut i = sc + 24;

        if lineb[i] == b' ' {
            i += 1;
        }
        while i < lineb.len() {
            tunnel.push(line[i..i+2].to_string());
            i += 4;
        }

        valve_tmp.push(Valve { id, flow_rate, tunnel });
    }

    let mut valves: Vec<ValveOpt> = Vec::new();

    for v in &valve_tmp {
        let mut tunnel: Vec<usize> = Vec::new();

        for t in &v.tunnel {
            for (n, v2) in (&valve_tmp).iter().enumerate() {
                if v2.id == *t {
                    tunnel.push(n);
                }
            }
        }

        valves.push(ValveOpt { flow_rate: v.flow_rate, is_open: false, tunnel });
    }

    println!("{}", optimize(&valves, start));
    println!("{}", optimize_2(&valves, start));
}
