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

        let arr: [u8; MAX_MEM as usize] = data.into_iter().collect::<Vec<u8>>().try_into().unwrap();

        Self { data: arr }
    }

    pub fn initialize(&mut self) {
        self.data = [1; MAX_MEM as usize];
    }

    pub fn reset(&mut self) {
        self.data = [0; MAX_MEM as usize];
    }

    // FIXME -> causes stack overflow
    // pub fn len(&self) -> usize {
    //     self.len()
    // }
}

#[test]
fn from_test() {
    let mem = Mem::from(vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        mem,
        Mem {
            data: [0, 0, 0, 0, 0, 0, 0, 0]
        }
    );
}
