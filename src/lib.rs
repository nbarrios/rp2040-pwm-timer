//! # RP2040 PWM Timer
//! 
//! The RP2040 has only one timer peripheral, which usually provides monotonic time for the app or rtOS.
//! This timer provides an additional time source (with millisecond resolution) for libraries which take a fugit_timer,
//! using a PWM slice counter.
//! # Example
//! 
//! ```
//! static PWM_TICK: AtomicU32 = AtomicU32::new(0);
//! 
//! let pwm_slices = Slices::new(cx.device.PWM, &mut resets);
//! let pwm_timer_driver = PwmTimerDriver::new(pwm_slices.pwm6, &PWM_TICK, clocks.peripheral_clock.freq().raw());
//! let pwm_timer = ExtDrivenTimer::<PWM_TIMER_HZ>::new(&PWM1_OVERFLOWS); 
//! // Now the timer can be handed off to e.g. a display driver
//! ...
//! // If the timer will be waited on using nb's block! macro or something similar, make sure this
//! // interrupt has higher priority than whatever context the polling happens in
//! #[interrupt]
//! fn PWM_IRQ_WRAP() {
//!     pwm_timer_driver.on_wrap();
//! }
//! ``` 
//! 
//! # ToDo
//! 
//! - Add drivers using other time sources (PIO, Systick)
//! - Add on-device tests using defmt-test harness
//! - Add example application
#![no_std]

pub mod timer_drivers;
pub mod ext_driven_timer;

pub use self::timer_drivers::pwm_timer_driver::PwmTimerDriver;
pub use self::ext_driven_timer::ExtDrivenTimer;