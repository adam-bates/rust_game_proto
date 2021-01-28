use specs::{Component, VecStorage};
use specs_derive::Component;

// Thanks Bevy for this timer implementation

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Timer {
    pub duration: f32,
    pub repeating: bool,
    pub elapsed: f32,
    pub finished: bool,
    pub should_tick: bool,
}

impl Timer {
    pub fn new(duration: std::time::Duration, repeating: bool) -> Self {
        Timer {
            duration: duration.as_secs_f32(),
            repeating,
            ..Default::default()
        }
    }

    pub fn from_seconds(seconds: f32, repeating: bool) -> Self {
        Timer {
            duration: seconds,
            repeating,
            ..Default::default()
        }
    }

    /// Returns the time elapsed on the timer. Guaranteed to be between 0.0 and `duration`, inclusive.
    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    #[inline]
    pub fn set_elapsed(&mut self, elapsed: f32) {
        self.elapsed = elapsed
    }

    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    #[inline]
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration
    }

    /// Returns the finished state of the timer.
    ///
    /// Non repeating timers will stop tracking and stay in the finished state until reset.
    /// Repeating timers will only be in the finished state on each tick `duration` is reached or exceeded, and can still be reset at any given point.
    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }

    #[inline]
    pub fn repeating(&self) -> bool {
        self.repeating
    }

    #[inline]
    pub fn set_repeating(&mut self, repeating: bool) {
        self.repeating = repeating
    }

    #[inline]
    pub fn should_tick(&self) -> bool {
        self.should_tick
    }

    pub fn set_should_tick(&mut self, should_tick: bool) {
        self.should_tick = should_tick;
    }

    /// Advances the timer by `delta` seconds.
    pub fn tick(&mut self, delta: f32) -> &Self {
        self.elapsed += delta;
        self.finished = self.elapsed >= self.duration;

        if self.finished {
            if self.repeating {
                // Repeating timers wrap around
                self.elapsed %= self.duration;
            } else {
                // Non-repeating timers clamp to duration
                self.elapsed = self.duration;
            }
        }
        self
    }

    #[inline]
    pub fn reset(&mut self) {
        self.finished = false;
        self.elapsed = 0.0;
    }

    /// Percent timer has elapsed (goes from 0.0 to 1.0)
    pub fn percent(&self) -> f32 {
        if self.finished {
            1.0
        } else {
            self.elapsed / self.duration
        }
    }

    /// Percent left on timer (goes from 1.0 to 0.0)
    pub fn percent_left(&self) -> f32 {
        (self.duration - self.elapsed) / self.duration
    }
}
