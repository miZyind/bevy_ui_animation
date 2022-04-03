use bevy::prelude::*;
pub use ease::Ease;

mod ease;
mod lerp;
mod plugin;

pub struct Vars {
    pub style: Option<Style>,
    pub color: Option<UiColor>,
    pub transform: Option<Transform>,
    pub delay: f32,
    pub duration: f32,
    pub ease: Ease,
    pub repeat: bool,
    pub yoyo: bool,
    pub paused: bool,
}
impl Default for Vars {
    fn default() -> Self {
        Self {
            style: None,
            color: None,
            transform: None,
            delay: 0.0,
            duration: 0.5,
            ease: ease::Ease::ExpoOut,
            repeat: false,
            yoyo: false,
            paused: false,
        }
    }
}

#[derive(Component)]
pub struct Animation {
    timer: Timer,
    delay_timer: Timer,
    direction: i16,
    vars: Vars,
}
impl Animation {
    pub fn new(vars: Vars) -> Self {
        Self {
            timer: Timer::from_seconds(vars.duration, false),
            delay_timer: Timer::from_seconds(vars.delay, false),
            direction: 1,
            vars,
        }
    }
}
