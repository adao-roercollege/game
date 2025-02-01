use crate::entity::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

const CAMERA_DECAY_RATE: f32 = 4.0;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(RunFixedMainLoop, update_camera);
  }
}

fn update_camera(
  mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
  player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
  time: Res<Time>,
) {
  let Ok(mut camera) = camera.get_single_mut() else {
    return;
  };

  let Ok(player) = player.get_single() else {
    return;
  };

  let Vec3 { x, y, .. } = player.translation;
  let direction = Vec3::new(x, y, camera.translation.z);

  camera
    .translation
    .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
