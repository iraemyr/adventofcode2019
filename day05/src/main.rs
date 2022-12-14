use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 5182797
    println!("{}", part2(contents)); // 12077198
}

fn part1(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.set_input(1);
    if comp.run() {
        let mut v = comp.get_outputs();
        return v.pop().unwrap();
    }
    -1
}

fn part2(program: String) -> i64 {
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
        assert_eq!(part1(contents), 5182797);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 12077198);
    }
}
