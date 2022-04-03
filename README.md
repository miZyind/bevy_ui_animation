# Bevy UI Animation

[![Rust](https://img.shields.io/badge/rust-v1.59.0-0e7261?style=for-the-badge&logo=rust&logoColor=fff&labelColor=2a3438)](https://www.rust-lang.org)
[![Bevy](https://img.shields.io/crates/v/bevy?style=for-the-badge&labelColor=2a3438&color=855667&label=Bevy)](https://bevyengine.org)
[![Crate](https://img.shields.io/crates/v/bevy_ui_animation?style=for-the-badge&labelColor=2a3438&label=Crate&color=a72144)](https://crates.io/crates/bevy_ui_animation)
[![Docrs](https://img.shields.io/docsrs/bevy_ui_animation?style=for-the-badge&labelColor=2a3438&label=Docrs)](https://docs.rs/bevy_ui_animation)

A [GSAP-like](https://greensock.com/gsap) animation plugin for Bevy UI.

## 🌌 Features

- ✅ Animate NodeBundle, ImageBundle, ButtonBundle
- 🚧 Animate TextBundle
- 🚧 Timeline support
- 🚧 Bevy event support

## 📜 Animatable Components

| Name      | Field       |
| --------- | ----------- |
| Style     | position    |
|           | margin      |
|           | padding     |
|           | border      |
|           | size        |
| UiColor   | color       |
| Transform | translation |
|           | rotation    |
|           | scale       |

## 🔮 Usage

To use this plugin, the first step is to add a dependency to your `Cargo.toml`:

```toml
[dependencies]
bevy_ui_animation = "1.0.0"
```

Add the `AnimationPlugin` to your App in `main.rs`:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationPlugin)
    .run();
```

Animate a Bevy UI bundle:

```rust
use bevy::prelude::*;
use bevy_ui_animation::*;

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
```

Preview:

![preview](https://github.com/miZyind/bevy_ui_animation/blob/master/examples/all.gif)

## 🙏 Thanks To

- @djeedai for [bevy_tweening](https://crates.io/crates/bevy_tweening)
- @mockersf for [bevy_easings](https://crates.io/crates/bevy_easings)
- @PistonDevelopers for [interpolation](https://crates.io/crates/interpolation)

## 🖋 Author

miZyind <mizyind@gmail.com>

## 📇 License

Licensed under the [MIT](LICENSE) License.
