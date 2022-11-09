use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pos: (usize, usize),
    keys: [bool; 26],
}

impl State {
    fn new(pos: (usize, usize), keys: [bool; 26]) -> State {
        State { pos, keys }
    }

    fn neighbors(&self, map: &HashMap<(usize, usize), char>) -> Vec<(State, usize)> {
        let mut states = Vec::new();
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pos = (
                (self.pos.0 as i32 + d.0) as usize,
                (self.pos.1 as i32 + d.1) as usize,
            );
            let c = map[&pos];
            match c {
                '#' => {}
                '.' | '@' => states.push((State::new(pos, self.keys), 1)),
                'a'..='z' => {
                    let mut st = State::new(pos, self.keys);
                    st.keys[c as usize - 'a' as usize] = true;
                    states.push((st, 1));
                }
                'A'..='Z' => {
                    if self.keys[c.to_ascii_lowercase() as usize - 'a' as usize] {
                        states.push((State::new(pos, self.keys), 1));
                    }
                }
                _ => {}
            }
        }
        states
    }

    fn neighbors_ignore_doors(&self, map: &HashMap<(usize, usize), char>) -> Vec<(State, usize)> {
        let mut states = Vec::new();
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pos = (
                (self.pos.0 as i32 + d.0) as usize,
                (self.pos.1 as i32 + d.1) as usize,
            );
            let c = map[&pos];
            match c {
                '#' => {}
                '.' | '@' | 'A'..='Z' => states.push((State::new(pos, self.keys), 1)),
                'a'..='z' => {
                    let mut st = State::new(pos, self.keys);
                    st.keys[c as usize - 'a' as usize] = true;
                    states.push((st, 1));
                }
                _ => {}
            }
        }
        states
    }
}

fn main() {
    let mut contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents)); // 4420
    contents = fs::read_to_string("input2.txt").expect("File not found");
    println!("{}", part2(contents)); //2128
}

fn part1(s: String) -> usize {
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut start = (0, 0);
    for (row, line) in s.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            if c == '@' {
                start = (row, col);
            }
            map.insert((row, col), c);
        }
    }
    let pos = State::new(start, [false; 26]);
    dijkstra(&pos, |p| p.neighbors(&map), |p| p.keys.iter().all(|b| *b))
        .unwrap()
        .1
}

fn part2(s: String) -> usize {
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut start = Vec::with_capacity(4);
    let mut keys = [0_usize; 4];
    for (row, line) in s.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            match c {
                '@' => start.push((row, col)),
                'a'..='z' => {
                    if row < 40 {
                        if col < 40 {
                            keys[0] += 1;
                        } else {
                            keys[1] += 1;
                        }
                    } else if col < 40 {
                        keys[2] += 1;
                    } else {
                        keys[3] += 1;
                    }
                }
                _ => {}
            }
            map.insert((row, col), c);
        }
    }

    let mut steps = 0;
    for (i, p) in start.iter().enumerate() {
        steps += dijkstra(
            &State::new(*p, [false; 26]),
            |p| p.neighbors_ignore_doors(&map),
            |p| p.keys.into_iter().filter(|b| *b).count() == keys[i],
        )
        .unwrap()
        .1
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 4420);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input2.txt").expect("File not found");
        assert_eq!(part2(contents), 2128);
    }
}
