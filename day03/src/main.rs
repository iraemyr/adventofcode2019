use std::cmp;
use std::collections::HashMap;
use std::fs;

const UP: (i32, i32) = (0, 1);
const RIGHT: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let result = part1_and_2(contents);
    println!("{}", result.0); // 352
    println!("{}", result.1); // 43848
}

fn part1_and_2(input: String) -> (i32, i32) {
    let mut lines = input.lines();
    let wire1 = lines.next().unwrap().split(',');
    let mut wire_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    let mut steps = 0;
    for dir in wire1 {
        let direction = get_delta(&dir[..1]);
        let length = dir[1..].parse::<i32>().unwrap();
        for _ in 0..length {
            steps += 1;
            pos.0 += direction.0;
            pos.1 += direction.1;
            if pos != (0, 0) {
                wire_map.entry(pos).or_insert(steps);
            }
        }
    }

    let wire2 = lines.next().unwrap().split(',');
    pos = (0, 0);
    steps = 0;
    let mut min = i32::MAX;
    let mut min_steps = i32::MAX;
    for dir in wire2 {
        let direction = get_delta(&dir[..1]);
        let length = dir[1..].parse::<i32>().unwrap();
        for _ in 0..length {
            steps += 1;
            pos.0 += direction.0;
            pos.1 += direction.1;
            if pos != (0, 0) {
                if let Some(x) = wire_map.get(&pos) {
                    min = cmp::min(min, manhattan(pos));
                    min_steps = cmp::min(min_steps, x + steps);
                }
            }
        }
    }
    (min, min_steps)
}

fn get_delta(direction: &str) -> (i32, i32) {
    match direction {
        "U" => UP,
        "R" => RIGHT,
        "D" => DOWN,
        "L" => LEFT,
        _ => {
            println!("Unknown direction");
            (0, 0)
        }
    }
}

fn manhattan(coord: (i32, i32)) -> i32 {
    coord.0.abs() + coord.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1_and_2(contents).0, 352);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1_and_2(contents).1, 43848);
    }
}
