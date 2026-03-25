use std::time::Instant;

use bevy::{
    app::{App, Plugin},
    time::Time,
};

pub mod prelude {
    #[allow(unused)]
    pub use crate::InGame;
}

///
pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {}
}

#[allow(unused)]
#[derive(Debug)]
pub struct InGame {
    last_external_time: Instant,
}

impl Default for InGame {
    fn default() -> Self {
        Self {
            last_external_time: Instant::now(),
        }
    }
}

#[allow(unused)]
trait InGameTime {
    fn update_from_external(&mut self, instant: Instant);
}

impl InGameTime for Time<InGame> {
    fn update_from_external(&mut self, instant: Instant) {
        let delta = instant - self.context().last_external_time;
        self.advance_by(delta);
        self.context_mut().last_external_time = instant;
    }
}
