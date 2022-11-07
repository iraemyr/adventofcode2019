use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 6672
    println!("{}", part2(contents)); // 923017
}

fn part1(program: String) -> usize {
    let mut comp = Intcode::intcode_instance(program);
    comp.run();
    let output = comp.get_outputs();
    let o: String = output.iter().map(|i| (*i as u8) as char).collect();
    let v: Vec<Vec<char>> = o.lines().map(|s| s.chars().collect()).collect();

    let mut alignment = 0;
    for y in 1..(v.len() - 2) {
        let vec = &v[y];
        for x in 1..(vec.len() - 1) {
            if is_intersection(&v, y, x) {
                alignment += x * y;
            }
        }
    }
    print_map(&v);
    alignment
}

fn part2(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.write(2, 0);
    comp.run();
    for b in b"B,C,C,A,A,B,B,C,C,A\n" {
        // Main program
        comp.set_input(*b as i64);
        comp.run();
    }
    for b in b"R,12,R,4,L,6,L,8,L,8\n" {
        // Program A
        comp.set_input(*b as i64);
        comp.run();
    }
    for b in b"L,12,R,4,R,4\n" {
        // Program B
        comp.set_input(*b as i64);
        comp.run();
    }
    for b in b"R,12,R,4,L,12\n" {
        // Program C
        comp.set_input(*b as i64);
        comp.run();
    }
    for b in b"n\n" {
        // No video feed
        comp.set_input(*b as i64);
        comp.run();
    }
    comp.get_last_output()
}

fn is_intersection(map: &[Vec<char>], row: usize, col: usize) -> bool {
    let c = map[row][col];
    if c != '#' {
        return false;
    }
    let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    neighbors.iter().all(|n| {
        let r = row as i32 + n.0;
        let c = col as i32 + n.1;
        map[r as usize][c as usize] == '#'
    })
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 6672);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 923017);
    }
}
