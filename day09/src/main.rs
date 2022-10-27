use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 2941952859
    println!("{}", part2(contents)); // 66113
}

fn part1(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.set_input(1);
    comp.run();
    comp.get_output()
}

fn part2(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.set_input(2);
    comp.run();
    comp.get_output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 2941952859);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 66113);
    }
}
