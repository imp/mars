//! ICWS '94 draft
//!

use std::default::Default;

#[derive(Clone, Copy, Debug, Default)]
pub struct Warrior(u32);

// #[derive(Clone, Copy, Debug, Default)]
// pub struct Address(u32);
pub type Address = u32;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    DAT, // terminate process
    MOV, // move from A to B
    ADD, // add A to B, store result in B
    SUB, // subtract A from B, store result in B
    MUL, // multiply A by B, store result in B
    DIV, // divide B by A, store result in B if A <> 0, else terminate
    MOD, // divide B by A, store remainder in B if A <> 0, else terminate
    JMP, // transfer execution to A
    JMZ, // transfer execution to A if B is zero
    JMN, // transfer execution to A if B is non-zero
    DJN, // decrement B, if B is non-zero, transfer execution to A
    CMP, // same as SEQ
    SLT, // skip next instruction if A is less than B
    SPL, // split off process to A
    SEQ, // (*) Skip next instruction if A is equal to B
    SNE, // (*) Skip next instruction if A is not equal to B
    NOP, // (*) No operation
    LDP, // (+) Load P-space cell A into core address B
    STP, // (+) Store A-number into P-space cell B
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::DAT
    }
}

#[derive(Clone, Copy, Debug)]
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::A
    }
}
#[derive(Clone, Copy, Debug)]
enum Mode {
    IMMEDIATE, // #
    DIRECT, // $ (Default)
    INDIRECT, // @
    PREDECREMENT, // <
    POSTINCREMENT, // >
}

impl Default for Mode {
    fn default() -> Self {
        Mode::IMMEDIATE
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Instruction {
    opcode: Opcode,
    modifier: Modifier,
    a_mode: Mode,
    a_address: Address,
    b_mode: Mode,
    b_address: Address,
}

// impl Default for Instruction {
//     fn default() -> Self {
//         Instruction {
//             opcode: Opcode::DAT,
//             modifier: Modifier::F,
//             a_mode: Mode::IMMEDIATE,
//             a_address: 0,
//             b_mode: Mode::IMMEDIATE,
//             b_address: 0,
//         }
//     }
// }

impl Instruction {
    pub fn random() -> Self {
        Instruction::default()  // XXX Need real random implementation
    }
}

// #[derive(Debug)]
// struct TaskPointer(Address);

#[derive(Debug)]
pub struct TaskQueue {
    task_limit: usize,
    task_pointer: Vec<Address>,
}

impl TaskQueue {
    fn new(limit: usize) -> Self {
        TaskQueue {
            task_limit: limit,
            task_pointer: vec![0,],
        }
    }

    pub fn queue(&mut self, addr: Address) {
        self.task_pointer.push(addr);
    }
}

#[derive(Debug)]
pub struct Core {
    core_size: Address,
    instructions: Vec<Instruction>,
    warrior: [TaskQueue; 2],
    pc: Address,
}

impl Core {
    pub fn queue(&mut self, warrior: usize, task_pointer: Address) {
        self.warrior[warrior].queue(task_pointer)
    }

    pub fn fold(&self, pointer: Address, limit: Address) -> Address {
        let mut result;

        result = pointer % limit;
        if result > limit / 2 {
            result += self.core_size - limit;
        }
        result
    }
}

#[derive(Debug)]
pub enum InitialInstruction {
    Default,
    Random,
    Instruction(Instruction),
}

#[derive(Debug)]
pub struct CoreBuilder {
    core_size: u32,
    initial_instruction: InitialInstruction,
    task_limit: usize,
}

impl CoreBuilder {
    pub fn new() -> Self {
        CoreBuilder {
            core_size: 8192,
            initial_instruction: InitialInstruction::Default,
            task_limit: 64,
        }
    }

    pub fn coresize(&mut self, size: u32) -> &Self {
        self.core_size = size;
        self
    }

    pub fn initial_instruction(&mut self, insn: InitialInstruction) -> &Self {
        self.initial_instruction = insn;
        self
    }

    pub fn task_limit(&mut self, limit: usize) -> &Self {
        self.task_limit = limit;
        self
    }

    pub fn build(&self) -> Core {
        let mut instructions = Vec::<Instruction>::new();
        for _ in 0..self.core_size {
            instructions.push(match self.initial_instruction {
                InitialInstruction::Default => Instruction::default(),
                InitialInstruction::Random => Instruction::random(),
                InitialInstruction::Instruction(insn) => insn,
            })
        }

        Core {
            core_size: self.core_size,
            instructions: instructions,
            warrior: [TaskQueue::new(self.task_limit), TaskQueue::new(self.task_limit)],
            pc: 0,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
