use crate::memory::*;

// FIXME not an enum
pub enum M6502 {
    Byte,
    Word,
    Mem,
    CPU,
    Statusflags,
}

pub struct CPU {
    pc: i32,
    sp: u8,
    reg: Register,
}

pub enum Register {
    A,
    X,
    Y,
}

impl CPU {
    pub fn reset(&self, mem: Option<Mem>) {
        // FIXME
        // self.pc = 0;
        // self.sp = 0;
        // self.reg = 0;
        mem.unwrap().initialize();
    }

    pub fn execute() {
        todo!();
    }
}

#[derive(Debug)]
pub struct StatusFlags;
