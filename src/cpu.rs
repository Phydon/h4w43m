const MAX_MEM: u32 = 1024 * 64;

pub enum M6502 {
    Byte,
    Word,
    Mem,
    CPU,
    Statusflags,
}

#[derive(Debug)]
pub struct Mem {
    data: Option<[u8; MAX_MEM as usize]>,
}

impl Mem {
    pub fn new() -> Option<Mem> {
        None
    }

    pub fn initialize(&self) -> Mem {
        Mem {
            data: Some([0; MAX_MEM as usize]),
        }
    }
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
        self.pc = 0;
        self.sp = 0;
        // FIXME
        self.reg = 0;
        mem.unwrap().initialize();
    }

    pub fn execute() {
        todo!();
    }
}

#[derive(Debug)]
pub struct StatusFlags;
