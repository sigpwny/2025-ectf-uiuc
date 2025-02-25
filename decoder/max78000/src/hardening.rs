use cortex_m::delay::Delay;
use rand::RngCore;

/// Delay for a random amount of time between `start_us` and `end_us`.
pub fn delay_random_us<R>(delay: &mut Delay, rng: &mut R, start_us: u32, end_us: u32)
where
    R: RngCore,
{
    assert!(start_us < end_us);
    let random_diff = rng.next_u32() % (end_us - start_us);
    let random_delay = start_us + random_diff;
    delay.delay_us(random_delay);
}

/// Macro that repeats the given expression 5 times
#[macro_export]
macro_rules! repeat_5 {
    ($e:expr) => {
        $e;
        $e;
        $e;
        $e;
        $e;
    };
}