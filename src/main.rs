use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use entity::*;
use player::{camera::CameraPlugin, movement::MovementPlugin, sprite::PlayerSpritePlugin};
use world::load::MapPlugin;

mod entity;
mod player;
mod world;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            name: "bevy.app".to_string().into(),
            ..default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(LdtkPlugin)
    .add_plugins((MapPlugin, PlayerSpritePlugin, MovementPlugin, CameraPlugin))
    .run();
}
