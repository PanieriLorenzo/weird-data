//! Internal module for extending float functionality

/// Returns whether or not the float is a signaling NaN.
/// A signaling NaN has a format like:
/// `s111 1111 1nxx xxxx xxxx xxxx xxxx xxxx`
/// where the `x`'s represent a non-zero number (zero
/// would be infinity) and `n` is 0.
/// The sign bit `s` may be anything.
///
/// On some old-fashioned platforms (PA-RISC, some MIPS)
/// a signaling NaN is marked by `n=1`, but the 2008 revision of
/// IEEE754 defines it to be `n=0`.
pub fn f32_is_signaling_nan(f: f32) -> bool {
    let uf: u32 = f.to_bits();
    let signal_bit = 1 << 22;
    let signal_bit_clear = (uf & signal_bit) == 0;
    f32::is_nan(f) && signal_bit_clear
}

/// Same as `f32_is_signaling_nan()` for `f64`'s.
/// The signaling-nan-bit is bit 51 instead of bit 22
pub fn f64_is_signaling_nan(f: f64) -> bool {
    let uf: u64 = f.to_bits();
    let signal_bit = 1 << 51;
    let signal_bit_clear = (uf & signal_bit) == 0;
    f64::is_nan(f) && signal_bit_clear
}

pub fn f32_exact_eq(lhs: f32, rhs: f32) -> bool {
    lhs.to_bits() == rhs.to_bits()
}

pub fn f64_exact_eq(lhs: f64, rhs: f64) -> bool {
    lhs.to_bits() == rhs.to_bits()
}
