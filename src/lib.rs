//! Generate random data in such a way as to make rare edge-cases very likely.
//!
//! > Disclaimer: the random number generators used in this crate are NOT
//! CRYPTOGRAPHICALLY SECURE. Only use these generators for generating testing
//! inputs, do not rely on them for cryptographic purposes in production code!
//! For instance, you may test a cryptographic tool with these generators, but
//! you may not deploy code that relies on these generators for security in
//! production.
//!
//! For instance, if generating a random `f32` by uniformly sampling 32 bits of
//! data, certain values will rarely appear, such as `NAN` and `INFINITY`. When
//! doing randomized testing, like fuzzing, it isn't very useful to repeatedly
//! generate well-behaved data. It is much more useful if we can artificially
//! increase the likelihood of these special values, so that we test with them
//! more often.
//!
//! Additionally, some random number crates will never generate certain
//! problematic bit-patterns, such as `NAN`.
//!
//! This crate is based on the [fastrand]() crate.
//!
//! This crate can work with `no_std`, if you disable the `std` feature. You
//! cannot use the global functions when in a `no_std` environment. In that
//! case, you can explicitly instantiate [Wdg] and call the methods on it.
//! They are equivalent.
//!
//! If using `std`, it's more ergonomic to use the global functions in the
//! [global_functions] module.

#![cfg_attr(not(feature = "std"), no_std)]

use fastrand as fr;
use paste::paste;

#[cfg(feature = "std")]
mod global_functions;

#[cfg(feature = "std")]
pub use global_functions::*;

#[cfg(test)]
mod float_utils;

/// A weird data generator
#[derive(Clone)]
pub struct Wdg(fr::Rng);

macro_rules! int {
    ($self:tt, [$($t:ty),+ $(,)?]) => {
        $(
            int_inner!($self, $t);
        )+
    };
}

macro_rules! int_inner {
    ($self:tt, $t:ty) => {
        paste! {
            /// Generate a random
            #[doc = stringify!($t)]
            /// "special" value
            ///
            /// A special value is what I call specific values that are unique and
            /// are pretty much impossible to generate by chance, and have some unusual
            /// properties. For instance `MAX` and 0.
            pub fn [<special_ $t>](&mut $self) -> $t {
                match $self.0.u8(0..5) {
                    0 => 0,
                    1 => 1,
                    2 => $t::MAX,
                    3 => -1,
                    4 => $t::MIN,
                    _ => unreachable!(),
                }
            }

            /// Generate a random
            #[doc = stringify!($t)]
            /// , such that special or problematic values are much
            /// more common than normal.
            pub fn $t(&mut $self) -> $t {
                match $self.0.u8(0..3) {
                    0 => $self.[<special_ $t>](),
                    1 => $self.0.$t(2..$t::MAX),
                    2 => $self.0.$t($t::MIN..-1),
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! uint {
    ($self:tt, [$($t:ty),+ $(,)?]) => {
        $(
            uint_inner!($self, $t);
        )+
    };
}

macro_rules! uint_inner {
    ($self:tt, $t:ty) => {
        paste! {
            /// Generate a random
            #[doc = stringify!($t)]
            /// "special" value
            ///
            /// A special value is what I call specific values that are unique and
            /// are pretty much impossible to generate by chance, and have some unusual
            /// properties.
            pub fn [<special_ $t>](&mut $self) -> $t {
                match $self.0.u8(0..3) {
                    0 => 0,
                    1 => 1,
                    2 => $t::MAX,
                    _ => unreachable!(),
                }
            }

            pub fn $t(&mut $self) -> $t {
                match $self.0.u8(0..2) {
                    0 => $self.[<special_ $t>](),
                    1 => $self.0.$t(2..$t::MAX),
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl Wdg {
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self(fr::Rng::with_seed(seed))
    }

    #[must_use]
    pub fn fork(&mut self) -> Self {
        Self(self.0.fork())
    }

    pub fn seed(&mut self, seed: u64) {
        self.0.seed(seed);
    }

    pub fn get_seed(&mut self) -> u64 {
        self.0.get_seed()
    }

    /// Generates a random f32 `NAN` value.
    ///
    /// There are multiple bit patterns that are equivalent to a `NAN`.
    /// This generator covers all possible `NAN` values as specified in
    /// IEEE-754, even ones that Rust would normally not generate.
    pub fn nan_f32(&mut self) -> f32 {
        let sign: u32 = self.0.u32(0..=1) << 31;
        let exponent: u32 = 0b1111_1111 << 23;

        // mantissa 00...00 is INFINITY not NAN!
        let mantissa: u32 = self.0.u32(1..(1 << 23));

        let bits = sign | exponent | mantissa;
        f32::from_bits(bits)
    }

    /// Generates a random f64 `NAN` value.
    ///
    /// There are multiple bit patterns that are equivalent to a `NAN`.
    /// This generator covers all possible `NAN` values as specified in
    /// IEEE-754, even ones that Rust would normally not generate.
    pub fn nan_f64(&mut self) -> f64 {
        let sign: u64 = self.0.u64(0..=1) << 63;
        let exponent: u64 = 0b0111_1111_1111 << 52;

        // mantissa 00...00 is INFINITY not NAN!
        let mantissa: u64 = self.0.u64(1..(1 << 52));

        let bits = sign | exponent | mantissa;
        f64::from_bits(bits)
    }

    /// Generates a random f32 denormal value.
    ///
    /// This generator covers all possible denormal values as specified in
    /// IEEE-754.
    pub fn subnormal_f32(&mut self) -> f32 {
        let sign: u32 = self.0.u32(0..=1) << 31;

        // mantissa 00...00 is zero not denormal!
        let mantissa: u32 = self.0.u32(1..(1 << 23));

        let bits = sign | mantissa;
        f32::from_bits(bits)
    }

    /// Generates a random f64 denormal value.
    ///
    /// This generator covers all possible denormal values as specified in
    /// IEEE-754.
    pub fn subnormal_f64(&mut self) -> f64 {
        let sign: u64 = self.0.u64(0..=1) << 63;

        // mantissa 00...00 is zero not denormal!
        let mantissa: u64 = self.0.u64(1..(1 << 52));

        let bits = sign | mantissa;
        f64::from_bits(bits)
    }

    /// Generate a random f32 normal value
    pub fn normal_f32(&mut self) -> f32 {
        let sign: u32 = self.0.u32(0..=1) << 31;

        // careful with this range, all zeros and all ones are not normal
        let exponent: u32 = self.0.u32(0b0000_0001..=0b1111_1110) << 23;

        let mantissa: u32 = self.0.u32(0..=(1 << 23));
        let bits = sign | exponent | mantissa;
        f32::from_bits(bits)
    }

    /// Generate a random f64 normal value
    pub fn normal_f64(&mut self) -> f64 {
        let sign: u64 = self.0.u64(0..=1) << 63;

        // careful with this range, all zeros and all ones are not normal
        let exponent: u64 = self.0.u64(0b000_0000_0001..=0b111_1111_1110) << 52;

        let mantissa: u64 = self.0.u64(0..=(1 << 52));
        let bits = sign | exponent | mantissa;
        f64::from_bits(bits)
    }

    /// Generate a random f32 "special" value
    ///
    /// A special value is what I call specific float values that are unique and
    /// are pretty much impossible to generate by chance, and have some unusual
    /// properties.
    pub fn special_f32(&mut self) -> f32 {
        match self.0.u8(0..=11) {
            0 => 0.0,
            1 => -0.0,
            2 => f32::INFINITY,
            3 => -f32::INFINITY,
            4 => 1.0,
            5 => -1.0,
            6 => f32::MIN,
            7 => f32::MAX,
            8 => f32::MIN_POSITIVE,
            9 => -f32::MIN_POSITIVE,
            10 => f32::EPSILON,
            11 => -f32::EPSILON,
            _ => unreachable!(),
        }
    }

    /// Generate a random f64 "special" value
    ///
    /// A special value is what I call specific float values that are unique and
    /// are pretty much impossible to generate by chance, and have some unusual
    /// properties.
    pub fn special_f64(&mut self) -> f64 {
        match self.0.u8(0..=11) {
            0 => 0.0,
            1 => -0.0,
            2 => f64::INFINITY,
            3 => -f64::INFINITY,
            4 => 1.0,
            5 => -1.0,
            6 => f64::MIN,
            7 => f64::MAX,
            8 => f64::MIN_POSITIVE,
            9 => -f64::MIN_POSITIVE,
            10 => f64::EPSILON,
            11 => -f64::EPSILON,
            _ => unreachable!(),
        }
    }

    /// Generate a random f32, such that special or problematic values are much
    /// more common than normal.
    ///
    /// The distribution is not statistically useful, but it ensures that all edge-case
    /// values get a fair chance of being generated. This is better than using a regular
    /// random number generator, because in the vast majority of cases, a random number
    /// generator will generate perfectly regular and well-behaved values, and certain
    /// values, like `INFINITY` and `NAN` may be impossible to generate.
    ///
    /// The distribution is as follows:
    /// - 25% normal values
    /// - 25% subnormal values
    /// - 25% `NAN` values, including all possible payloads, quiet and signaling `NAN`.
    /// - 25% "special" values, i.e. unique values with special properties such as `INFINITY` and `-0.0`
    pub fn f32(&mut self) -> f32 {
        match self.0.u8(0..4) {
            0 => self.normal_f32(),
            1 => self.subnormal_f32(),
            2 => self.nan_f32(),
            3 => self.special_f32(),
            _ => unreachable!(),
        }
    }

    /// Generate a random f64, such that special or problematic values are much
    /// more common than normal.
    ///
    /// The distribution is not statistically useful, but it ensures that all edge-case
    /// values get a fair chance of being generated. This is better than using a regular
    /// random number generator, because in the vast majority of cases, a random number
    /// generator will generate perfectly regular and well-behaved values, and certain
    /// values, like `INFINITY` and `NAN` may be impossible to generate.
    ///
    /// The distribution is as follows:
    /// - 25% normal values
    /// - 25% subnormal values
    /// - 25% `NAN` values, including all possible payloads, quiet and signaling `NAN`.
    /// - 25% "special" values, i.e. unique values with special properties such as `INFINITY` and `-0.0`
    pub fn f64(&mut self) -> f64 {
        match self.0.u8(0..4) {
            0 => self.normal_f64(),
            1 => self.subnormal_f64(),
            2 => self.nan_f64(),
            3 => self.special_f64(),
            _ => unreachable!(),
        }
    }

    uint!(self, [u8, u16, u32, u64, u128, usize]);

    int!(self, [i8, i16, i32, i64, i128, isize]);
}

#[cfg(test)]
mod test_unit {
    extern crate std;

    use super::*;

    #[test]
    fn nan_f32() {
        let mut gen = Wdg::with_seed(0);
        assert!(gen.nan_f32().is_nan());
    }

    #[test]
    fn nan_f64() {
        let mut gen = Wdg::with_seed(0);
        assert!(gen.nan_f64().is_nan());
    }

    #[test]
    fn subnormal_f32() {
        let mut gen = Wdg::with_seed(0);
        assert!(gen.subnormal_f32().is_subnormal());
    }

    #[test]
    fn subnormal_f64() {
        let mut gen = Wdg::with_seed(0);
        assert!(gen.subnormal_f64().is_subnormal());
    }

    #[test]
    fn normal_f32() {
        let mut gen = Wdg::with_seed(0);
        assert!(!gen.normal_f32().is_subnormal());
    }

    #[test]
    fn normal_f64() {
        let mut gen = Wdg::with_seed(0);
        assert!(!gen.normal_f64().is_subnormal());
    }
}

#[cfg(test)]
mod test_fuzz {
    // fuzzing tests, they may take a while to run. Shouldn't last more than
    // a minute per test (or I'll get impatient/cargo will complain)

    extern crate std;

    use crate::float_utils::{f32_exact_eq, f64_exact_eq};

    use super::*;

    // TODO: all seeds here should be picked at random from RANDOM.org

    #[test]
    #[ignore]
    fn nan_f32_is_nan() {
        let mut gen = Wdg::with_seed(0x0b_65_58_2b_4e_d8_20_fe);
        for i in 0..(1 << 30) {
            let num = gen.nan_f32();
            assert!(num.is_nan(), "{}: {:032b}", i, num.to_bits());
        }
    }

    #[test]
    #[ignore]
    fn nan_f64_is_nan() {
        let mut gen = Wdg::with_seed(0x36_44_3e_f8_40_af_6e_49);
        // TODO: this test has poor coverage, there are 1 << 52 possible mantissas
        //       way too many to guess the bad ones at random. Maybe do something
        //       meta where you use this crate to fuzz itself?
        for i in 0..1 << 30 {
            let num = gen.nan_f64();
            assert!(num.is_nan(), "{}: {:064b}", i, num.to_bits());
        }
    }

    #[test]
    fn nan_f32_range() {
        let mut gen = Wdg::with_seed(0x29_21_f1_bd_8b_a9_c6_b6);
        let mut coverage: u32 = 0b0;
        for _ in 0..10000 {
            let num = gen.nan_f32();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u32::MAX, "{:032b}", coverage);
    }

    #[test]
    fn nan_f64_range() {
        let mut gen = Wdg::with_seed(0x6f_35_67_53_e6_37_13_c3);
        let mut coverage: u64 = 0b0;
        for _ in 0..10000 {
            let num = gen.nan_f64();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u64::MAX, "{:064b}", coverage);
    }

    #[test]
    #[ignore]
    fn subnoraml_f32_is_subnormal() {
        let mut gen = Wdg::with_seed(0x52_58_4a_d1_55_e1_72_10);
        for i in 0..(1 << 30) {
            let num = gen.subnormal_f32();
            assert!(num.is_subnormal(), "{}: {:032b}", i, num.to_bits());
        }
    }

    #[test]
    #[ignore]
    fn subnormal_f64_is_subnormal() {
        let mut gen = Wdg::with_seed(0x2d_46_cc_c0_45_c5_ec_03);
        // TODO: this test has poor coverage, there are 1 << 52 possible mantissas
        //       way too many to guess the bad ones at random. Maybe do something
        //       meta where you use this crate to fuzz itself?
        for i in 0..1 << 30 {
            let num = gen.subnormal_f64();
            assert!(num.is_subnormal(), "{}: {:064b}", i, num.to_bits());
        }
    }

    #[test]
    fn subnormal_f32_range() {
        let mut gen = Wdg::with_seed(0x98_fb_6b_ef_ac_5d_81_f3);
        let mut coverage: u32 = 0b1111_1111 << 23;
        for _ in 0..10000 {
            let num = gen.subnormal_f32();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u32::MAX, "{:032b}", coverage);
    }

    #[test]
    fn subnormal_f64_range() {
        let mut gen = Wdg::with_seed(0x7a_07_58_14_f4_b8_2f_49);
        let mut coverage: u64 = 0b111_1111_1111 << 52;
        for _ in 0..10000 {
            let num = gen.subnormal_f64();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u64::MAX, "{:064b}", coverage);
    }

    #[test]
    #[ignore]
    fn noraml_f32_is_not_subnormal() {
        let mut gen = Wdg::with_seed(0x2c_fe_59_bb_7a_56_28_20);
        for i in 0..(1 << 30) {
            let num = gen.normal_f32();
            assert!(!num.is_subnormal(), "{}: {:032b}", i, num.to_bits());
        }
    }

    #[test]
    #[ignore]
    fn normal_f64_is_not_subnormal() {
        let mut gen = Wdg::with_seed(0xa9_26_d1_d9_7b_d7_94_15);
        // TODO: this test has poor coverage, there are 1 << 52 possible mantissas
        //       way too many to guess the bad ones at random. Maybe do something
        //       meta where you use this crate to fuzz itself?
        for i in 0..1 << 30 {
            let num = gen.normal_f64();
            assert!(!num.is_subnormal(), "{}: {:064b}", i, num.to_bits());
        }
    }

    #[test]
    fn normal_f32_range() {
        let mut gen = Wdg::with_seed(0x15_63_e3_11_09_cb_11_b5);
        let mut coverage: u32 = 0;
        for _ in 0..10000 {
            let num = gen.normal_f32();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u32::MAX, "{:032b}", coverage);
    }

    #[test]
    fn normal_f64_range() {
        let mut gen = Wdg::with_seed(0x56_e5_19_b1_47_f2_5e_0d);
        let mut coverage: u64 = 0;
        for _ in 0..10000 {
            let num = gen.normal_f64();
            coverage |= num.to_bits();
        }

        // every bit should be generated at least once, given enough attempts
        assert_eq!(coverage, u64::MAX, "{:064b}", coverage);
    }

    #[test]
    fn special_f32() {
        let mut gen = Wdg::with_seed(0x69_1b_e9_82_15_ed_a0_7d);
        for _ in 0..10000 {
            gen.special_f32();
        }
    }

    #[test]
    fn special_f64() {
        let mut gen = Wdg::with_seed(0xf5_31_9e_51_c4_1f_9e_35);
        for _ in 0..10000 {
            gen.special_f64();
        }
    }

    macro_rules! int_uint {
        ($($t:ty),+ $(,)?) => {
            $(
                int_uint_inner!($t);
            )+
        };
    }

    macro_rules! int_uint_inner {
        ($t:ty) => {
            paste! {
                #[test]
                pub fn [<special_ $t>]() {
                    let mut gen = Wdg::with_seed(0x29_2d_3a_df_ed_dd_c0_82);
                    for _ in 0..10000 {
                        gen.[<special_ $t>]();
                    }
                }

                #[test]
                pub fn $t(){
                    let mut gen = Wdg::with_seed(0x8e_bd_46_37_50_b4_9b_1a);
                    for _ in 0..10000 {
                        gen.$t();
                    }
                }
            }
        };
    }

    int_uint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

    #[test]
    fn special_f32_range() {
        let mut gen = Wdg::with_seed(0x90_ae_72_03_34_a0_d7_4b);
        let mut had_infinite = false;
        let mut had_neg_infinite = false;
        let mut had_zero = false;
        let mut had_neg_zero = false;
        let mut had_one = false;
        let mut had_neg_one = false;
        let mut had_min_positive = false;
        let mut had_max_negative = false;
        let mut had_epsilon = false;
        let mut had_neg_epsilon = false;
        for _ in 0..10000 {
            let num = gen.special_f32();
            had_infinite |= f32_exact_eq(num, f32::INFINITY);
            had_neg_infinite |= f32_exact_eq(num, f32::NEG_INFINITY);
            had_zero |= f32_exact_eq(num, 0.0);
            had_neg_zero |= f32_exact_eq(num, -0.0);
            had_one |= f32_exact_eq(num, 1.0);
            had_neg_one |= f32_exact_eq(num, -1.0);
            had_min_positive |= f32_exact_eq(num, f32::MIN_POSITIVE);
            had_max_negative |= f32_exact_eq(num, -f32::MIN_POSITIVE);
            had_epsilon |= f32_exact_eq(num, f32::EPSILON);
            had_neg_epsilon |= f32_exact_eq(num, -f32::EPSILON);
        }
        assert!(
            had_infinite
                && had_neg_infinite
                && had_zero
                && had_neg_zero
                && had_one
                && had_neg_one
                && had_min_positive
                && had_max_negative
                && had_epsilon
                && had_neg_epsilon
        );
    }

    #[test]
    fn special_f64_range() {
        let mut gen = Wdg::with_seed(0x10_6c_a1_34_a5_6d_03_97);
        let mut had_infinite = false;
        let mut had_neg_infinite = false;
        let mut had_zero = false;
        let mut had_neg_zero = false;
        let mut had_one = false;
        let mut had_neg_one = false;
        let mut had_min_positive = false;
        let mut had_max_negative = false;
        let mut had_epsilon = false;
        let mut had_neg_epsilon = false;
        for _ in 0..10000 {
            let num = gen.special_f64();
            had_infinite |= f64_exact_eq(num, f64::INFINITY);
            had_neg_infinite |= f64_exact_eq(num, f64::NEG_INFINITY);
            had_zero |= f64_exact_eq(num, 0.0);
            had_neg_zero |= f64_exact_eq(num, -0.0);
            had_one |= f64_exact_eq(num, 1.0);
            had_neg_one |= f64_exact_eq(num, -1.0);
            had_min_positive |= f64_exact_eq(num, f64::MIN_POSITIVE);
            had_max_negative |= f64_exact_eq(num, -f64::MIN_POSITIVE);
            had_epsilon |= f64_exact_eq(num, f64::EPSILON);
            had_neg_epsilon |= f64_exact_eq(num, -f64::EPSILON);
        }
        assert!(
            had_infinite
                && had_neg_infinite
                && had_zero
                && had_neg_zero
                && had_one
                && had_neg_one
                && had_min_positive
                && had_max_negative
                && had_epsilon
                && had_neg_epsilon
        );
    }

    #[test]
    fn f32_range() {
        let mut gen = Wdg::with_seed(0x7c_65_54_c7_d6_a9_d4_b7);

        // these should all be true by the end, given enough attempts
        let mut had_normal = false;
        let mut had_subnormal = false;
        let mut had_nan = false;
        let mut had_special = false;
        for _ in 0..10000 {
            let num = gen.f32();
            had_normal |= num.is_normal();
            had_subnormal |= num.is_subnormal();
            had_nan |= num.is_nan();
            had_special |= num.is_infinite()
                | f32_exact_eq(num, 0.0)
                | f32_exact_eq(num, -0.0)
                | f32_exact_eq(num, 1.0)
                | f32_exact_eq(num, -1.0)
                | f32_exact_eq(num, f32::MIN)
                | f32_exact_eq(num, f32::MAX)
                | f32_exact_eq(num, f32::MIN_POSITIVE)
                | f32_exact_eq(num, -f32::MIN_POSITIVE)
                | f32_exact_eq(num, f32::EPSILON)
                | f32_exact_eq(num, -f32::EPSILON);
        }
        assert!(had_normal && had_subnormal && had_nan && had_special);
    }

    #[test]
    fn f64_range() {
        let mut gen = Wdg::with_seed(0x9a_a4_ee_0f_08_ba_d9_de);

        // these should all be true by the end, given enough attempts
        let mut had_normal = false;
        let mut had_subnormal = false;
        let mut had_nan = false;
        let mut had_special = false;
        for _ in 0..10000 {
            let num = gen.f64();
            had_normal |= num.is_normal();
            had_subnormal |= num.is_subnormal();
            had_nan |= num.is_nan();
            had_special |= num.is_infinite()
                | f64_exact_eq(num, 0.0)
                | f64_exact_eq(num, -0.0)
                | f64_exact_eq(num, 1.0)
                | f64_exact_eq(num, -1.0)
                | f64_exact_eq(num, f64::MIN)
                | f64_exact_eq(num, f64::MAX)
                | f64_exact_eq(num, f64::MIN_POSITIVE)
                | f64_exact_eq(num, -f64::MIN_POSITIVE)
                | f64_exact_eq(num, f64::EPSILON)
                | f64_exact_eq(num, -f64::EPSILON);
        }
        assert!(had_normal && had_subnormal && had_nan && had_special);
    }
}
