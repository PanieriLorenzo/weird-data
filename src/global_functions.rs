//! A global, thread-local [Wdg] instance.

use fastrand as fr;

use crate::Wdg;

use std::cell::Cell;

// clippy is not aware that deriving Default is only possible when no std
// because Rng does not implement in no std either
#[allow(clippy::derivable_impls)]
impl Default for Wdg {
    fn default() -> Self {
        Self(fr::Rng::default())
    }
}

impl Wdg {
    /// Create a new Wdg by forking the global Wdg.
    ///
    /// If you want to control the initial seed, use [with_seed] instead.
    pub fn new() -> Self {
        try_with_wdg(Wdg::fork).unwrap_or_else(|_| Wdg::with_seed(0x0d_6a_b0_f1_c7_ff_b9_1b))
    }
}

thread_local! {
    /// Likely to be truly random, using system provided entropy. It may be
    /// based on a default seed if the system entropy isn't available.
    static GLOBAL_WDG: Cell<Wdg> = Cell::new(Wdg(fr::Rng::new()));
}

/// Run an operation with the current thread-local generator.
fn with_wdg<R>(f: impl FnOnce(&mut Wdg) -> R) -> R {
    GLOBAL_WDG.with(|wdg| {
        let current = wdg.replace(Wdg::with_seed(0));
        let mut restore = RestoreOnDrop { wdg, current };
        f(&mut restore.current)
    })
}

/// Try to run an operation with the current thread-local generator.
fn try_with_wdg<R>(f: impl FnOnce(&mut Wdg) -> R) -> Result<R, std::thread::AccessError> {
    GLOBAL_WDG.try_with(|wdg| {
        let current = wdg.replace(Wdg::with_seed(0));
        let mut restore = RestoreOnDrop { wdg, current };
        f(&mut restore.current)
    })
}

/// Make sure the original WDG is restored even on panic.
struct RestoreOnDrop<'a> {
    wdg: &'a Cell<Wdg>,
    current: Wdg,
}

impl Drop for RestoreOnDrop<'_> {
    fn drop(&mut self) {
        self.wdg.set(Wdg(self.current.0.clone()));
    }
}

/// Initialize the thread-local generator with the given seed.
pub fn seed(seed: u64) {
    with_wdg(|wdg| wdg.seed(seed));
}

/// Gives back the _current_ seed that is being held by the thread-local generator.
pub fn get_seed() -> u64 {
    with_wdg(|wdg| wdg.get_seed())
}

// Generates a random f32 `NAN` value.
///
/// There are multiple bit patterns that are equivalent to a `NAN`.
/// This generator covers all possible `NAN` values as specified in
/// IEEE-754, even ones that Rust would normally not generate.
pub fn nan_f32() -> f32 {
    with_wdg(|wdg| wdg.nan_f32())
}

/// Generates a random f64 `NAN` value.
///
/// There are multiple bit patterns that are equivalent to a `NAN`.
/// This generator covers all possible `NAN` values as specified in
/// IEEE-754, even ones that Rust would normally not generate.
pub fn nan_f64() -> f64 {
    with_wdg(|wdg| wdg.nan_f64())
}

/// Generates a random f32 denormal value.
///
/// This generator covers all possible denormal values as specified in
/// IEEE-754.
pub fn subnormal_f32() -> f32 {
    with_wdg(|wdg| wdg.subnormal_f32())
}

/// Generates a random f64 denormal value.
///
/// This generator covers all possible denormal values as specified in
/// IEEE-754.
pub fn subnormal_f64() -> f64 {
    with_wdg(|wdg| wdg.subnormal_f64())
}

/// Generate a random f32 normal value
pub fn normal_f32() -> f32 {
    with_wdg(|wdg| wdg.normal_f32())
}

/// Generate a random f64 normal value
pub fn normal_f64() -> f64 {
    with_wdg(|wdg| wdg.normal_f64())
}

/// Generate a random f32 "special" value
///
/// A special value is what I call specific float values that are unique and
/// are pretty much impossible to generate by chance, and have some unusual
/// properties.
pub fn special_f32() -> f32 {
    with_wdg(|wdg| wdg.special_f32())
}

/// Generate a random f64 "special" value
///
/// A special value is what I call specific float values that are unique and
/// are pretty much impossible to generate by chance, and have some unusual
/// properties.
pub fn special_f64() -> f64 {
    with_wdg(|wdg| wdg.special_f64())
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
pub fn f32() -> f32 {
    with_wdg(|wdg| wdg.f32())
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
pub fn f64() -> f64 {
    with_wdg(|wdg| wdg.f64())
}
