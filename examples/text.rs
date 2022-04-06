use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ui_animation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Text".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
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
                .spawn_bundle(TextBundle {
                    style: Style::default(),
                    text: Text::with_section(
                        "Hello, World!",
                        TextStyle {
                            font: assets.load("fonts/FiraMono-Regular.ttf"),
                            font_size: 24.0,
                            color: Color::BLUE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(Animation::new(Vars {
                    text_color: Some(TextColor {
                        target: Color::RED,
                        section: 0,
                    }),
                    duration: 2.0,
                    repeat: true,
                    yoyo: true,
                    ..Default::default()
                }));
        });
}
