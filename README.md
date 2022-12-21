# RP2040 PWM Timer

The RP2040 has only one timer peripheral, which usually provides monotonic time for the app or rtOS.
This timer provides an additional time source (with millisecond resolution) for libraries which take a fugit_timer,
using a PWM slice counter.

# Example

```
static PWM_TICK: AtomicU32 = AtomicU32::new(0);

let pwm_slices = Slices::new(cx.device.PWM, &mut resets);
let pwm_timer_driver = PwmTimerDriver::new(pwm_slices.pwm6, &PWM_TICK, clocks.peripheral_clock.freq().raw());
let pwm_timer = ExtDrivenTimer::<PWM_TIMER_HZ>::new(&PWM1_OVERFLOWS); 
// Now the timer can be handed off to e.g. a display driver
...
// If the timer will be waited on using nb's block! macro or something similar, make sure this
// interrupt has higher priority than whatever context the polling happens in
#[interrupt]
fn PWM_IRQ_WRAP() {
    pwm_timer_driver.on_wrap();
}
``` 

# ToDo

- Add drivers using other time sources (PIO, Systick)
- Add on-device tests using defmt-test harness
- Add example application

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.