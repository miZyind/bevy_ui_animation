use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ui_animation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rotation".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                        ..Default::default()
                    },
                    color: UiColor(Color::RED),
                    ..Default::default()
                })
                .insert(Animation::new(Vars {
                    transform_rotation: Some(TransformRotation::z(360.0)),
                    duration: 2.0,
                    repeat: true,
                    ..Default::default()
                }))
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            ..Default::default()
                        },
                        color: UiColor(Color::BLUE),
                        ..Default::default()
                    });
                });
        });
}
