use core::sync::atomic::{AtomicU32, Ordering};
use rp2040_hal::pwm::{Slice, SliceId, FreeRunning};

pub const PWM_TIMER_HZ: u32 = 1000;

/// An external driver for a fugit_timer using one of RP2040's pwm slices.
/// 
/// The driver owns the pwm slice and takes a reference to a (likely statically allocated)
/// AtomicU32 that the timer will use as its tick.
pub struct PwmTimerDriver<'a, T: SliceId> {
    pwm: Slice<T, FreeRunning>,
    ticks: &'a AtomicU32,
}

impl<'a, T: SliceId> PwmTimerDriver<'a, T> {
    /// Create a new PwmTimerDriver
    pub fn new(
        mut pwm: Slice<T, FreeRunning>,
        ticks: &'a AtomicU32,
        periph_timer_freq: u32,
    ) -> Self {
        pwm.enable();
        pwm.clr_ph_correct();

        let div_int: u32 = periph_timer_freq / (PWM_TIMER_HZ * PWM_TIMER_HZ);
        assert!(div_int < 256);
        pwm.set_div_int(div_int as u8);

        let frac_mod = periph_timer_freq % (PWM_TIMER_HZ * PWM_TIMER_HZ);
        if frac_mod > 0 {
            let frac_mul = (PWM_TIMER_HZ * PWM_TIMER_HZ) / frac_mod;
            let div_frac = 0xFFFF / frac_mul;
            pwm.set_div_frac(div_frac as u8);
        } else {
            pwm.set_div_frac(0);
        }

        pwm.set_top(PWM_TIMER_HZ as u16);
        pwm.enable_interrupt();

        Self { pwm, ticks }
    }

    /// Increments the tick counter on every PWM wrap
    /// 
    /// Must be called on every PWM_IRQ_WRAP interrupt to tick the driven timer
    /// Since the RP2040 core is thumbv6, we must first load and then store the counter.
    /// I believe, at worst, the timer would read the previous tick
    pub fn on_wrap(&mut self) {
        if self.pwm.has_overflown() {
            let t = self.ticks.load(Ordering::Relaxed);
            self.ticks.store(t + 1, Ordering::Relaxed);
            self.pwm.clear_interrupt();
        }
    }
}