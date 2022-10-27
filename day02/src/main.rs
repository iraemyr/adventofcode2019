use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 3101878
    println!("{}", part2(contents)); // 8444
}

fn part1(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.restore();
    comp.run();
    comp.read(0) // 3101878
}

fn part2(program: String) -> i64 {
    let mut result = 0;
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut comp = Intcode::intcode_instance(program.clone());
            comp.write(noun, 1);
            comp.write(verb, 2);
            comp.run();
            if comp.read(0) == 19690720 {
                result = noun * 100 + verb; // 8444
                break 'outer;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 3101878);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 8444);
    }
}
