use std::collections::{hash_map::Entry, HashMap};

use bevy::prelude::*;

use crate::{ease::Delta, lerp::Lerp, Animation, CompleteEvent};

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CompleteEvent>()
            .add_system(animation_system);
    }
}

type SourceMap = HashMap<u32, (Style, Option<UiColor>, Transform, Option<Text>)>;
type Targets<'a> = (
    Entity,
    &'a mut Style,
    Option<&'a mut UiColor>,
    &'a mut Transform,
    Option<&'a mut Text>,
    &'a mut Animation,
);

fn animation_system(
    time: Res<Time>,
    mut commands: Commands,
    mut source: Local<SourceMap>,
    mut query: Query<Targets>,
    mut complete_event_writer: EventWriter<CompleteEvent>,
) {
    for (entity, mut style, color, mut transform, text, ref mut animation) in query.iter_mut() {
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
                    let entry = source.entry(entity.id()).or_insert((
                        style.clone(),
                        color.as_ref().map(|color| **color),
                        *transform,
                        text.as_ref().map(|text| (**text).clone()),
                    ));
                    if let Some(ref target) = animation.vars.style {
                        *style = entry.0.lerp(target, delta);
                    }
                    if let Some(ref target) = animation.vars.color {
                        if let Some(mut color) = color {
                            if let Some(source) = entry.1 {
                                *color = source.lerp(target, delta);
                            }
                        }
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
                    if let Some(ref text_color) = animation.vars.text_color {
                        if let Some(mut text) = text {
                            if let Some(ref source) = entry.3 {
                                let target: Vec4 = text_color.target.into();
                                let source: Vec4 =
                                    source.sections[text_color.section].style.color.into();
                                let value = source.lerp(target, delta);
                                text.sections[text_color.section].style.color = value.into();
                            }
                        }
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
                    complete_event_writer.send(CompleteEvent { entity });
                }
            }
        }
    }
}
