use nohash_hasher::IntMap;
use std::collections::VecDeque;

pub struct Intcode {
    pc: usize,
    rb: usize,
    program: IntMap<usize, i64>,
    input: i64,
    input_ready: bool,
    output: VecDeque<i64>,
    halted: bool,
    debug: bool,
    orig_program: String,
}

fn parse_program(p: String) -> IntMap<usize, i64> {
    let program: IntMap<usize, i64> = p
        .split(',')
        .map(|a| a.trim().parse::<i64>().unwrap())
        .enumerate()
        .collect();
    program
}

impl Intcode {
    pub fn intcode_instance(p: String) -> Self {
        Self {
            pc: 0,
            rb: 0,
            program: parse_program(p.clone()),
            input: 0,
            input_ready: false,
            output: VecDeque::new(),
            halted: false,
            debug: false,
            orig_program: p,
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            let instr = self.program[&self.pc];
            if self.debug {
                println!("pc: {}, instr: {}", self.pc, instr);
            }
            let padded: String = format!("{instr:05}");
            let op = padded[3..].to_owned().parse::<i32>().unwrap();
            let mut modes = padded[..3].to_owned();

            match op {
                1 | 2 | 7 | 8 => self.three_args(op, &mut modes),
                3 => {
                    if self.input_ready {
                        self.one_arg(op, modes.pop().unwrap());
                    } else {
                        return false;
                    }
                }
                4 | 9 => self.one_arg(op, modes.pop().unwrap()),
                5 | 6 => self.two_args(op, &mut modes),
                99 => {
                    if self.debug {
                        println!("Halt");
                    }
                    self.halted = true;
                    return true;
                }
                _ => println!(r#"Invalid opcode"#),
            }
        }
    }

    fn three_args(&mut self, op: i32, modes: &mut String) {
        self.pc += 1;
        let mut mode = modes.pop().unwrap();
        let arg1 = self.get_value(self.program[&self.pc], mode);
        self.pc += 1;
        mode = modes.pop().unwrap();
        let arg2 = self.get_value(self.program[&self.pc], mode);
        self.pc += 1;
        mode = modes.pop().unwrap();
        let mut dest = self.program[&self.pc];
        dest = self.get_dest(dest, mode);
        self.pc += 1;
        if self.debug {
            println!("op: {}, arg1: {}, arg2: {}, dest: {}", op, arg1, arg2, dest);
        }
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
        let arg1 = self.get_value(self.program[&self.pc], mode);
        self.pc += 1;
        mode = modes.pop().unwrap();
        let arg2 = self.get_value(self.program[&self.pc], mode);
        self.pc += 1;
        if self.debug {
            println!("op: {}, arg1: {}, arg2: {}", op, arg1, arg2);
        }
        match op {
            5 => self.jump_true(arg1, arg2),
            6 => self.jump_false(arg1, arg2),
            _ => println!("Unsupported operation"),
        }
    }

    fn one_arg(&mut self, op: i32, mode: char) {
        self.pc += 1;
        let arg = self.program[&self.pc];
        self.pc += 1;
        if self.debug {
            println!("op: {}, arg: {}, mode: {}", op, arg, mode);
        }
        match op {
            3 => {
                let dest: i64 = self.get_dest(arg, mode);
                self.get_input(dest);
            }
            4 => {
                let val = self.get_value(arg, mode);
                self.set_output(val);
            }
            9 => {
                let v = self.get_value(arg, mode);
                self.adjust_rb(v)
            }
            _ => println!("Unsupported operation"),
        }
    }

    fn get_value(&mut self, arg: i64, mode: char) -> i64 {
        match mode {
            '0' => self.read(arg as usize),
            '1' => arg,
            '2' => {
                let tmp = self.rb as i64 + arg;
                if self.debug {
                    let result = self.read(tmp as usize);
                    println!(
                        "  rb: {}, arg: {} -> rb: {} = {}",
                        self.rb, arg, tmp, result
                    );
                }
                self.read(tmp as usize)
            }
            _ => panic!("Invalid mode"),
        }
    }

    fn get_dest(&mut self, arg: i64, mode: char) -> i64 {
        match mode {
            '0' | '1' => arg,
            '2' => self.rb as i64 + arg,
            _ => {
                println!("Unsupported mode for destination");
                -1
            }
        }
    }

    fn add(&mut self, arg1: i64, arg2: i64, dest: i64) {
        if self.debug {
            println!("{} + {} = {} -> {}\n", arg1, arg2, arg1 + arg2, dest);
        }
        self.write(arg1 + arg2, dest);
    }

    fn mul(&mut self, arg1: i64, arg2: i64, dest: i64) {
        if self.debug {
            println!("{} * {} = {} -> {}\n", arg1, arg2, arg1 * arg2, dest);
        }
        self.write(arg1 * arg2, dest);
    }

    fn less_than(&mut self, arg1: i64, arg2: i64, dest: i64) {
        if self.debug {
            println!("{} < {} -> {}\n", arg1, arg2, dest);
        }
        if arg1 < arg2 {
            self.write(1, dest);
        } else {
            self.write(0, dest);
        }
    }

    fn equals(&mut self, arg1: i64, arg2: i64, dest: i64) {
        if self.debug {
            println!("{} == {} -> {}\n", arg1, arg2, dest);
        }
        if arg1 == arg2 {
            self.write(1, dest);
        } else {
            self.write(0, dest);
        }
    }

    fn jump_true(&mut self, arg1: i64, arg2: i64) {
        if self.debug {
            println!("{} != 0 -> jump to {}\n", arg1, arg2);
        }
        if arg1 != 0 {
            self.pc = arg2 as usize;
        }
    }

    fn jump_false(&mut self, arg1: i64, arg2: i64) {
        if self.debug {
            println!("{} == 0 -> jump to {}\n", arg1, arg2);
        }
        if arg1 == 0 {
            self.pc = arg2 as usize;
        }
    }

    fn get_input(&mut self, dest: i64) {
        self.write(self.input, dest);
        if self.debug {
            println!("input: {} -> {}\n", self.input, dest);
        }
        self.input_ready = false;
    }

    fn set_output(&mut self, arg: i64) {
        if self.debug {
            println!("output: {}\n", arg);
        }
        self.output.push_back(arg);
    }

    fn adjust_rb(&mut self, arg: i64) {
        let tmp = self.rb as i64 + arg;
        if self.debug {
            println!("    rb: {}, arg: {}, -> rb: {}\n", self.rb, arg, tmp);
        }
        self.rb = tmp as usize;
    }

    pub fn debug(&self) {
        for i in &self.program {
            println!("{}", i.1);
        }
    }

    pub fn restore(&mut self) {
        self.program.insert(1, 12);
        self.program.insert(2, 2);
    }

    pub fn write(&mut self, val: i64, pos: i64) {
        self.program.insert(pos as usize, val);
    }

    pub fn read(&mut self, pos: usize) -> i64 {
        *self.program.entry(pos).or_default()
    }

    pub fn set_input(&mut self, data: i64) {
        self.input = data;
        self.input_ready = true;
    }

    pub fn get_output(&mut self) -> i64 {
        self.output.pop_front().unwrap()
    }

    pub fn get_last_output(&mut self) -> i64 {
        self.output.pop_back().unwrap()
    }

    pub fn get_outputs(&mut self) -> Vec<i64> {
        let mut v: Vec<i64> = Vec::new();
        for n in &self.output {
            v.push(*n);
        }
        self.output.clear();
        v
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn set_debug(&mut self, b: bool) {
        self.debug = b;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.rb = 0;
        self.halted = false;
        self.program = parse_program(self.orig_program.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::Intcode as Computer;

    #[test]
    fn simple_add() {
        let s = "1, 0, 0, 0, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.read(0), 2);
    }

    #[test]
    fn simple_mul() {
        let s = "2, 3, 0, 3, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.read(3), 6);
    }

    #[test]
    fn simple_mul2() {
        let s = "2, 4, 4, 5, 99, 0".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.read(5), 9801);
    }

    #[test]
    fn compound() {
        let s = "1, 1, 1, 4, 99, 5, 6, 0, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert!(comp.read(0) == 30 && comp.read(4) == 2);
    }

    #[test]
    fn io() {
        let s = "3, 0, 4, 0, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(44);
        comp.run();
        assert_eq!(comp.get_output(), 44);
    }

    #[test]
    fn mul_modes() {
        let s = "1002, 4, 3, 4, 33".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.read(4), 99);
    }

    #[test]
    fn negative() {
        let s = "1101, 100, -1, 4, 0".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.read(4), 99);
    }

    #[test]
    fn compare_eq_position_mode() {
        let s = "3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(8);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_eq_immediate_mode() {
        let s = "3, 3, 1108, -1, 8, 3, 4, 3, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(8);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_less_position_mode() {
        let s = "3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(6);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn compare_less_immediate_mode() {
        let s = "3, 3, 1107, -1, 8, 3, 4, 3, 99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(6);
        comp.run();
        assert_eq!(comp.get_output(), 1);
    }

    #[test]
    fn jump_position_mode() {
        let s = "3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(0);
        comp.run();
        assert_eq!(comp.get_output(), 0);
    }

    #[test]
    fn jump_immediate_mode() {
        let s = "3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.set_input(0);
        comp.run();
        assert_eq!(comp.get_output(), 0);
    }

    #[test]
    fn test_large_number() {
        let s = "1102,34915192,34915192,7,4,7,99,0".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.get_output(), 1219070632396864);
    }

    #[test]
    fn test_copy_program() {
        let s = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        let result = comp.get_outputs();
        let ans = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(result, ans);
    }

    #[test]
    fn test_large_output() {
        let s = "104,1125899906842624,99".to_string();
        let mut comp = Computer::intcode_instance(s);
        comp.run();
        assert_eq!(comp.get_output(), 1125899906842624);
    }
}
