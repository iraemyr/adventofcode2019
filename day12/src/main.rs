use num::integer::lcm;
use std::cmp::Ordering;

#[derive(Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
    original_position: (i32, i32, i32),
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
            original_position: (x, y, z),
        }
    }

    fn gravity(&mut self, position: (i32, i32, i32)) {
        match self.position.0.cmp(&position.0) {
            Ordering::Less => self.velocity.0 += 1,
            Ordering::Greater => self.velocity.0 -= 1,
            _ => {}
        };

        match self.position.1.cmp(&position.1) {
            Ordering::Less => self.velocity.1 += 1,
            Ordering::Greater => self.velocity.1 -= 1,
            _ => {}
        };

        match self.position.2.cmp(&position.2) {
            Ordering::Less => self.velocity.2 += 1,
            Ordering::Greater => self.velocity.2 -= 1,
            _ => {}
        };
    }

    fn go(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn energy(&self) -> i32 {
        (self.position.0.abs() + self.position.1.abs() + self.position.2.abs())
            * (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs())
    }

    fn is_x_orig(&self) -> bool {
        self.position.0 == self.original_position.0 && self.velocity.0 == 0
    }

    fn is_y_orig(&self) -> bool {
        self.position.1 == self.original_position.1 && self.velocity.1 == 0
    }

    fn is_z_orig(&self) -> bool {
        self.position.2 == self.original_position.2 && self.velocity.2 == 0
    }

    pub fn load_moons() -> Vec<Moon> {
        let mut moons: Vec<Moon> = Vec::with_capacity(4);
        let mut moon = Moon::new(13, -13, -2);
        moons.push(moon);
        moon = Moon::new(16, 2, -15);
        moons.push(moon);
        moon = Moon::new(7, -18, -12);
        moons.push(moon);
        moon = Moon::new(-3, -8, -8);
        moons.push(moon);
        moons
    }
}

fn main() {
    let mut moons = Moon::load_moons();
    println!("{}", part1(&mut moons, 1000)); // 12082
    moons = Moon::load_moons();
    println!("{}", part2(&mut moons)); // 295693702908636
}

fn part1(moons: &mut Vec<Moon>, steps: i32) -> i32 {
    step(moons, steps);
    let mut total_energy = 0;
    for moon in moons {
        total_energy += moon.energy();
    }
    total_energy
}

fn part2(moons: &mut [Moon]) -> u64 {
    let mut x_period = 0_u64;
    let mut y_period = 0_u64;
    let mut z_period = 0_u64;
    let mut steps = 0_u64;

    while x_period == 0 || y_period == 0 || z_period == 0 {
        step(moons, 1);
        steps += 1;
        if x_period == 0 && moons.iter().filter(|m| m.is_x_orig()).count() == 4 {
            x_period = steps;
        }
        if y_period == 0 && moons.iter().filter(|m| m.is_y_orig()).count() == 4 {
            y_period = steps;
        }
        if z_period == 0 && moons.iter().filter(|m| m.is_z_orig()).count() == 4 {
            z_period = steps;
        }
    }

    lcm(lcm(x_period, y_period), z_period)
}

fn step(moons: &mut [Moon], steps: i32) {
    for _ in 0..steps {
        let positions = moons.iter().map(|moon| moon.position).collect::<Vec<_>>();

        for moon1 in moons.iter_mut() {
            for moon2 in positions.iter() {
                moon1.gravity(*moon2);
            }
        }

        for moon in moons.iter_mut() {
            moon.go();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_moons() -> Vec<Moon> {
        let mut moons: Vec<Moon> = Vec::with_capacity(4);
        let mut moon = Moon::new(-1, 0, 2);
        moons.push(moon);
        moon = Moon::new(2, -10, -7);
        moons.push(moon);
        moon = Moon::new(4, -8, 8);
        moons.push(moon);
        moon = Moon::new(3, 5, -1);
        moons.push(moon);
        moons
    }

    fn load_test_moons2() -> Vec<Moon> {
        let mut moons: Vec<Moon> = Vec::with_capacity(4);
        let mut moon = Moon::new(-8, -10, 0);
        moons.push(moon);
        moon = Moon::new(5, 5, 10);
        moons.push(moon);
        moon = Moon::new(2, -7, 3);
        moons.push(moon);
        moon = Moon::new(9, -8, -3);
        moons.push(moon);
        moons
    }
    #[test]
    fn test_part1() {
        let mut moons = Moon::load_moons();
        assert_eq!(part1(&mut moons, 1000), 12082);
    }

    #[test]
    fn test_part1_simple() {
        let mut moons = load_test_moons();
        assert_eq!(part1(&mut moons, 10), 179);
    }

    #[test]
    fn test_part2_simple() {
        let mut moons = load_test_moons();
        assert_eq!(part2(&mut moons), 2772);
    }

    #[test]
    fn test_part2_hard() {
        let mut moons = load_test_moons2();
        assert_eq!(part2(&mut moons), 4686774924);
    }

    #[test]
    fn test_part() {
        let mut moons = Moon::load_moons();
        assert_eq!(part2(&mut moons), 295693702908636);
    }
}
