//! ICWS '94 draft
//!

use std::ops::{Index, IndexMut};


// #[derive(Clone, Copy, Debug, Default)]
// pub struct Address(u32);
pub type Address = u32;

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
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
pub enum Modifier {
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    IMMEDIATE, // #
    DIRECT, // $ (Default)
    INDIRECT, // @
    DECREMENT, // <
    INCREMENT, // >
}

impl Default for Mode {
    fn default() -> Self {
        Mode::IMMEDIATE
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Instruction {
    pub opcode: Opcode,
    pub modifier: Modifier,
    pub a_mode: Mode,
    pub a_number: Address,
    pub b_mode: Mode,
    pub b_number: Address,
}

impl Instruction {
    pub fn random() -> Self {
        Instruction::default()  // XXX Need real random implementation
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Warrior(u32);

#[derive(Debug)]
pub struct TaskQueue {
    task_limit: usize,
    task_pointer: Vec<Address>,
}

impl TaskQueue {
    fn new(limit: usize) -> Self {
        TaskQueue {
            task_limit: limit,
            task_pointer: Vec::<Address>::with_capacity(limit),
        }
    }

    pub fn queue(&mut self, addr: Address) {
        if self.task_pointer.len() < self.task_limit {
            self.task_pointer.push(addr);
        }
    }

    pub fn next(&mut self) -> Option<Address> {
        if self.task_pointer.len() > 0 {
            Some(self.task_pointer.remove(0))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CoreMemory(Vec<Instruction>);

impl Index<Address> for CoreMemory {
    type Output = Instruction;

    fn index<'a>(&'a self, _index: Address) -> &'a Instruction {
        &self.0[_index as usize]
    }
}

impl IndexMut<Address> for CoreMemory {
    fn index_mut<'a>(&'a mut self, _index: Address) -> &'a mut Instruction {
        &mut self.0[_index as usize]
    }
}

#[derive(Debug)]
pub struct Core {
    core_size: Address,
    core: CoreMemory,
    warrior: [TaskQueue; 2],
    pc: Address,
    read_limit: Address,
    write_limit: Address,
}

impl Core {
    pub fn queue(&mut self, warrior: usize, task_pointer: Address) {
        self.warrior[warrior].queue(task_pointer)
    }

    fn fold(&self, pointer: Address, limit: Address) -> Address {
        let mut result = pointer % limit;
        if result > limit / 2 {
            result += self.core_size - limit;
        }
        result
    }

    pub fn emi94(&mut self) {
        let ir: Instruction;
        let ira: Instruction;
        let irb: Instruction;
        let mut rpa: Address;
        let mut wpa: Address;
        let rpb: Address;
        let wpb: Address;
        let pip: Address;

        ir = self.core[self.pc];

        // Evaluate A-operand
        //
        if ir.a_mode == Mode::IMMEDIATE {
            // For instructions with an Immediate A-mode, the Pointer A
            // points to the source of the current instruction.
            rpa = 0;
            wpa = 0;
        } else {
            // For instructions with a Direct A-mode, the Pointer A
            // points to the instruction IR.ANumber away, relative to
            // the Program Counter.
            rpa = self.fold(ir.a_number, self.read_limit);
            wpa = self.fold(ir.a_number, self.write_limit);

            if ir.a_mode != Mode::DIRECT {
                if ir.a_mode == Mode::DECREMENT {
                    self.core[((self.pc + wpa) % self.core_size)].b_number =
                        (self.core[((self.pc + wpa) % self.core_size)].b_number +
                         self.core_size - 1) % self.core_size;
                }

                if ir.a_mode == Mode::INCREMENT {
                    pip = (self.pc + wpa) % self.core_size;
                }

                rpa = self.fold((rpa + self.core[((self.pc + rpa) % self.core_size)].b_number),
                                self.read_limit);
                wpa = self.fold((wpa + self.core[((self.pc + wpa) % self.core_size)].b_number),
                                self.write_limit);
            }
        }

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
            core: CoreMemory(instructions),
            warrior: [TaskQueue::new(self.task_limit), TaskQueue::new(self.task_limit)],
            pc: 0,
            read_limit: 300,
            write_limit: 300,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_queue_simple() {
        let mut tq = TaskQueue::new(2);
        tq.queue(1);
        tq.queue(3);
        assert_eq!(tq.next(), Some(1));
        assert_eq!(tq.next(), Some(3));
        assert_eq!(tq.next(), None);
    }

    #[test]
    fn task_queue_limit() {
        let mut tq = TaskQueue::new(3);
        tq.queue(100);
        tq.queue(200);
        tq.queue(300);
        tq.queue(400);

        assert_eq!(tq.next(), Some(100));
        assert_eq!(tq.next(), Some(200));
        assert_eq!(tq.next(), Some(300));
        assert_eq!(tq.next(), None);
    }

    #[test]
    fn task_queue_empty() {
        let mut tq = TaskQueue::new(100);

        assert_eq!(tq.next(), None);
    }
}
