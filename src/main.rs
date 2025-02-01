use bevy::prelude::*;
use entity::*;
use player::{camera::*, movement::*};

mod entity;
mod player;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        name: "bevy.app".to_string().into(),
        ..default()
      }),
      ..default()
    }))
    .add_plugins((PhysicsPlugin, CameraPlugin))
    .add_systems(Startup, spawn_player)
    .run();
}

fn spawn_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((
    Player,
    Mesh2d(meshes.add(Circle::new(25.))),
    MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
    //Sprite::from_image(asset_server.load("player.png")),
    Transform::from_scale(Vec3::splat(0.3)),
    AccumulatedInput::default(),
    Velocity::default(),
    PhysicalTranslation::default(),
    PreviousPhysicalTranslation::default(),
  ));

  commands.spawn((Camera2d, Camera { ..default() }));
}
