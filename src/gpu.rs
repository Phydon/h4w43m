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

    pub fn split(&mut self) -> Vec<Vec<u8>> {
        let cores = self.get_cores().unwrap();
        let len = self.mem.data.len();
        assert!(len >= cores);

        let mut mem_copy = Vec::new();
        for i in 0..len {
            mem_copy.push(self.mem.data[i]);
        }

        let mut coll: Vec<Vec<u8>> = Vec::new();
        let quo = len / cores;
        let iter: Vec<&[u8]> = mem_copy.chunks(quo).collect();
        for i in iter {
            coll.push(i.into())
        }

        coll
    }

    pub fn merge(mem: Mem) {
        todo!();
    }

    pub fn process(&mut self, operation: Operation, num: u8) {
        match operation {
            Operation::Addition => {
                todo!();
                // TODO threading (on cpu?)
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
        // TODO bitwise addition, etc
        for i in 0..self.mem.data.len() {
            self.mem.data[i] += num;
        }
    }
}

pub enum Operation {
    Addition,
    Multiplication,
}

#[test]
fn split_test() {
    let mem = Mem::from(vec![0, 0, 0, 0, 0, 0, 0, 0]);
    dbg!(&mem);
    let mut gpu = GPU::new(mem);
    gpu.split();
    dbg!(&gpu);
    // assert_eq!();
}
