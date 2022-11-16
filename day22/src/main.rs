use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents)); //2519
}

fn part1(s: String) -> usize {
    let mut arr: [u32; 10_007] = core::array::from_fn(|i| i as u32);

    for line in s.lines() {
        if line.starts_with("deal") {
            let fields: Vec<&str> = line.split(' ').collect();
            if fields[1] == "into" {
                arr.reverse();
            } else {
                arr = deal_increment(arr, fields[3].parse::<usize>().unwrap());
            }
        } else {
            let fields: Vec<&str> = line.split(' ').collect();
            let num = fields[1].parse::<i32>().unwrap();
            if num >= 0 {
                arr.rotate_left(num as usize);
            } else {
                arr.rotate_right(num.unsigned_abs() as usize);
            }
        }
    }
    for (i, x) in arr.into_iter().enumerate() {
        if x == 2019 {
            return i;
        }
    }
    0
}

fn deal_increment(arr: [u32; 10_007], inc: usize) -> [u32; 10_007] {
    let mut result = [0; 10_007];
    let mut index = 0;
    for i in arr {
        result[index] = i;
        index += inc;
        index %= 10_007;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 2519);
    }
}
