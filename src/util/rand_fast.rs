/// Performs fast random number generation using the PCG algorithm.
///
/// # Arguments
/// * `state` - A mutable reference to the current state of the random number generator.
///
/// # Returns
/// A random `u32` number generated from the current state.
#[inline(always)]
pub fn pcg32_fast(state: &mut u64) -> u32 {
    let oldstate = *state;

    *state = oldstate
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);

    let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;

    let rot = (oldstate >> 59) as u32;

    xorshifted.rotate_right(rot)
}

/// Generates a random number from a seed between a minimum and maximum range.
///
/// # Arguments
/// * `seed` - A mutable reference to a seed value for generating the random number.
/// * `min` - The minimum value for the random number (inclusive).
/// * `max` - The maximum value for the random number (inclusive).
///
/// # Returns
/// A random number of type `u64` between `min` and `max`.
#[inline(always)]
pub fn rand_num(seed: &mut u64, min: u64, max: u64) -> u64 {
    let rand_num = pcg32_fast(seed) as u64;
    let range = max - min + 1;

    // Shifting is faster.
    let scaled = (rand_num.wrapping_mul(range)) >> 32;

    min + scaled
}
