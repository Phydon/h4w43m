use std::thread;

use crate::memory::*;

#[derive(Debug)]
pub struct GPU {
    pub mem: Mem,
}

impl GPU {
    pub fn new(memory: Mem) -> Self {
        Self { mem: memory }
    }

    pub fn process(&mut self, operation: Operation, num: u8) {
        match operation {
            Operation::Addition => {
                todo!();
                // TODO threading (on cpu?)
                // FIXME
                // self.mem.split().iter().map(|mut chunk| {
                //     let handle = thread::spawn(move || Self::add(&mut chunk, num));
                //     handle.join().unwrap();
                // });
            }
            Operation::Multiplication => {
                todo!();
            }
        }
    }

    pub fn add(chunk: &mut Vec<u8>, num: u8) {
        // TODO bitwise addition, etc
        for i in 0..chunk.len() {
            chunk[i] += num;
        }
    }
}

pub enum Operation {
    Addition,
    Multiplication,
}

// TODO
#[test]
fn add_test() {
    let mem = Mem::new();
    let gpu = GPU::new(mem);
    dbg!(&gpu);
}
