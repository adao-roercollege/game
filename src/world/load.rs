use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(LevelSelection::index(0));
    app.add_systems(Startup, load_world);
  }
}

fn load_world(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(LdtkWorldBundle {
    // TODO: change this to the actual map
    ldtk_handle: asset_server
      .load("world/Typical_TopDown_example.ldtk")
      .into(),
    ..Default::default()
  });
}
