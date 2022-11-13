use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs};

#[derive(PartialEq, Debug)]
enum Tile {
    Wall,
    Space,
    Symbol(u8),
    Unused,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents)); // 686
}

fn part1(contents: String) -> usize {
    let map: HashMap<(usize, usize), Tile> = contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().map(move |(x, b)| {
                let t = match b {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Space,
                    b'A'..=b'Z' => Tile::Symbol(b),
                    _ => Tile::Unused,
                };
                ((x, y), t)
            })
        })
        .filter(|(_, t)| *t != Tile::Unused)
        .collect();
    let portals = process_map(&map);
    let start = portals[&(b'A', b'A')].first().unwrap();
    let end = portals[&(b'Z', b'Z')].first().unwrap();
    dijkstra(start, |p| neighbors(&map, &portals, *p), |p| *p == *end)
        .unwrap()
        .1
}

fn process_map(map: &HashMap<(usize, usize), Tile>) -> HashMap<(u8, u8), Vec<(usize, usize)>> {
    let mut portals = HashMap::new();
    for ((x, y), t) in map.iter() {
        if let Tile::Symbol(s) = *t {
            if *x > 0 && *y > 0 {
                if let Some(Tile::Space) = map.get(&(x.saturating_sub(1), *y)) {
                    if let Tile::Symbol(s1) = map[&(*x + 1, *y)] {
                        let v: &mut Vec<(usize, usize)> =
                            portals.entry((s.max(s1), s.min(s1))).or_default();
                        v.push((x.saturating_sub(1), *y));
                    }
                } else if let Some(Tile::Space) = map.get(&(*x + 1, *y)) {
                    if let Tile::Symbol(s1) = map[&(x.saturating_sub(1), *y)] {
                        let v: &mut Vec<(usize, usize)> =
                            portals.entry((s.max(s1), s.min(s1))).or_default();
                        v.push((*x + 1, *y));
                    }
                } else if let Some(Tile::Space) = map.get(&(*x, y.saturating_sub(1))) {
                    if let Tile::Symbol(s1) = map[&(*x, *y + 1)] {
                        let v: &mut Vec<(usize, usize)> =
                            portals.entry((s.max(s1), s.min(s1))).or_default();
                        v.push((*x, y.saturating_sub(1)));
                    }
                } else if let Some(Tile::Space) = map.get(&(*x, *y + 1)) {
                    if let Tile::Symbol(s1) = map[&(*x, y.saturating_sub(1))] {
                        let v: &mut Vec<(usize, usize)> =
                            portals.entry((s.max(s1), s.min(s1))).or_default();
                        v.push((*x, *y + 1));
                    }
                }
            }
        }
    }
    portals
}

fn neighbors(
    map: &HashMap<(usize, usize), Tile>,
    portals: &HashMap<(u8, u8), Vec<(usize, usize)>>,
    pos: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut n = Vec::new();
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let p = (
            (pos.0 as i32 + dir.0) as usize,
            (pos.1 as i32 + dir.1) as usize,
        );
        match map[&p] {
            Tile::Space => n.push((p, 1)),
            Tile::Symbol(c) => {
                let p2 = ((p.0 as i32 + dir.0) as usize, (p.1 as i32 + dir.1) as usize);
                if let Tile::Symbol(c2) = map.get(&p2).unwrap() {
                    if c != *c2 {
                        let points = portals.get(&(c.max(*c2), c.min(*c2))).unwrap();
                        let first = *points.first().unwrap();
                        if pos != first {
                            n.push((first, 1));
                        } else {
                            n.push((*points.get(1).unwrap(), 1));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 686);
    }

    // #[test]
    // fn test_part2() {
    //     let contents = fs::read_to_string("input2.txt").expect("File not found");
    //     assert_eq!(part2(contents), 2128);
    // }
}
