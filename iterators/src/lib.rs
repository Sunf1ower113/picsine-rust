use std::iter::Enumerate;

#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    fn next(&mut self) -> Option<Self::Item> {
           let s = "*".to_string();
        s = s.repeat(10)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self {
            v: 0,
        }
    }
}

pub fn collatz(n: u64) -> usize {
    0
}