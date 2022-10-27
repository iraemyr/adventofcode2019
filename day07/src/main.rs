use int_code_computer::Intcode;
use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 95757
    println!("{}", part2(contents)); // 4275738
}

fn part1(program: String) -> i64 {
    let mut max = i64::MIN;
    for perm in (0..=4).permutations(5) {
        let mut output = 0;
        for p in perm {
            let mut amp = Intcode::intcode_instance(program.clone());
            amp.set_input(p);
            amp.run();
            amp.set_input(output);
            amp.run();
            output = amp.get_output();
        }
        max = max.max(output);
    }
    max
}

fn part2(program: String) -> i64 {
    let mut max = i64::MIN;
    for perm in (5..=9).permutations(5) {
        let mut amps: Vec<Intcode> = Vec::new();
        for p in perm {
            let mut amp = Intcode::intcode_instance(program.clone());
            amp.set_input(p);
            amp.run();
            amps.push(amp);
        }

        let mut outputs = [0; 5];
        'outer: loop {
            for i in 0..amps.len() {
                let amp = amps.get_mut(i).unwrap();
                if !amp.is_halted() {
                    amp.set_input(if i == 0 { outputs[4] } else { outputs[i - 1] });
                    amp.run();
                    outputs[i] = amp.get_output();
                }
                if i == 4 && amp.is_halted() {
                    break 'outer;
                }
            }
        }
        max = max.max(outputs[4]);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        let contents = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
        assert_eq!(part1(contents), 43210);
    }

    #[test]
    fn test_part1_simple2() {
        let contents =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string();
        assert_eq!(part1(contents), 54321);
    }

    #[test]
    fn test_part1_simple3() {
        let contents = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string();
        assert_eq!(part1(contents), 65210);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 95757);
    }

    #[test]
    fn test_part2_simple() {
        let contents =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string();
        assert_eq!(part2(contents), 139629729);
    }

    #[test]
    fn test_part2_simple2() {
        let contents =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string();
        assert_eq!(part2(contents), 18216);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 4275738);
    }
}
