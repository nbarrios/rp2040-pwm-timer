// Based on the [atat](https://github.com/BlackbirdHQ/atat/blob/master/atat/examples/common/timer.rs) DWT timer.
use core::sync::atomic::{AtomicU32, Ordering};
use nb;

pub struct ExtDrivenTimer<'a, const TIMER_HZ: u32> {
    ticks: &'a AtomicU32,
    end_time: Option<fugit::TimerInstantU32<TIMER_HZ>>,
}

impl<'a, const TIMER_HZ: u32> ExtDrivenTimer<'a, TIMER_HZ> {
    pub fn new(ticks: &'a AtomicU32) -> Self {
        Self {
            ticks: ticks,
            end_time: None,
        }
    }

    pub fn ticks(&mut self) -> u64 {
        self.ticks.load(Ordering::Relaxed) as u64
    }
}

impl<'a, const TIMER_HZ: u32> fugit_timer::Timer<TIMER_HZ> for ExtDrivenTimer<'a, TIMER_HZ> {
    type Error = core::convert::Infallible;

    fn now(&mut self) -> fugit::TimerInstantU32<TIMER_HZ> {
        fugit::TimerInstantU32::from_ticks(self.ticks() as u32)
    }

    fn start(&mut self, duration: fugit::TimerDurationU32<TIMER_HZ>) -> Result<(), Self::Error> {
        let end = self.now() + duration;
        self.end_time.replace(end);
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.end_time.take();
        Ok(())
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        let now = self.now();
        match self.end_time {
            Some(end) if end <= now => Ok(()),
            _ => Err(nb::Error::WouldBlock),
        }
    }
}
