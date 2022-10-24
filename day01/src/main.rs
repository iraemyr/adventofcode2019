use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let result = part_1_and_2(contents);
    println!("{}", result.0); // 3295539
    println!("{}", result.1); // 4940441
}

fn part_1_and_2(contents: String) -> (i64, i64) {
    let fields = contents.split("\r\n");
    let mut tot: i64 = 0;
    let mut tot2: i64 = 0;
    for field in fields {
        if let Ok(mass) = field.trim().parse::<i64>() {
            tot += calc_fuel(mass);
            tot2 += calc_fuel2(mass);
        } else {
            println!("Houston, we have a problem");
        }
    }
    (tot, tot2)
}

fn calc_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn calc_fuel2(mass: i64) -> i64 {
    let mut fuel = 0i64;
    let mut m = mass;
    while m > 0 {
        m = calc_fuel(m);
        if m > 0 {
            fuel += m;
        }
    }
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcf1() {
        assert_eq!(calc_fuel(12), 2);
    }

    #[test]
    fn calcf2() {
        assert_eq!(calc_fuel(14), 2);
    }

    #[test]
    fn calcf3() {
        assert_eq!(calc_fuel(1969), 654);
    }

    #[test]
    fn calcf4() {
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn calc2f1() {
        assert_eq!(calc_fuel2(14), 2);
    }

    #[test]
    fn calc2f2() {
        assert_eq!(calc_fuel2(1969), 966);
    }

    #[test]
    fn calc2f3() {
        assert_eq!(calc_fuel2(100756), 50346);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part_1_and_2(contents).0, 3295539);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part_1_and_2(contents).1, 4940441);
    }
}
