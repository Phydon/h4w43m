// TODO change later to larger value
const MAX_MEM: u32 = 8;
// const MAX_MEM: u32 = 1024;

// TODO
#[derive(Debug, Clone, Copy)]
pub struct Mem {
    pub data: [u8; MAX_MEM as usize],
}

impl Mem {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_MEM as usize],
        }
    }

    pub fn initialize(&mut self) {
        self.data = [1; MAX_MEM as usize];
    }

    pub fn reset(&mut self) {
        self.data = [0; MAX_MEM as usize];
    }

    pub fn len(&self) -> usize {
        self.len()
    }
}
