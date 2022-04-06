use std::collections::{hash_map::Entry, HashMap};

use bevy::prelude::*;

use crate::{ease::Delta, lerp::Lerp, Animation};

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system);
    }
}

fn animation_system(
    time: Res<Time>,
    mut commands: Commands,
    mut source: Local<HashMap<u32, (Style, UiColor, Transform)>>,
    mut query: Query<(
        Entity,
        &mut Style,
        &mut UiColor,
        &mut Transform,
        &mut Animation,
    )>,
) {
    for (entity, mut style, mut color, mut transform, ref mut animation) in query.iter_mut() {
        if !animation.vars.paused {
            animation.delay_timer.tick(time.delta());
            if animation.delay_timer.finished() {
                animation.timer.tick(time.delta());
                if animation.timer.duration().as_secs_f32() != 0.0 {
                    let progress = if animation.direction.is_positive() {
                        animation.timer.percent()
                    } else {
                        animation.timer.percent_left()
                    };
                    let delta = progress.delta(animation.vars.ease);
                    let entry =
                        source
                            .entry(entity.id())
                            .or_insert((style.clone(), *color, *transform));
                    if let Some(ref target) = animation.vars.style {
                        *style = entry.0.lerp(target, delta);
                    }
                    if let Some(ref target) = animation.vars.color {
                        *color = entry.1.lerp(target, delta);
                    }
                    if let Some(ref target) = animation.vars.transform {
                        *transform = entry.2.lerp(target, delta);
                    }
                    if let Some(ref target) = animation.vars.transform_rotation {
                        let source_angle = entry.2.rotation.to_axis_angle().1;
                        let target_angle = target.degree.to_radians();
                        let delta_angle = source_angle + (target_angle - source_angle) * delta;
                        transform.rotation = Quat::from_axis_angle(target.axis, delta_angle);
                    }
                }
                if animation.timer.just_finished() {
                    if animation.vars.repeat {
                        if animation.vars.yoyo {
                            animation.direction *= -1;
                        }
                        animation.timer.reset();
                    } else {
                        commands.entity(entity).remove::<Animation>();
                        if let Entry::Occupied(o) = source.entry(entity.id()) {
                            o.remove_entry();
                        }
                    }
                }
            }
        }
    }
}
