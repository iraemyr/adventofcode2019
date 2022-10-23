extern crate pathfinding;

use pathfinding::prelude::bfs;
use std::collections::HashMap;
use std::fs;

struct Orbits<'a> {
    orb: HashMap<&'a str, Vec<&'a str>>,
}

impl Orbits<'_> {
    pub fn start(&self) -> &str {
        self.orb.get("YOU").unwrap()[0]
    }

    pub fn end(&self) -> &str {
        self.orb.get("SAN").unwrap()[0]
    }

    pub fn successors(&self, k: &str) -> Vec<&str> {
        self.orb.get(k).unwrap().clone()
    }
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 261306
    println!("{}", part2(contents)); // 382
}

fn part1(data: String) -> i32 {
    let orbits: HashMap<_, _> = data.lines().map(|a| -> (&str, &str) { parse(a) }).collect();
    //println!("{:?}", orbits);
    let mut orbit_counts: HashMap<&str, i32> = HashMap::new();
    orbit_counts.insert("COM", 0);
    for key in orbits.keys() {
        calc_orbits(&mut orbit_counts, &orbits, key);
    }
    //println!("{:?}", orbit_counts);
    orbit_counts.values().sum()
}

fn part2(data: String) -> i32 {
    let mut transfers: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in data.lines() {
        let points = parse(line);
        let mut links = transfers.entry(points.0).or_insert_with(Vec::new);
        links.push(points.1);
        links = transfers.entry(points.1).or_insert_with(Vec::new);
        links.push(points.0);
    }

    let orbits = Orbits { orb: transfers };

    let result = bfs(
        &orbits.start(),
        |p| orbits.successors(*p),
        |p| *p == orbits.end(),
    )
    .unwrap()
    .len()
        - 1;
    result as i32
}

fn parse(s: &str) -> (&str, &str) {
    let fields = s.split_once(')').unwrap();
    (fields.1, fields.0)
}

fn calc_orbits<'a>(
    orbit_counts: &mut HashMap<&'a str, i32>,
    orbit_map: &HashMap<&str, &'a str>,
    object: &'a str,
) -> i32 {
    if let Some(x) = orbit_counts.get(&object) {
        return *x;
    }
    let count = 1 + calc_orbits(orbit_counts, orbit_map, orbit_map.get(object).unwrap());
    orbit_counts.insert(object, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(261306, part1(contents));
    }

    #[test]
    fn test_part1_simple() {
        let contents = fs::read_to_string("test.txt").expect("File not found");
        assert_eq!(42, part1(contents));
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(382, part2(contents));
    }

    #[test]
    fn test_part2_simple() {
        let contents = fs::read_to_string("test2.txt").expect("File not found");
        assert_eq!(4, part2(contents));
    }
}
