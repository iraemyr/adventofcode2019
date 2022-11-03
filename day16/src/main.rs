use std::fs;
use std::iter::repeat;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let mut v: Vec<i32> = contents
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    //println!("{:?}", v);
    v = part1(v);
    println!("{:?}", v); // 32002835
                         //println!("{}", part1());
}

fn pattern(index: usize) -> impl Iterator<Item = i32> + Clone {
    repeat(0)
        .take(index + 1)
        .chain(repeat(1).take(index + 1))
        .chain(repeat(0).take(index + 1))
        .chain(repeat(-1).take(index + 1))
        .cycle()
        .skip(1)
}

fn part1(mut v: Vec<i32>) -> Vec<i32> {
    for _ in 0..100_usize {
        v = fft(v);
    }
    let s: String = v[0..8].iter().map(|a| a.to_string()).collect();
    println!("{}", s);
    v.into_iter().take(8).collect()
}

fn fft(v: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::<i32>::with_capacity(v.len());
    for index in 0..v.len() {
        output.push(
            v.iter()
                .zip(pattern(index))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .abs()
                % 10,
        );
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part1() {
        let digits = "80871224585914546619083218645595";
        let d = get_digits(digits);
        assert_eq!(part1(d), vec![2, 4, 1, 7, 6, 1, 7, 6]);
    }

    #[test]
    fn test_simple2_part1() {
        let digits = "19617804207202209144916044189917";
        let d = get_digits(digits);
        assert_eq!(part1(d), vec![7, 3, 7, 4, 5, 4, 1, 8]);
    }

    fn get_digits(s: &str) -> Vec<i32> {
        s.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }
}
