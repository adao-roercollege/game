use bevy::{
  prelude::{Bundle, Component},
  sprite::Sprite,
};
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player;

// FIXME: Lets pray that this is just because im using and example ldtk map
/*
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
  player: Player,
  #[sprite_sheet]
  sprite_sheet: Sprite,
  #[grid_coords]
  grid_coords: GridCoords,
}
*/
