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
    let map = parse_map(contents);
    let portals = process_map(&map);
    println!("{}", part1(&map, &portals)); // 686
    println!("{}", part2(&map, &portals)); // 8384
}

fn part1(
    map: &HashMap<(usize, usize), Tile>,
    portals: &HashMap<(u8, u8), Vec<(usize, usize)>>,
) -> usize {
    let start = portals[&(b'A', b'A')].first().unwrap();
    let end = portals[&(b'Z', b'Z')].first().unwrap();
    dijkstra(start, |p| neighbors(map, portals, *p), |p| *p == *end)
        .unwrap()
        .1
}

fn part2(
    map: &HashMap<(usize, usize), Tile>,
    portals: &HashMap<(u8, u8), Vec<(usize, usize)>>,
) -> usize {
    let (x, y) = portals[&(b'A', b'A')].first().unwrap();
    let start = &(*x, *y, 0_usize);
    let (x, y) = portals[&(b'Z', b'Z')].first().unwrap();
    let end = &(*x, *y, 0_usize);
    dijkstra(start, |p| neighbors_dim(map, portals, *p), |p| *p == *end)
        .unwrap()
        .1
}

fn parse_map(s: String) -> HashMap<(usize, usize), Tile> {
    s.lines()
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
        .collect()
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

fn neighbors_dim(
    map: &HashMap<(usize, usize), Tile>,
    portals: &HashMap<(u8, u8), Vec<(usize, usize)>>,
    pos: (usize, usize, usize),
) -> Vec<((usize, usize, usize), usize)> {
    let mut n = Vec::new();
    let inner = pos.0 >= 35 && pos.0 <= 100 && pos.1 >= 35 && pos.1 <= 100;
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let p = (
            (pos.0 as i32 + dir.0) as usize,
            (pos.1 as i32 + dir.1) as usize,
        );
        match map[&p] {
            Tile::Space => n.push(((p.0, p.1, pos.2), 1)),
            Tile::Symbol(c) => {
                let p2 = ((p.0 as i32 + dir.0) as usize, (p.1 as i32 + dir.1) as usize);
                if let Tile::Symbol(c2) = map.get(&p2).unwrap() {
                    if (inner || pos.2 > 0) && c != *c2 {
                        let points = portals.get(&(c.max(*c2), c.min(*c2))).unwrap();
                        let first = *points.first().unwrap();
                        let p_dim = if inner {
                            pos.2.saturating_add(1)
                        } else {
                            pos.2.saturating_sub(1)
                        };
                        if (pos.0, pos.1) != first {
                            n.push(((first.0, first.1, p_dim), 1));
                        } else {
                            let point = *points.get(1).unwrap();
                            n.push(((point.0, point.1, p_dim), 1));
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
        let map = parse_map(contents);
        let portals = process_map(&map);
        assert_eq!(part1(&map, &portals), 686);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let map = parse_map(contents);
        let portals = process_map(&map);
        assert_eq!(part2(&map, &portals), 8384);
    }
}
