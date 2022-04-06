use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ui_animation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "All".to_string(),
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
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(Color::BLUE),
            transform: Transform::default(),
            ..Default::default()
        })
        .insert(Animation::new(Vars {
            style: Some(Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                ..Default::default()
            }),
            color: Some(UiColor(Color::RED)),
            transform: Some(Transform::from_rotation(Quat::from_rotation_z(
                180_f32.to_radians(),
            ))),
            duration: 2.0,
            ease: Ease::ExpoOut,
            repeat: true,
            yoyo: true,
            ..Default::default()
        }));
}
