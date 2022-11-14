use int_code_computer::Intcode;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 19361850
    println!("{}", part2(contents)); // 1138943788
}

fn part1(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.run();
    let bytes = b"NOT B J\nNOT C T\nOR T J\nAND D J\nNOT A T\nOR T J\nWALK\n";
    for byte in bytes.iter() {
        comp.set_input(*byte as i64);
        comp.run();
    }
    comp.get_last_output()
}

fn part2(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    comp.run();
    let bytes = b"NOT B J\nNOT C T\nOR T J\nAND D J\nAND H J\nNOT A T\nOR T J\nRUN\n";
    for byte in bytes.iter() {
        comp.set_input(*byte as i64);
        comp.run();
    }
    comp.get_last_output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 19361850);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 1138943788);
    }
}
