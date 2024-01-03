//! Internal module for extending float functionality

pub fn f32_exact_eq(lhs: f32, rhs: f32) -> bool {
    lhs.to_bits() == rhs.to_bits()
}

pub fn f64_exact_eq(lhs: f64, rhs: f64) -> bool {
    lhs.to_bits() == rhs.to_bits()
}
