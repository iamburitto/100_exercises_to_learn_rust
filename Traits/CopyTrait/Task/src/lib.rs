use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// Add function
impl Add for WrappingU32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.value.wrapping_add(rhs.value))
    }
}