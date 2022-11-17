use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); //2519
    println!("{}", part2(contents)); // 58966729050483
}

fn part1(s: String) -> usize {
    let mut arr: [u32; 10_007] = core::array::from_fn(|i| i as u32);

    for line in s.lines() {
        let fields: Vec<&str> = line.split(' ').collect();
        if fields[0] == "deal" {
            if fields[1] == "into" {
                arr.reverse();
            } else {
                arr = deal_increment(arr, fields[3].parse::<usize>().unwrap());
            }
        } else {
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

fn part2(s: String) -> i128 {
    // Algorithm based on solution by ChrisVittal in adventofcode 2019 reddit post
    let size = 119315717514047;
    let iters = 101741582076661;
    let pos = 2020;
    let mut a = 1;
    let mut b = 0;
    for line in s.lines().rev() {
        let fields: Vec<&str> = line.split(' ').collect();
        if fields[0] == "deal" {
            if fields[1] == "into" {
                b = -(b + 1);
                a = -a;
            } else {
                let n = fields[3].parse::<i128>().unwrap();
                let inv = inv(n, size);
                a = a * inv % size;
                b = b * inv % size;
            }
        } else {
            let n = fields[1].parse::<i128>().unwrap();
            b += if n < 0 { n + size } else { n };
        }
        a %= size;
        b %= size;

        if a < 0 {
            a += size;
        }

        if b < 0 {
            b += size;
        }
    }
    let i1 = modp(a, iters, size) * pos % size;
    let i2 = (modp(a, iters, size) + size - 1) % size;
    let i3 = b * i2 % size;
    let i4 = inv(a - 1, size);
    (i1 + i3 * i4) % size
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
    let mut x = 1;
    let mut p = b % base;

    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % base;
        }
        p = p * p % base;
    }
    x
}

fn inv(a: i128, n: i128) -> i128 {
    modp(a, n - 2, n)
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

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents), 58966729050483);
    }
}
