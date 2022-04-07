use bevy::prelude::*;
pub use ease::Ease;
pub use plugin::AnimationPlugin;

mod ease;
mod lerp;
mod plugin;

pub struct Vars {
    pub style: Option<Style>,
    pub color: Option<UiColor>,
    pub transform: Option<Transform>,
    pub transform_rotation: Option<TransformRotation>,
    pub text_color: Option<TextColor>,
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
            transform_rotation: None,
            text_color: None,
            delay: 0.0,
            duration: 0.5,
            ease: ease::Ease::ExpoOut,
            repeat: false,
            yoyo: false,
            paused: false,
        }
    }
}

pub struct TransformRotation {
    axis: Vec3,
    degree: f32,
}
impl TransformRotation {
    pub fn x(degree: f32) -> Self {
        Self {
            axis: Vec3::X,
            degree: degree * -1.0,
        }
    }
    pub fn y(degree: f32) -> Self {
        Self {
            axis: Vec3::Y,
            degree: degree * -1.0,
        }
    }
    pub fn z(degree: f32) -> Self {
        Self {
            axis: Vec3::Z,
            degree: degree * -1.0,
        }
    }
}

pub struct TextColor {
    pub target: Color,
    pub section: usize,
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
