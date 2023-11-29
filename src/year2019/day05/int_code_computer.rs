pub struct IntCodeComputer {
    program: Vec<i32>,
    instruction_pointer: usize,
    halt: bool,
    pub io: IO
}

pub struct IO {
    pub value: i32
}

impl IO {
    fn get(&self) -> i32 {
        self.value
    }

    fn set(&mut self, value: i32) {
        self.value = value;
    }
}

pub enum OptCode {
    Add = 1,
    Mul = 2,
    In = 3,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Out = 4,
    Halt = 99
}

impl From<i32> for OptCode {
    fn from(value: i32) -> Self {
        match value {
            1 => OptCode::Add,
            2 => OptCode::Mul,
            3 => OptCode::In,
            4 => OptCode::Out,
            5 => OptCode::JumpIfTrue,
            6 => OptCode::JumpIfFalse,
            7 => OptCode::LessThan,
            8 => OptCode::Equals,
            99 => OptCode::Halt,
            _ => unreachable!("Unexpected opcode")
        }
    }
}

pub enum ParameterMode {
    Position,
    Immediate
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!("Unexpected parameter mode"),
        }
    }
}



impl IntCodeComputer {
    pub fn new(program: Vec<i32>, io: IO) -> Self {
        Self {
            program,
            instruction_pointer: 0,
            halt: false,
            io,
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }

    fn step(&mut self) {
        let (opt_code, parameter_modes) = IntCodeComputer::decode_instruction(self.read_memory(self.instruction_pointer));
        match opt_code {
            OptCode::Add => self.add(&parameter_modes),
            OptCode::Mul => self.mul(&parameter_modes),
            OptCode::In => self.input(&parameter_modes),
            OptCode::Out => self.output(&parameter_modes),
            OptCode::JumpIfTrue => self.jump_if_true(&parameter_modes),
            OptCode::JumpIfFalse => self.jump_if_false(&parameter_modes),
            OptCode::LessThan => self.less_than(&parameter_modes),
            OptCode::Equals => self.equals(&parameter_modes),
            OptCode::Halt => self.halt(),
        }
    }

    fn add(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2, destination) = self.consume_3_int_codes(parameter_modes);
        self.write_memory(destination, operand1 + operand2);
        self.instruction_pointer += 4;
    }

    fn mul(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2, destination) = self.consume_3_int_codes(parameter_modes);
        self.write_memory(destination, operand1 * operand2);
        self.instruction_pointer += 4;
    }

    fn halt(&mut self) { self.halt = true; }

    fn input(&mut self, parameter_modes: &[ParameterMode]) {
        let destination = self.destination(0, parameter_modes);
        let value = self.io.get();
        self.write_memory(destination, value);
        self.instruction_pointer += 2;
    }

    fn output(&mut self, parameter_modes: &[ParameterMode]) {
        let param = self.parameter(0, parameter_modes);
        self.io.set(param);
        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2) = self.consume_2_int_codes(parameter_modes);
        self.instruction_pointer = if operand1 != 0 { operand2 as usize } else { self.instruction_pointer + 3 }
    }

    fn jump_if_false(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2) = self.consume_2_int_codes(parameter_modes);
        self.instruction_pointer = if operand1 == 0 { operand2 as usize } else { self.instruction_pointer + 3 }
    }

    fn less_than(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2, destination) = self.consume_3_int_codes(parameter_modes);
        self.write_memory(destination, if operand1 < operand2 { 1 } else { 0 });
        self.instruction_pointer += 4;
    }

    fn equals(&mut self, parameter_modes: &[ParameterMode]) {
        let (operand1, operand2, destination) = self.consume_3_int_codes(parameter_modes);
        self.write_memory(destination, if operand1 == operand2 { 1 } else { 0 });
        self.instruction_pointer += 4;
    }

    fn consume_3_int_codes(&self, parameter_modes: &[ParameterMode]) -> (i32, i32, i32) {
        let param1 = self.parameter(0, parameter_modes);
        let param2 = self.parameter(1, parameter_modes);
        let destination = self.destination(2, parameter_modes);

        (param1, param2, destination)
    }

    fn consume_2_int_codes(&self, parameter_modes: &[ParameterMode]) -> (i32, i32) {
        let param1 = self.parameter(0, parameter_modes);
        let param2 = self.parameter(1, parameter_modes);

        (param1, param2)
    }

    pub fn decode_instruction(instruction: i32) -> (OptCode, [ParameterMode; 3]) {
        let opt_code = instruction % 100;
        let param_modes = [
            ((instruction / 100) % 10).into(),
            ((instruction / 1000) % 10).into(),
            ((instruction / 10000) % 10).into()
        ];

        (opt_code.into(), param_modes)
    }

    fn read_memory(&self, index: usize) -> i32 {
        if index >= self.program.len() {
            0
        } else {
            self.program[index]
        }
    }

    fn parameter(&self, index: usize, parameter_mode: &[ParameterMode]) -> i32 {
        let param = self.read_memory(self.instruction_pointer + index + 1);

        match parameter_mode[index] {
            ParameterMode::Position => self.read_memory(param as usize),
            ParameterMode::Immediate => param
        }
    }

    fn write_memory(&mut self, index: i32, value: i32) {
        let index = index as usize;

        if index >= self.program.len() {
            self.program.resize(index + 1, 0);
        }

        self.program[index] = value;
    }

    fn destination(&self, index: usize, parameter_modes: &[ParameterMode]) -> i32 {
        let destination = self.read_memory(self.instruction_pointer + index + 1);
        match parameter_modes[index] {
            ParameterMode::Position => destination,
            ParameterMode::Immediate => unreachable!("Invalid mode for destination"),
        }
    }
}