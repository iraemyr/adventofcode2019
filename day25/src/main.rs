use int_code_computer::Intcode;
use std::fs;
use std::io::stdin;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents)); // 25165890
}

fn _play(program: String) {
    let mut comp = Intcode::intcode_instance(program);
    comp.run();
    loop {
        let output = comp.get_outputs();
        let o: String = output.iter().map(|i| (*i as u8) as char).collect();
        println!("{o}");
        let mut line = String::new();
        _ = stdin().read_line(&mut line);
        for b in line.trim().as_bytes() {
            comp.set_input(*b as i64);
            comp.run();
        }
        comp.set_input(10_i64);
        comp.run();
    }
}

fn part1(program: String) -> u32 {
    let mut comp = Intcode::intcode_instance(program);
    comp.run();
    let command = "east\ntake ornament\nsouth\ntake festive hat\nnorth\nwest\nnorth\nnorth\ntake space heater\neast\ntake semiconductor\nwest\nsouth\nsouth\nwest\nnorth\nnorth\nwest\n";
    for b in command.as_bytes() {
        let _ = comp.get_outputs();
        comp.set_input(*b as i64);
        comp.run();
    }
    let output = comp.get_outputs();
    let o: String = output.iter().map(|i| (*i as u8) as char).collect();
    o.split(' ').nth(49).unwrap().parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 25165890);
    }
}
