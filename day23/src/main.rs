use int_code_computer::Intcode;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 20367
    println!("{}", part2(contents)); // 15080
}

fn part1(program: String) -> i64 {
    let mut computers: Vec<Intcode> = Vec::new();
    let mut messages: Vec<VecDeque<i64>> = Vec::new();
    for i in 0..50 {
        let mut comp = Intcode::intcode_instance(program.clone());
        comp.set_input(i as i64);
        comp.run();
        computers.push(comp);
        messages.push(VecDeque::new());
    }

    loop {
        for i in 0..50 {
            let comp = &mut computers[i];
            if comp.awaiting_input() {
                let q = &mut messages[i];
                if q.is_empty() {
                    comp.set_input(-1);
                } else {
                    comp.set_input(q.pop_front().unwrap());
                }
            }
            comp.step();
            if comp.packet_ready() {
                let v = comp.get_outputs();
                let addr = v[0];
                if addr == 255 {
                    return v[2];
                }
                let q = &mut messages[addr as usize];
                q.push_back(v[1]);
                q.push_back(v[2]);
            }
        }
    }
}

fn part2(program: String) -> i64 {
    let mut computers: Vec<Intcode> = Vec::new();
    let mut messages: Vec<VecDeque<i64>> = Vec::new();
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut sent = -1;
    for i in 0..50 {
        let mut comp = Intcode::intcode_instance(program.clone());
        comp.set_input(i as i64);
        comp.run();
        computers.push(comp);
        messages.push(VecDeque::new());
    }

    let mut idle = [0; 50];
    loop {
        for i in 0..50 {
            let comp = &mut computers[i];
            if comp.awaiting_input() {
                let q = &mut messages[i];
                if q.is_empty() {
                    comp.set_input(-1);
                    idle[i] += 1;
                } else {
                    comp.set_input(q.pop_front().unwrap());
                    idle[i] = 0;
                }
            }
            comp.step();
            if comp.packet_ready() {
                let v = comp.get_outputs();
                let addr = v[0];
                if addr == 255 {
                    nat_x = v[1];
                    nat_y = v[2];
                } else {
                    let q = &mut messages[addr as usize];
                    q.push_back(v[1]);
                    q.push_back(v[2]);
                }
            }
        }
        let mut all_idle = true;
        for id in idle {
            if id < 2 {
                all_idle = false;
                break;
            }
        }
        if all_idle {
            if sent == nat_y {
                return nat_y;
            }
            let q = &mut messages[0];
            q.push_back(nat_x);
            q.push_back(nat_y);
            sent = nat_y;
            idle[0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 20367);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 15080);
    }
}
