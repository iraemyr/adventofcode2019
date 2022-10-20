pub struct Intcode {
    pc: usize,
    program: Vec<i32>,
}

impl Intcode {
    pub fn intcode_instance(p: Vec<i32>) -> Self {
        Self { pc: 0, program: p }
    }

    pub fn run(&mut self) -> () {
        loop {
            let op = self.program[self.pc];
            match op {
                1 | 2 => self.three_args(op),
                99 => break,
                _ => println!(r#"Invalid opcode"#),
            }
        }
    }

    fn three_args(&mut self, op: i32) -> () {
        self.pc += 1;
        let arg1 = self.read(self.program[self.pc]);
        self.pc += 1;
        let arg2 = self.read(self.program[self.pc]);
        self.pc += 1;
        let dest = self.program[self.pc];
        self.pc += 1;
        match op {
            1 => self.add(arg1, arg2, dest),
            2 => self.mul(arg1, arg2, dest),
            _ => println!("Unsupported operation"),
        }
    }

    fn add(&mut self, arg1: i32, arg2: i32, dest: i32) -> () {
        self.write(arg1 + arg2, dest);
    }

    fn mul(&mut self, arg1: i32, arg2: i32, dest: i32) -> () {
        self.write(arg1 * arg2, dest);
    }

    pub fn debug(&self) -> () {
        for i in &self.program {
            println!("{}", i);
        }
    }

    pub fn restore(&mut self) -> () {
        self.program[1] = 12;
        self.program[2] = 2;
    }

    pub fn write(&mut self, val: i32, pos: i32) -> () {
        self.program[pos as usize] = val;
    }

    pub fn read(&self, pos: i32) -> i32 {
        self.program[pos as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Intcode as Computer;

    #[test]
    fn simple_add() {
        let v = vec![1, 0, 0, 0, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert_eq!(2, comp.read(0));
    }

    #[test]
    fn simple_mul() {
        let v = vec![2, 3, 0, 3, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert_eq!(6, comp.read(3));
    }

    #[test]
    fn simple_mul2() {
        let v = vec![2, 4, 4, 5, 99, 0];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert_eq!(9801, comp.read(5));
    }

    #[test]
    fn compound() {
        let v = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert!(comp.read(0) == 30 && comp.read(4) == 2);
    }
}
