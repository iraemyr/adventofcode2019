#![allow(dead_code)]
use console::Term;
use int_code_computer::Intcode;
use std::collections::HashMap;
use std::fs;
//use std::{thread, time};

#[derive(Default)]
struct MyMap {
    map: HashMap<(i32, i32), char>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl MyMap {
    fn insert(&mut self, pos: (i32, i32), c: char) {
        if pos.0 < self.min_x {
            self.min_x = pos.0;
        }
        if pos.0 > self.max_x {
            self.max_x = pos.0;
        }
        if pos.1 < self.min_y {
            self.min_y = pos.1;
        }
        if pos.1 > self.max_y {
            self.max_y = pos.1;
        }
        self.map.insert(pos, c);
    }

    fn get(&self, pos: (i32, i32)) -> char {
        *self.map.get(&pos).unwrap()
    }

    pub fn print(&self, pos: (i32, i32)) {
        println!("\n\n");
        for y in (self.min_y..=self.max_y).rev() {
            for x in self.min_x..=self.max_x {
                let p = (x, y);
                if p == pos {
                    print!("*");
                } else if p == (0, 0) {
                    print!("S");
                } else if let Some(c) = self.map.get(&p) {
                    print!("{}", c);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn print_plain(&self) {
        println!("\n\n");
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let p = (x, y);
                print!("{}", self.map.get(&p).unwrap());
            }
            println!();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("map.txt").expect("File not found");
    let result = part1_part2(contents);
    println!("{}", result.0); // 218
    println!("{}", result.1); // 544
                              //robot_sim(contents);
}

fn robot_sim(program: String) {
    let mut comp = Intcode::intcode_instance(program);
    let mut map = MyMap::default();
    let mut pos = (0, 0);
    comp.run();
    while !comp.is_halted() {
        let direction = get_input();
        let poi = match direction {
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0, pos.1 - 1),
            3 => (pos.0 - 1, pos.1),
            4 => (pos.0 + 1, pos.1),
            _ => (-1, -1),
        };
        if poi == (-1, -1) {
            continue;
        }
        comp.set_input(direction as i64);
        comp.run();
        match comp.get_output() {
            0 => {
                map.insert(poi, '#');
            }
            1 => {
                map.insert(poi, '.');
                pos = poi
            }
            2 => {
                map.insert(poi, '0');
                pos = poi
            }
            _ => {}
        }
        map.print(pos);
    }
}

fn part1_part2(contents: String) -> (i32, i32) {
    let mut map = MyMap::default();
    let mut start = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '0' {
                start = (x as i32, y as i32);
            }
            map.insert((x as i32, y as i32), c);
        }
    }

    let mut q = Vec::<(i32, i32)>::new();
    let mut q2 = Vec::<(i32, i32)>::new();
    q.push(start);
    let mut step = -1;
    let mut part1 = 0;
    while !q.is_empty() {
        step += 1;

        for p in &q {
            if map.get(*p) == 'S' {
                part1 = step;
            }
            map.insert(*p, '*');
            let mut c = map.get((p.0 + 1, p.1));
            if c == '.' || c == 'S' {
                q2.push((p.0 + 1, p.1));
            }
            c = map.get((p.0 - 1, p.1));
            if c == '.' || c == 'S' {
                q2.push((p.0 - 1, p.1));
            }
            c = map.get((p.0, p.1 + 1));
            if c == '.' || c == 'S' {
                q2.push((p.0, p.1 + 1));
            }
            c = map.get((p.0, p.1 - 1));
            if c == '.' || c == 'S' {
                q2.push((p.0, p.1 - 1));
            }
        }
        //map.print_plain();
        std::mem::swap(&mut q, &mut q2);
        q2.clear();
        //thread::sleep(time::Duration::from_millis(500));
    }

    (part1, step)
}

fn get_input() -> i32 {
    let stdout = Term::buffered_stdout();
    if let Ok(c) = stdout.read_char() {
        match c {
            'w' => 1,
            's' => 2,
            'a' => 3,
            'd' => 4,
            _ => -1,
        }
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_and_part2() {
        let contents = fs::read_to_string("map.txt").expect("File not found");
        let result = part1_part2(contents);
        assert_eq!(result.0, 218);
        assert_eq!(result.1, 544);
    }
}
