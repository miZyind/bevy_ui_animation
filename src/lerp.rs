// Reference: https://github.com/vleue/bevy_easings
use bevy::prelude::*;

pub trait Lerp {
    fn lerp(&self, target: &Self, delta: f32) -> Self;
}
impl Lerp for f32 {
    fn lerp(&self, target: &f32, delta: f32) -> f32 {
        self + (target - self) * delta
    }
}
impl Lerp for Val {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        match (self, target) {
            (Val::Percent(self_val), Val::Percent(other_val)) => {
                Val::Percent(self_val.lerp(other_val, delta))
            }
            (Val::Px(self_val), Val::Px(other_val)) => Val::Px(self_val.lerp(other_val, delta)),
            _ => *self,
        }
    }
}
impl Lerp for Size<Val> {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        Size {
            width: self.width.lerp(&target.width, delta),
            height: self.height.lerp(&target.height, delta),
        }
    }
}
impl Lerp for Rect<Val> {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        Rect {
            left: self.left.lerp(&target.left, delta),
            right: self.right.lerp(&target.right, delta),
            top: self.top.lerp(&target.top, delta),
            bottom: self.bottom.lerp(&target.bottom, delta),
        }
    }
}
impl Lerp for Style {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        Style {
            position: self.position.lerp(&target.position, delta),
            margin: self.margin.lerp(&target.margin, delta),
            padding: self.padding.lerp(&target.padding, delta),
            border: self.border.lerp(&target.border, delta),
            size: self.size.lerp(&target.size, delta),
            ..*self
        }
    }
}
impl Lerp for UiColor {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        UiColor(Vec4::from(self.0).lerp(Vec4::from(target.0), delta).into())
    }
}
impl Lerp for Transform {
    fn lerp(&self, target: &Self, delta: f32) -> Self {
        Transform {
            translation: self.translation.lerp(target.translation, delta),
            rotation: self.rotation.lerp(target.rotation, delta),
            scale: self.scale.lerp(target.scale, delta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style() {
        let source = Style {
            position: Rect::all(Val::Px(0.0)),
            margin: Rect::all(Val::Px(0.0)),
            padding: Rect::all(Val::Px(0.0)),
            border: Rect::all(Val::Px(0.0)),
            size: Size::new(Val::Px(0.0), Val::Px(0.0)),
            ..Default::default()
        };
        let ref target = Style {
            position: Rect::all(Val::Px(10.0)),
            margin: Rect::all(Val::Px(10.0)),
            padding: Rect::all(Val::Px(10.0)),
            border: Rect::all(Val::Px(10.0)),
            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
            ..Default::default()
        };

        let a = source.lerp(target, 0.0);
        assert_eq!(a.position, Rect::all(Val::Px(0.0)));
        assert_eq!(a.margin, Rect::all(Val::Px(0.0)));
        assert_eq!(a.padding, Rect::all(Val::Px(0.0)));
        assert_eq!(a.border, Rect::all(Val::Px(0.0)));
        assert_eq!(a.size, Size::new(Val::Px(0.0), Val::Px(0.0)));

        let b = source.lerp(target, 0.5);
        assert_eq!(b.position, Rect::all(Val::Px(5.0)));
        assert_eq!(b.margin, Rect::all(Val::Px(5.0)));
        assert_eq!(b.padding, Rect::all(Val::Px(5.0)));
        assert_eq!(b.border, Rect::all(Val::Px(5.0)));
        assert_eq!(b.size, Size::new(Val::Px(25.0), Val::Px(25.0)));

        let c = source.lerp(target, 1.0);
        assert_eq!(c.position, Rect::all(Val::Px(10.0)));
        assert_eq!(c.margin, Rect::all(Val::Px(10.0)));
        assert_eq!(c.padding, Rect::all(Val::Px(10.0)));
        assert_eq!(c.border, Rect::all(Val::Px(10.0)));
        assert_eq!(c.size, Size::new(Val::Px(50.0), Val::Px(50.0)));
    }

    #[test]
    fn color() {
        let source = UiColor::default();
        let ref target = UiColor(Vec4::default().into());

        assert_eq!(
            source.lerp(target, 0.0).0,
            UiColor(Vec4::new(1.0, 1.0, 1.0, 1.0).into()).0
        );
        assert_eq!(
            source.lerp(target, 0.5).0,
            UiColor(Vec4::new(0.5, 0.5, 0.5, 0.5).into()).0
        );
        assert_eq!(
            source.lerp(target, 1.0).0,
            UiColor(Vec4::default().into()).0
        );
    }

    #[test]
    fn transform() {
        let source = Transform::default();
        let ref target = Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_rotation_z(100_f32.to_radians()),
            scale: Vec3::new(2.0, 3.0, 4.0),
        };

        let a = source.lerp(target, 0.0);
        assert!(a.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(a.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(a.scale.abs_diff_eq(Vec3::ONE, 1e-5));

        let b = source.lerp(target, 0.5);
        assert!(b.translation.abs_diff_eq(Vec3::new(0.5, 1.0, 1.5), 1e-5));
        assert!(b
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(50_f32.to_radians()), 1e-5));
        assert!(b.scale.abs_diff_eq(Vec3::new(1.5, 2.0, 2.5), 1e-5));

        let c = source.lerp(target, 1.0);
        assert!(c.translation.abs_diff_eq(Vec3::new(1.0, 2.0, 3.0), 1e-5));
        assert!(c
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(100_f32.to_radians()), 1e-5));
        assert!(c.scale.abs_diff_eq(Vec3::new(2.0, 3.0, 4.0), 1e-5));
    }
}
