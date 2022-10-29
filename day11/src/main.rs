use int_code_computer::Intcode;
use std::collections::HashMap;
use std::fs;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Robot {
    pos: (i32, i32),
    direction: Direction,
    hull: HashMap<(i32, i32), bool>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            pos: (0, 0),
            direction: Direction::Up,
            hull: HashMap::new(),
            x_min: i32::MAX,
            y_min: i32::MAX,
            x_max: i32::MIN,
            y_max: i32::MIN,
        }
    }

    fn turn(&mut self, d: i32) {
        match self.direction {
            Direction::Up => {
                if d == 0 {
                    self.direction = Direction::Left;
                } else {
                    self.direction = Direction::Right;
                }
            }
            Direction::Right => {
                if d == 0 {
                    self.direction = Direction::Up;
                } else {
                    self.direction = Direction::Down;
                }
            }
            Direction::Down => {
                if d == 0 {
                    self.direction = Direction::Right;
                } else {
                    self.direction = Direction::Left;
                }
            }
            Direction::Left => {
                if d == 0 {
                    self.direction = Direction::Down;
                } else {
                    self.direction = Direction::Up;
                }
            }
        }
    }

    fn go(&mut self) {
        let delta = match self.direction {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
        };
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }

    fn get_color(&self) -> i32 {
        if self.hull.contains_key(&self.pos) && *self.hull.get(&self.pos).unwrap() {
            1
        } else {
            0
        }
    }

    fn paint(&mut self, color: i32) {
        let mut c = false;
        if color == 1 {
            c = true
        }
        self.hull.insert(self.pos, c);
        self.x_min = self.x_min.min(self.pos.0);
        self.x_max = self.x_max.max(self.pos.0);
        self.y_min = self.y_min.min(self.pos.1);
        self.y_max = self.y_max.max(self.pos.1);
    }

    fn print(&self) {
        for y in (self.y_min..=self.y_max).rev() {
            for x in self.x_min..=self.x_max {
                let c = match self.hull.get(&(x, y)) {
                    Some(b) => {
                        if *b {
                            '*'
                        } else {
                            ' '
                        }
                    }
                    None => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", part1(contents.clone())); // 1932
    part2(contents); // EGHKGJER
}

fn part1(program: String) -> i32 {
    let mut comp = Intcode::intcode_instance(program);
    let mut robot = Robot::new();
    while !comp.is_halted() {
        comp.set_input(robot.get_color() as i64);
        comp.run();
        robot.paint(comp.get_output() as i32);
        robot.turn(comp.get_output() as i32);
        robot.go();
    }
    robot.hull.len() as i32
}

fn part2(program: String) {
    let mut comp = Intcode::intcode_instance(program);
    let mut robot = Robot::new();
    robot.paint(1);
    while !comp.is_halted() {
        comp.set_input(robot.get_color() as i64);
        comp.run();
        robot.paint(comp.get_output() as i32);
        robot.turn(comp.get_output() as i32);
        robot.go();
    }
    robot.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        assert_eq!(part1(contents), 1932);
    }
}
