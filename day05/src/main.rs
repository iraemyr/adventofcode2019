use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let v: Vec<i32> = contents
        .split(',')
        .map(|a| a.trim().parse::<i32>().unwrap())
        .collect();
    println!("{}", part1(v.clone())); // 5182797
    println!("{}", part2(v)); // 12077198
}

fn part1(program: Vec<i32>) -> i32 {
    let mut comp = Intcode::intcode_instance(program);
    comp.set_input(1);
    if comp.run() {
        return comp.get_output();
    }
    -1
}

fn part2(program: Vec<i32>) -> i32 {
    let mut comp = Intcode::intcode_instance(program);
    comp.set_input(5);
    if comp.run() {
        return comp.get_output();
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let v: Vec<i32> = contents
            .split(',')
            .map(|a| a.trim().parse::<i32>().unwrap())
            .collect();
        assert_eq!(part1(v), 5182797);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let v: Vec<i32> = contents
            .split(',')
            .map(|a| a.trim().parse::<i32>().unwrap())
            .collect();
        assert_eq!(part2(v), 12077198);
    }
}
