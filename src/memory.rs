use std::{io, thread};

// TODO change later to larger value
const MAX_MEM: u32 = 32;
// const MAX_MEM: u32 = 1024;

// TODO
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mem {
    pub data: [u8; MAX_MEM as usize],
}

impl Mem {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_MEM as usize],
        }
    }

    pub fn from(data: Vec<u8>) -> Self {
        assert!(data.len() <= MAX_MEM as usize);

        // let arr: [u8; MAX_MEM as usize] = data.into_iter().collect::<Vec<u8>>().try_into().unwrap();
        let mut arr = [0; MAX_MEM as usize];
        for (idx, val) in data.iter().enumerate() {
            arr[idx] = val.to_owned();
        }

        Self { data: arr }
    }

    pub fn initialize(&mut self) {
        self.data = [1; MAX_MEM as usize];
    }

    pub fn reset(&mut self) {
        self.data = [0; MAX_MEM as usize];
    }

    fn get_cores(&self) -> io::Result<usize> {
        let cores = thread::available_parallelism()?.get();

        Ok(cores)
    }

    pub fn split(&mut self) -> Vec<Vec<u8>> {
        let cores = self.get_cores().unwrap();
        let len = self.data.len();
        assert!(len >= cores);

        let mut mem_copy = Vec::new();
        for i in 0..len {
            mem_copy.push(self.data[i]);
        }

        let mut chunks: Vec<Vec<u8>> = Vec::new();
        let quo = len / cores;
        let slices: Vec<&[u8]> = mem_copy.chunks(quo).collect();
        for slice in slices {
            chunks.push(slice.into())
        }

        chunks
    }

    pub fn merge(mem: Mem) {
        todo!();
    }

    // FIXME -> causes stack overflow
    // pub fn len(&self) -> usize {
    //     self.len()
    // }
}

#[test]
fn from_zeros_test() {
    let mem = Mem::from(vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        mem,
        Mem {
            data: [0; MAX_MEM as usize]
        }
    );
}

#[test]
fn from_non_zeros_test() {
    let mem = Mem::from(vec![3, 3, 3, 3, 3]);
    assert_eq!(
        mem,
        Mem {
            data: [
                3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        }
    );
}

#[test]
fn split_test() {
    let mut mem = Mem::from(vec![1, 1, 1, 1, 2, 2, 2, 2]);
    assert_eq!(
        mem.split(),
        vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]
    );
}
