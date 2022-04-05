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
    PowerIn,
    PowerInOut,
    PowerOut,
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
            Ease::PowerIn => {
                if self <= 0.0 {
                    0.0
                } else {
                    self.powf(2.0)
                }
            }
            Ease::PowerInOut => {
                if self <= 0.0 {
                    return 0.0;
                }
                if self >= 1.0 {
                    return 1.0;
                }
                if self < 0.5 {
                    (self * 2.0).powf(2.0) / 2.0
                } else {
                    1.0 - (2.0 * (1.0 - self)).powf(2.0) / 2.0
                }
            }
            Ease::PowerOut => {
                if self >= 1.0 {
                    1.0
                } else {
                    1.0 - (1.0 - self).powf(2.0)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back() {
        assert_eq!(0.0.delta(Ease::BackIn), 0.0);
        assert_eq!(0.5.delta(Ease::BackIn), -0.375);
        assert_eq!(1.0.delta(Ease::BackIn), 1.0000001);
        assert_eq!(0.0.delta(Ease::BackInOut), 0.0);
        assert_eq!(0.4.delta(Ease::BackInOut), 0.02088593);
        assert_eq!(0.5.delta(Ease::BackInOut), 0.49999994);
        assert_eq!(0.6.delta(Ease::BackInOut), 0.9791142);
        assert_eq!(1.0.delta(Ease::BackInOut), 1.0);
        assert_eq!(0.0.delta(Ease::BackOut), -1.1920929e-7);
        assert_eq!(0.5.delta(Ease::BackOut), 1.375);
        assert_eq!(1.0.delta(Ease::BackOut), 1.0);
    }

    #[test]
    fn bounce() {
        assert_eq!(0.0.delta(Ease::BounceIn), 0.0);
        assert_eq!(0.5.delta(Ease::BounceIn), 0.28124976);
        assert_eq!(1.0.delta(Ease::BounceIn), 1.0);
        assert_eq!(0.0.delta(Ease::BounceInOut), 0.0);
        assert_eq!(0.4.delta(Ease::BounceInOut), 0.34875);
        assert_eq!(0.5.delta(Ease::BounceInOut), 0.5);
        assert_eq!(0.6.delta(Ease::BounceInOut), 0.65125006);
        assert_eq!(1.0.delta(Ease::BounceInOut), 1.0);
        assert_eq!(0.0.delta(Ease::BounceOut), 0.0);
        assert_eq!(0.5.delta(Ease::BounceOut), 0.71875024);
        assert_eq!(1.0.delta(Ease::BounceOut), 1.0);
    }

    #[test]
    fn elastic() {
        assert_eq!(0.0.delta(Ease::ElasticIn), 0.0);
        assert_eq!(0.5.delta(Ease::ElasticIn), -4.2966086e-8);
        assert_eq!(1.0.delta(Ease::ElasticIn), 2.7498295e-6);
        assert_eq!(0.0.delta(Ease::ElasticInOut), 0.0);
        assert_eq!(0.4.delta(Ease::ElasticInOut), 0.07347279);
        assert_eq!(0.5.delta(Ease::ElasticInOut), 0.9999986);
        assert_eq!(0.6.delta(Ease::ElasticInOut), 1.073474);
        assert_eq!(1.0.delta(Ease::ElasticInOut), 1.0);
        assert_eq!(0.0.delta(Ease::ElasticOut), 0.99999726);
        assert_eq!(0.5.delta(Ease::ElasticOut), 1.0000001);
        assert_eq!(1.0.delta(Ease::ElasticOut), 1.0);
    }

    #[test]
    fn expo() {
        assert_eq!(0.0.delta(Ease::ExpoIn), 0.0);
        assert_eq!(0.5.delta(Ease::ExpoIn), 0.03125);
        assert_eq!(1.0.delta(Ease::ExpoIn), 1.0);
        assert_eq!(0.0.delta(Ease::ExpoInOut), 0.0);
        assert_eq!(0.4.delta(Ease::ExpoInOut), 0.125);
        assert_eq!(0.5.delta(Ease::ExpoInOut), 0.5);
        assert_eq!(0.6.delta(Ease::ExpoInOut), 0.875);
        assert_eq!(1.0.delta(Ease::ExpoInOut), 1.0);
        assert_eq!(0.0.delta(Ease::ExpoOut), 0.0);
        assert_eq!(0.5.delta(Ease::ExpoOut), 0.96875);
        assert_eq!(1.0.delta(Ease::ExpoOut), 1.0);
    }

    #[test]
    fn linear() {
        assert_eq!(0.0.delta(Ease::Linear), 0.0);
        assert_eq!(0.5.delta(Ease::Linear), 0.5);
        assert_eq!(1.0.delta(Ease::Linear), 1.0);
    }

    #[test]
    fn power() {
        assert_eq!(0.0.delta(Ease::PowerIn), 0.0);
        assert_eq!(0.5.delta(Ease::PowerIn), 0.25);
        assert_eq!(1.0.delta(Ease::PowerIn), 1.0);
        assert_eq!(0.0.delta(Ease::PowerInOut), 0.0);
        assert_eq!(0.4.delta(Ease::PowerInOut), 0.32000002);
        assert_eq!(0.5.delta(Ease::PowerInOut), 0.5);
        assert_eq!(0.6.delta(Ease::PowerInOut), 0.68000007);
        assert_eq!(1.0.delta(Ease::PowerInOut), 1.0);
        assert_eq!(0.0.delta(Ease::PowerOut), 0.0);
        assert_eq!(0.5.delta(Ease::PowerOut), 0.75);
        assert_eq!(1.0.delta(Ease::PowerOut), 1.0);
    }
}
