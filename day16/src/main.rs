use rayon::prelude::*;
use std::fs;
use std::iter::repeat;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let v: Vec<i32> = contents
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    println!("{}", part1(v.clone())); // 32002835
    println!("{}", part2(v)); // 69732268
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

fn part1(mut v: Vec<i32>) -> i32 {
    for _ in 0..100_usize {
        v = fft(v);
    }
    v[0..8].iter().fold(0, |accum, i| accum * 10 + *i)
}

fn part2(v: Vec<i32>) -> i32 {
    let offset = v[0..7].iter().fold(0, |accum, i| accum * 10 + *i) as usize;
    let v_big = v.repeat(10_000);
    let mut trunc = v_big[offset..].to_vec();
    for _ in 0..100 {
        let mut acc = 0;
        for index in (0..trunc.len()).rev() {
            acc += trunc[index];
            trunc[index] = acc % 10;
        }
    }
    trunc[0..8].iter().fold(0, |accum, i| accum * 10 + *i)
}

fn fft(v: Vec<i32>) -> Vec<i32> {
    (0..v.len())
        .into_par_iter()
        .map(|index| {
            v.iter()
                .zip(pattern(index))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part1() {
        let digits = "80871224585914546619083218645595";
        let d = get_digits(digits);
        assert_eq!(part1(d), 24176176);
    }

    #[test]
    fn test_simple2_part1() {
        let digits = "19617804207202209144916044189917";
        let d = get_digits(digits);
        assert_eq!(part1(d), 73745418);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let v: Vec<i32> = contents
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        assert_eq!(part1(v), 32002835);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let v: Vec<i32> = contents
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        assert_eq!(part2(v), 69732268);
    }

    fn get_digits(s: &str) -> Vec<i32> {
        s.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }
}
