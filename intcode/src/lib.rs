pub struct Intcode {
    pc: usize,
    program: Vec<i32>,
    input: i32,
    input_ready: bool,
    output: i32,
}

impl Intcode {
    pub fn intcode_instance(p: Vec<i32>) -> Self {
        Self {
            pc: 0,
            program: p,
            input: 0,
            input_ready: false,
            output: 0,
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            let instr = self.program[self.pc];
            let padded: String = format!("{instr:05}");
            let op = padded[3..].to_owned().parse::<i32>().unwrap();
            let mut modes = padded[..3].to_owned();

            match op {
                1 | 2 | 7 | 8 => self.three_args(op, &mut modes),
                3 => {
                    if self.input_ready {
                        self.one_arg(3, modes.pop().unwrap());
                    } else {
                        return false;
                    }
                }
                4 => self.one_arg(4, modes.pop().unwrap()),
                5 | 6 => self.two_args(op, &mut modes),
                99 => return true,
                _ => println!(r#"Invalid opcode"#),
            }
        }
    }

    fn three_args(&mut self, op: i32, modes: &mut String) {
        self.pc += 1;
        let mut mode = modes.pop().unwrap();
        let arg1 = self.get_value(self.program[self.pc], mode);
        self.pc += 1;
        mode = modes.pop().unwrap();
        let arg2 = self.get_value(self.program[self.pc], mode);
        self.pc += 1;
        let dest = self.program[self.pc];
        self.pc += 1;
        match op {
            1 => self.add(arg1, arg2, dest),
            2 => self.mul(arg1, arg2, dest),
            7 => self.less_than(arg1, arg2, dest),
            8 => self.equals(arg1, arg2, dest),
            _ => println!("Unsupported operation"),
        }
    }

    fn two_args(&mut self, op: i32, modes: &mut String) {
        self.pc += 1;
        let mut mode = modes.pop().unwrap();
        let arg1 = self.get_value(self.program[self.pc], mode);
        self.pc += 1;
        mode = modes.pop().unwrap();
        let arg2 = self.get_value(self.program[self.pc], mode);
        self.pc += 1;
        match op {
            5 => self.jump_true(arg1, arg2),
            6 => self.jump_false(arg1, arg2),
            _ => println!("Unsupported operation"),
        }
    }

    fn one_arg(&mut self, op: i32, mode: char) {
        self.pc += 1;
        let arg = self.program[self.pc];
        self.pc += 1;
        match op {
            3 => self.get_input(arg),
            4 => {
                let val = self.get_value(arg, mode);
                self.set_output(val);
            }
            _ => println!("Unsupported operation"),
        }
    }

    fn get_value(&mut self, arg: i32, mode: char) -> i32 {
        match mode {
            '0' => self.read(arg),
            '1' => arg,
            _ => panic!("Invalid mode"),
        }
    }

    fn add(&mut self, arg1: i32, arg2: i32, dest: i32) {
        self.write(arg1 + arg2, dest);
    }

    fn mul(&mut self, arg1: i32, arg2: i32, dest: i32) {
        self.write(arg1 * arg2, dest);
    }

    fn less_than(&mut self, arg1: i32, arg2: i32, dest: i32) {
        if arg1 < arg2 {
            self.write(1, dest);
        } else {
            self.write(0, dest);
        }
    }

    fn equals(&mut self, arg1: i32, arg2: i32, dest: i32) {
        if arg1 == arg2 {
            self.write(1, dest);
        } else {
            self.write(0, dest);
        }
    }

    fn jump_true(&mut self, arg1: i32, arg2: i32) {
        if arg1 != 0 {
            self.pc = arg2 as usize;
        }
    }

    fn jump_false(&mut self, arg1: i32, arg2: i32) {
        if arg1 == 0 {
            self.pc = arg2 as usize;
        }
    }

    fn get_input(&mut self, dest: i32) {
        self.write(self.input, dest);
        self.input_ready = false;
    }

    fn set_output(&mut self, arg: i32) {
        self.output = arg;
    }

    pub fn debug(&self) {
        for i in &self.program {
            println!("{}", i);
        }
    }

    pub fn restore(&mut self) {
        self.program[1] = 12;
        self.program[2] = 2;
    }

    pub fn write(&mut self, val: i32, pos: i32) {
        self.program[pos as usize] = val;
    }

    pub fn read(&self, pos: i32) -> i32 {
        self.program[pos as usize]
    }

    pub fn set_input(&mut self, data: i32) {
        self.input = data;
        self.input_ready = true;
    }

    pub fn get_output(&mut self) -> i32 {
        self.output
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

    #[test]
    fn io() {
        let v = vec![3, 0, 4, 0, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(44);
        comp.run();
        assert_eq!(44, comp.get_output());
    }

    #[test]
    fn mul_modes() {
        let v = vec![1002, 4, 3, 4, 33];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert_eq!(comp.read(4), 99);
    }

    #[test]
    fn negative() {
        let v = vec![1101, 100, -1, 4, 0];
        let mut comp = Computer::intcode_instance(v);
        comp.run();
        assert_eq!(comp.read(4), 99);
    }

    #[test]
    fn compare_eq_position_mode() {
        let v = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(8);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_eq_immediate_mode() {
        let v = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(8);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_less_position_mode() {
        let v = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(6);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_less_immediate_mode() {
        let v = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(6);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn jump_position_mode() {
        let v = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(0);
        comp.run();
        assert_eq!(comp.get_output(), 0);
    }

    #[test]
    fn jump_immediate_mode() {
        let v = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut comp = Computer::intcode_instance(v);
        comp.set_input(0);
        comp.run();
        assert_eq!(comp.get_output(), 0);
    }
}
