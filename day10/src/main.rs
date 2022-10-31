use array2d::Array2D;
use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone(), 21)); // 221 at (11, 11)
    println!("{}", part2(contents, 21, (11, 11))); // 806
}

fn part1(mut input: String, width: usize) -> i32 {
    input.retain(|c| !c.is_whitespace());
    let map: Array2D<bool> =
        Array2D::from_iter_row_major(input.into_bytes().iter().map(|b| *b != 46_u8), width, width);
    let mut max = i32::MIN;
    let mut best_pos = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for row in 0..(map.num_rows()) {
        for col in 0..(map.num_columns()) {
            let pos = (row, col);
            if map[pos] {
                let n = can_see(pos, &map, &mut set);
                if n > max {
                    max = n;
                    best_pos = pos;
                }
                set.clear();
            }
        }
    }
    println!("{:?}", best_pos);
    max
}

fn part2(mut input: String, width: usize, base: (usize, usize)) -> i32 {
    let mut angles: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    input.retain(|c| !c.is_whitespace());
    let map: Array2D<bool> =
        Array2D::from_iter_row_major(input.into_bytes().iter().map(|b| *b != 46_u8), width, width);
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            if map[(row, col)] && (row, col) != base {
                let diff = (col as i32 - base.0 as i32, row as i32 - base.1 as i32);
                let li = angles
                    .entry((calc_angle(diff.0, diff.1) * 100.0) as i32)
                    .or_default();
                li.push((col as i32, row as i32));
            }
        }
    }
    let mut keys: Vec<&i32> = angles.keys().collect();
    keys.sort();
    let p = angles.get(keys.get(199).unwrap()).unwrap().first().unwrap();
    p.0 * 100 + p.1
}

fn calc_angle(x: i32, y: i32) -> f32 {
    let f = y as f32;
    let angle = f.atan2(x as f32).to_degrees();
    if x < 0 && y < 0 {
        return angle + 450.0;
    }
    angle + 90.0
}

fn can_see(pos: (usize, usize), map: &Array2D<bool>, set: &mut HashSet<(i32, i32)>) -> i32 {
    for row in 0..(map.num_rows()) {
        for col in 0..(map.num_columns()) {
            if map[(row, col)] {
                let mut diff = (row as i32 - pos.0 as i32, col as i32 - pos.1 as i32);
                if diff != (0, 0) {
                    let gcd = gcd(diff.0, diff.1);
                    diff.0 /= gcd;
                    diff.1 /= gcd;
                    set.insert(diff);
                }
            }
        }
    }
    set.len() as i32
}

pub fn print_map(map: Array2D<bool>) {
    for row_iter in map.rows_iter() {
        for element in row_iter {
            if *element {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_part1() {
        let contents = fs::read_to_string("smallinput.txt").expect("File not found");
        assert_eq!(part1(contents, 5), 8);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents, 21), 221);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part2(contents, 21, (11, 11)), 806);
    }

    #[test]
    fn test_angle() {
        assert_eq!(calc_angle(0, -1), 0.0);
        assert_eq!(calc_angle(0, 1), 180.0);
        assert_eq!(calc_angle(1, 0), 90.0);
        assert_eq!(calc_angle(-1, 0), 270.0);
        assert_eq!(calc_angle(1, -2), 26.565048);
        assert_eq!(calc_angle(1, 2), 153.43495);
        assert_eq!(calc_angle(-1, -2), 333.43494);
        assert_eq!(calc_angle(-1, 2), 206.56505);
        assert_eq!(calc_angle(-2, -1), 296.56506);
    }
}
