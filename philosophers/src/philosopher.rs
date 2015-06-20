#[derive(Clone, Copy, Debug)]
pub struct Philosopher { pub weight: u32, pub num_eaten: u32 }

impl Philosopher {
    pub fn new(weight: u32) -> Self {
        Philosopher { weight: weight, num_eaten: 0 }
    }

    pub fn eat(&self) -> Self {
        Philosopher { num_eaten: self.num_eaten + 1, ..*self }
    }
}
