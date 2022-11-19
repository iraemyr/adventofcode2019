use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 28772955
    println!("{}", part2(contents)); // 2023
}

fn part1(s: String) -> u32 {
    let mut life = parse_map(s);
    let mut states: HashSet<u32> = HashSet::new();
    states.insert(score(&life));
    loop {
        sim(&mut life);
        let score = score(&life);
        if states.contains(&score) {
            return score;
        }
        states.insert(score);
    }
}

fn part2(s: String) -> u32 {
    let mut life = parse_map_rec(s);
    for _ in 0..200 {
        sim_rec(&mut life);
    }
    life.len() as u32
}

fn parse_map(s: String) -> HashMap<(i32, i32), bool> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, b)| ((x as i32, y as i32), matches!(b, b'#')))
        })
        .collect()
}

fn parse_map_rec(s: String) -> HashSet<(i32, i32, i32)> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().map(move |(x, b)| {
                let t = matches!(b, b'#');
                ((x as i32, y as i32, 0), t)
            })
        })
        .filter(|(_, b)| *b)
        .map(|(p, _)| p)
        .collect()
}

fn score(map: &HashMap<(i32, i32), bool>) -> u32 {
    let mut mul = 1;
    let mut score = 0;
    for row in 0..5_i32 {
        for col in 0..5_i32 {
            if map[&(col, row)] {
                score += mul;
            }
            mul <<= 1;
        }
    }
    score
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i32, i32), bool>) {
    for y in 0..5 {
        for x in 0..5 {
            if map[&(x, y)] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_level(map: &HashSet<(i32, i32, i32)>, level: i32) {
    for y in 0..5 {
        for x in 0..5 {
            if map.contains(&(x, y, level)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn neighbors(pos: (i32, i32), map: &HashMap<(i32, i32), bool>) -> u8 {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|p| *map.get(&(pos.0 + p.0, pos.1 + p.1)).unwrap_or(&false))
        .filter(|b| *b)
        .count() as u8
}

fn neighbors_rec(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    if pos.0 == 2 && pos.1 == 2 {
        panic!("Don't call neighbors on middle");
    }
    let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();
    // Up
    if pos.1 == 0 {
        neighbors.push((2, 1, pos.2 - 1));
    } else if pos.1 == 3 && pos.0 == 2 {
        for i in 0..5 {
            neighbors.push((i, 4, pos.2 + 1));
        }
    } else {
        neighbors.push((pos.0, pos.1 - 1, pos.2));
    }

    // Down
    if pos.1 == 4 {
        neighbors.push((2, 3, pos.2 - 1));
    } else if pos.1 == 1 && pos.0 == 2 {
        for i in 0..5 {
            neighbors.push((i, 0, pos.2 + 1));
        }
    } else {
        neighbors.push((pos.0, pos.1 + 1, pos.2));
    }

    // Right
    if pos.0 == 4 {
        neighbors.push((3, 2, pos.2 - 1));
    } else if pos.0 == 1 && pos.1 == 2 {
        for i in 0..5 {
            neighbors.push((0, i, pos.2 + 1));
        }
    } else {
        neighbors.push((pos.0 + 1, pos.1, pos.2));
    }

    // Left
    if pos.0 == 0 {
        neighbors.push((1, 2, pos.2 - 1));
    } else if pos.0 == 3 && pos.1 == 2 {
        for i in 0..5 {
            neighbors.push((4, i, pos.2 + 1));
        }
    } else {
        neighbors.push((pos.0 - 1, pos.1, pos.2));
    }
    neighbors
}

fn sim(map: &mut HashMap<(i32, i32), bool>) {
    let mut v: Vec<((i32, i32), bool)> = Vec::new();
    for y in 0..5 {
        for x in 0..5 {
            let p = &(x, y);
            let n = neighbors(*p, map);
            let b = map[p];
            if b && n != 1 {
                v.push((*p, false));
            } else if !b && (n == 1 || n == 2) {
                v.push((*p, true));
            }
        }
    }
    while !v.is_empty() {
        let (pos, b) = v.pop().unwrap();
        map.insert(pos, b);
    }
}

fn sim_rec(map: &mut HashSet<(i32, i32, i32)>) {
    let mut visit: HashSet<(i32, i32, i32)> = HashSet::new();
    for p in map.iter() {
        visit.insert(*p);
        visit.extend(neighbors_rec(*p).iter());
    }
    let mut ins: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut remove: HashSet<(i32, i32, i32)> = HashSet::new();
    for p in visit {
        let num = neighbors_rec(p)
            .into_iter()
            .filter(|p| map.contains(p))
            .count();
        let contains = map.contains(&p);
        if contains && num != 1 {
            remove.insert(p);
        } else if !contains && (num == 1 || num == 2) {
            ins.insert(p);
        }
    }
    map.extend(ins.iter());
    map.retain(|p| !remove.contains(p));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 28772955);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 2023);
    }
}
