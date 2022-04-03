// Reference: https://github.com/PistonDevelopers/interpolation
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Ease {
    BackIn,
    BackInOut,
    BackOut,
    BounceIn,
    BounceInOut,
    BounceOut,
    ElasticIn,
    ElasticInOut,
    ElasticOut,
    ExpoIn,
    ExpoInOut,
    ExpoOut,
    Linear,
}

fn clamp(p: f32) -> f32 {
    match () {
        _ if p > 1.0 => 1.0,
        _ if p < 0.0 => 0.0,
        _ => p,
    }
}

pub trait Delta {
    fn delta(self, ease: Ease) -> Self;
}
impl Delta for f32 {
    fn delta(self, ease: Ease) -> f32 {
        match ease {
            Ease::BackIn => {
                let p = clamp(self);
                p * p * p - p * (p * PI).sin()
            }
            Ease::BackInOut => {
                let p = clamp(self);
                if p < 0.5 {
                    let f = 2.0 * p;
                    0.5 * (f * f * f - f * (f * PI).sin())
                } else {
                    let f = 1.0 - (2.0 * p - 1.0);
                    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
                }
            }
            Ease::BackOut => {
                let p = clamp(self);
                let f = 1.0 - p;
                1.0 - (f * f * f - f * (f * PI).sin())
            }
            Ease::BounceIn => {
                let p = clamp(self);
                1.0 - (1.0 - p).delta(Ease::BounceOut)
            }
            Ease::BounceInOut => {
                let p = clamp(self);
                if p < 0.5 {
                    0.5 * (p * 2.0).delta(Ease::BounceIn)
                } else {
                    0.5 * (p * 2.0 - 1.0).delta(Ease::BounceOut) + 0.5
                }
            }
            Ease::BounceOut => {
                let p = clamp(self);
                if p < 4.0 / 11.0 {
                    (121.0 * p * p) / 16.0
                } else if p < 8.0 / 11.0 {
                    (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
                } else if p < 9.0 / 10.0 {
                    (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
                } else {
                    (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
                }
            }
            Ease::ElasticIn => {
                let p = clamp(self);
                (13.0 * PI * 2.0 * p).sin() * (2.0_f32).powf(10.0 * (p - 1.0))
            }
            Ease::ElasticInOut => {
                let p = clamp(self);
                if p < 0.5 {
                    0.5 * (13.0 * PI * 2.0 * (2.0 * p)).sin()
                        * (2.0_f32).powf(10.0 * ((2.0 * p) - 1.0))
                } else {
                    0.5 * ((-13.0 * PI * 2.0 * ((2.0 * p - 1.0) + 1.0)).sin()
                        * (2.0_f32).powf(-10.0 * (2.0 * p - 1.0))
                        + 2.0)
                }
            }
            Ease::ElasticOut => {
                let p = clamp(self);
                (-13.0 * PI * 2.0 * (p + 1.0)).sin() * (2.0_f32).powf(-10.0 * p) + 1.0
            }
            Ease::ExpoIn => {
                if self <= 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * (self.min(1.0) - 1.0))
                }
            }
            Ease::ExpoInOut => {
                if self <= 0.0 {
                    return 0.0;
                }
                if self >= 1.0 {
                    return 1.0;
                }
                if self < 0.5 {
                    0.5 * (2.0_f32).powf((20.0 * self) - 10.0)
                } else {
                    -0.5 * (2.0_f32).powf((-20.0 * self) + 10.0) + 1.0
                }
            }
            Ease::ExpoOut => {
                if self >= 1.0 {
                    1.0
                } else {
                    1.0 - (2.0_f32).powf(-10.0 * self.max(0.0))
                }
            }
            Ease::Linear => clamp(self),
        }
    }
}
