use int_code_computer::Intcode;
use std::collections::HashMap;
use std::fs;
use std::iter;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 236
    println!("{}", part2(contents)); // 11040
}

fn part1(program: String) -> i32 {
    let mut comp = Intcode::intcode_instance(program);
    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    comp.run();
    let outputs = comp.get_outputs();
    let mut output = outputs.iter().peekable();
    while output.peek().is_some() {
        let x = *output.next().unwrap();
        let y = *output.next().unwrap();
        let c = *output.next().unwrap();
        screen.insert((x, y), c);
    }
    //draw_screen(&screen);
    screen.values().filter(|c| **c == 2_i64).count() as i32
}

fn part2(program: String) -> i64 {
    let mut comp = Intcode::intcode_instance(program);
    //let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    comp.write(2, 0);
    wall_hack(&mut comp);
    let mut zeroes = iter::repeat(0_i64);
    while !comp.is_halted() {
        comp.run();
        //let outputs = comp.get_outputs();
        //let mut output = outputs.iter().peekable();
        //while output.peek().is_some() {
        //    let x = *output.next().unwrap();
        //    let y = *output.next().unwrap();
        //    let c = *output.next().unwrap();
        //    screen.insert((x, y), c);
        //}
        //draw_screen(&screen);
        comp.set_input(zeroes.next().unwrap());
    }
    comp.get_last_output()
}

pub fn draw_screen(screen: &HashMap<(i64, i64), i64>) {
    for x in 0..21_i64 {
        for y in 0..36_i64 {
            print!("{}", screen.get(&(y, x)).unwrap_or(&-1_i64));
        }
        println!();
    }
}
fn wall_hack(comp: &mut Intcode) {
    // Moved the wall above the paddle to avoid infinite loop
    let base = 1360_i64 - (2 * 36);
    for x in 0..34 {
        comp.write(1, base + x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 236);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 11040);
    }
}
