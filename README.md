# Bevy UI Animation

[![Rust](https://img.shields.io/badge/rust-v1.59.0-0e7261?style=for-the-badge&logo=rust&logoColor=fff&labelColor=2a3438)](https://www.rust-lang.org)
[![Crate](https://img.shields.io/crates/v/bevy_ui_animation?style=for-the-badge&&logo=rust&logoColor=fff&labelColor=2a3438&label=Crate&color=a72144)](https://crates.io/crates/bevy_ui_animation)
[![Docrs](https://img.shields.io/docsrs/bevy_ui_animation?style=for-the-badge&&logo=rust&logoColor=fff&labelColor=2a3438&label=Docrs)](https://docs.rs/bevy_ui_animation)
[![Bevy](https://img.shields.io/crates/v/bevy?style=for-the-badge&labelColor=2a3438&color=855667&label=Bevy)](https://bevyengine.org)

A [GSAP-like](https://greensock.com/gsap) animation plugin for Bevy UI.

## 🌌 Features

- ✅ Animate NodeBundle, ImageBundle, TextBundle, ButtonBundle
- 🚧 Timeline support
- 🚧 Bevy event support

## 🔮 Usage

To use this plugin, the first step is to add a dependency to your `Cargo.toml`:

```toml
[dependencies]
bevy_ui_animation = "1.0.0"
```

Add the `AnimationPlugin` to your `main.rs`:

```rust
use bevy::prelude::*;
use bevy_ui_animation::*;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationPlugin)
    .run();
```

Animate a bundle:

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

You can directly run this example by:

```bash
cargo run --example all --features="bevy/bevy_winit"
```

## ✳️ Vars

| Field              | Type                        | Default   | Description                                                                 |
| ------------------ | --------------------------- | --------- | --------------------------------------------------------------------------- |
| style              | `Option<Style>`             | `None`    | A Bevy Style component containing the destination fields to animate to.     |
| color              | `Option<UiColor>`           | `None`    | A Bevy UiColor component containing the destination fields to animate to.   |
| transform          | `Option<Transform>`         | `None`    | A Bevy Transform component containing the destination fields to animate to. |
| transform_rotation | `Option<TransformRotation>` | `None`    | A struct to rotate a Bevy Transform component around a given fixed axis.    |
| text_color         | `Option<TextColor>`         | `None`    | A struct to lerp the color of a Bevy Text component                         |
| delay              | `f32`                       | `0.0`     | Amount of delay before the animation should begin (in seconds).             |
| duration           | `f32`                       | `0.5`     | The duration of the animation (in seconds).                                 |
| ease               | `Ease`                      | `ExpoOut` | The ease function to control the rate of change during the animation.       |
| repeat             | `bool`                      | `false`   | If `true`, the animation will keep repeating.                               |
| yoyo               | `bool`                      | `false`   | If `true`, the animation will run in the opposite direction once finished.  |
| paused             | `bool`                      | `false`   | If `true`, the animation will pause itself immediately upon creation.       |

## 📜 Animatable Components

| Name      | Field            |
| --------- | ---------------- |
| Style     | position         |
|           | margin           |
|           | padding          |
|           | border           |
|           | size             |
| UiColor   | color            |
| Transform | translation      |
|           | rotation         |
|           | scale            |
| Text      | TextStyle::color |

## 📈 Ease

| Name         | 0%         | 50%        | 100%      |
| ------------ | ---------- | ---------- | --------- |
| BackIn       | `0.0`      | `-0.375`   | `1.0`     |
| BackInOut    | `0.0`      | `0.499`    | `1.0`     |
| BackOut      | `-1.19e-7` | `1.375`    | `1.0`     |
| BounceIn     | `0.0`      | `0.281`    | `1.0`     |
| BounceInOut  | `0.0`      | `0.5`      | `1.0`     |
| BounceOut    | `0.0`      | `0.718`    | `1.0`     |
| ElasticIn    | `0.0`      | `-4.29e-8` | `2.74e-6` |
| ElasticInOut | `0.0`      | `0.099`    | `1.0`     |
| ElasticOut   | `0.099`    | `1.0`      | `1.0`     |
| ExpoIn       | `0.0`      | `0.031`    | `1.0`     |
| ExpoInOut    | `0.0`      | `0.5`      | `1.0`     |
| ExpoOut      | `0.0`      | `0.968`    | `1.0`     |
| Linear       | `0.0`      | `0.5`      | `1.0`     |
| PowerIn      | `0.0`      | `0.25`     | `1.0`     |
| PowerInOut   | `0.0`      | `0.5`      | `1.0`     |
| PowerOut     | `0.0`      | `0.75`     | `1.0`     |

## ❇️ Compatibility

| `bevy_ui_animation` | `bevy` |
| ------------------- | ------ |
| `1.0`               | `0.6`  |

## 🙏 Thanks

- @djeedai for [bevy_tweening](https://crates.io/crates/bevy_tweening)
- @mockersf for [bevy_easings](https://crates.io/crates/bevy_easings)
- @PistonDevelopers for [interpolation](https://crates.io/crates/interpolation)

## 🖋 Author

miZyind <mizyind@gmail.com>

## 📇 License

Licensed under the [MIT](LICENSE) License.
