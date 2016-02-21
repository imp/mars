//
// ICWS '94 draft
//

enum Opcode {
    DAT,    // terminate process
    MOV,    // move from A to B
    ADD,    // add A to B, store result in B
    SUB,    // subtract A from B, store result in B
    MUL,    // multiply A by B, store result in B
    DIV,    // divide B by A, store result in B if A <> 0, else terminate
    MOD,    // divide B by A, store remainder in B if A <> 0, else terminate
    JMP,    // transfer execution to A
    JMZ,    // transfer execution to A if B is zero
    JMN,    // transfer execution to A if B is non-zero
    DJN,    // decrement B, if B is non-zero, transfer execution to A
    SPL,    // split off process to A
    SLT,    // skip next instruction if A is less than B
    CMP,    // same as SEQ
    SEQ,    // (*) Skip next instruction if A is equal to B
    SNE,    // (*) Skip next instruction if A is not equal to B
    NOP,    // (*) No operation
    LDP,    // (+) Load P-space cell A into core address B
    STP,    // (+) Store A-number into P-space cell B
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
