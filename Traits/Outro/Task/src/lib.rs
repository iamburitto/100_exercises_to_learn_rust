// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct SaturatingU16 {
    value: u16,
}

impl u16 for SaturatingU16 {

}

impl u8 for SaturatingU16 {
    Self::value.as_
}

impl &u16 for SaturatingU16 {
    &mut Self.value
}

impl &u8 for SaturatingU16 {

}

impl Add for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.value.saturating_add(rhs.value))
    }
}

