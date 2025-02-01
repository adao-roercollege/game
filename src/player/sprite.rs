use super::movement::{
  AccumulatedInput, PhysicalTranslation, PreviousPhysicalTranslation, Velocity,
};
use crate::entity::{Player /* ,PlayerBundle*/};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_player);
    // FIXME: Lets pray that this is just because im using and example ldtk map
    //.register_ldtk_entity::<PlayerBundle>("Player");
  }
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
}
