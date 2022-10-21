fn main() {
    //println!("{}", part1()); // 460
    println!("{}", part1_stream()); // 460
                                    //println!("{}", part2()); // 290
    println!("{}", part2_stream()); // 290
}

fn part1() -> i32 {
    let mut valid = 0;
    for i in 382345..=843167 {
        if valid_password(i) {
            valid += 1;
        }
    }
    valid
}

fn part1_stream() -> i32 {
    (382345..=843167).filter(|a| valid_password(*a)).count() as i32
}

fn part2_stream() -> i32 {
    (382345..=843167).filter(|a| valid_password2(*a)).count() as i32
}

fn part2() -> i32 {
    let mut valid = 0;
    for i in 382345..=843167 {
        if valid_password2(i) {
            valid += 1;
        }
    }
    valid
}

fn valid_password(pass: i32) -> bool {
    let mut repeat = false;
    let s = pass.to_string();
    if s.len() != 6 {
        return false;
    }
    let mut iter = s.chars();
    let mut prev = iter.next().unwrap().to_digit(10 as u32).unwrap();
    for c in iter {
        let digit = c.to_digit(10 as u32).unwrap();
        if digit < prev {
            return false;
        }
        if digit == prev {
            repeat = true;
        }
        prev = digit;
    }
    repeat
}

fn valid_password2(pass: i32) -> bool {
    let mut repeat = false;
    let s = pass.to_string();
    if s.len() != 6 {
        return false;
    }
    let mut iter = s.chars();
    let mut prev = iter.next().unwrap().to_digit(10 as u32).unwrap();
    let mut count = 1;
    for c in iter {
        let digit = c.to_digit(10 as u32).unwrap();
        if digit < prev {
            return false;
        }
        if digit == prev {
            count += 1;
        } else {
            if count == 2 {
                repeat = true;
            }
            count = 1;
        }
        prev = digit;
    }
    repeat || count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(valid_password(111111));
    }

    #[test]
    fn test_invalid_decreasing() {
        assert!(!valid_password(223450));
    }

    #[test]
    fn test_invalid_no_repeat() {
        assert!(!valid_password(123789));
    }

    #[test]
    fn test_valid2() {
        assert!(valid_password2(112233));
    }

    #[test]
    fn test_invalid_decreasing2() {
        assert!(!valid_password2(123444));
    }

    #[test]
    fn test_valid2_with_larger_repeat() {
        assert!(valid_password2(111122));
    }

    #[test]
    fn test_valid2_with_repeat_start() {
        assert!(valid_password2(112345));
    }

    #[test]
    fn test_part1() {
        assert_eq!(460, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(290, part2());
    }

    #[test]
    fn test_part1_stream() {
        assert_eq!(460, part1_stream());
    }

    #[test]
    fn test_part2_stream() {
        assert_eq!(290, part2_stream());
    }
}