use int_code_computer::Intcode;
use std::fs;

fn main() {
    let mut contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents)); // 121
    contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part2(contents)); // 15090773
}

fn part1(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    for y in 0..50 {
        for x in 0..50 {
            comp.reset();
            comp.run();
            comp.set_input(x);
            comp.run();
            comp.set_input(y);
            comp.run();
        }
    }
    comp.get_outputs().into_iter().sum()
}

fn part2(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    let mut x_low = 0;
    for y in 0.. {
        for x in x_low.. {
            if is_pulled(&mut comp, x, y) {
                x_low = x;
                if !is_pulled(&mut comp, x + 99, y) {
                    break;
                } else if is_pulled(&mut comp, x, y + 99) {
                    return x * 10_000 + y;
                }
            }
        }
    }
    0
}

fn is_pulled(comp: &mut Intcode, x: i64, y: i64) -> bool {
    comp.reset();
    comp.run();
    comp.set_input(x);
    comp.run();
    comp.set_input(y);
    comp.run();
    comp.get_last_output() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 121);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 15090773);
    }
}
