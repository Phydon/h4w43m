// TODO threading (on cpu?)
// TODO bitwise addition, etc
use std::{io, thread};

use crate::memory::*;

#[derive(Debug)]
pub struct GPU {
    pub mem: Mem,
}

impl GPU {
    pub fn new(memory: Mem) -> Self {
        Self { mem: memory }
    }

    fn get_cores(&self) -> io::Result<usize> {
        let cores = thread::available_parallelism()?.get();

        Ok(cores)
    }

    // FIXME
    pub fn split(&mut self) {
        let cores = self.get_cores().unwrap();
        let len = self.mem.len();
        assert!(len >= cores);

        let mut mem_copy = Vec::new();
        for i in 0..len {
            mem_copy.push(self.mem)
        }
        dbg!(&mem_copy);

        let (quo, rem) = (len / cores, len % cores);
        let split = (quo + 1) * rem;
        let mut iter = mem_copy[..split]
            .chunks(quo + 1)
            .chain(mem_copy[split..].chunks(quo));
        dbg!(iter);
    }

    pub fn merge(mem: Mem) {
        todo!();
    }

    pub fn process(&mut self, operation: Operation, num: u8) {
        match operation {
            Operation::Addition => {
                todo!();
                // for chunk in self.mem.split() {
                //     let handle = thread::spawn(|| chunk.add(num));
                //     handle.join().unwrap();
                // }
            }
            Operation::Multiplication => {
                todo!();
            }
        }
    }

    pub fn add(&mut self, num: u8) {
        for i in 0..self.mem.data.len() {
            self.mem.data[i] += num;
        }
    }
}

pub enum Operation {
    Addition,
    Multiplication,
}
