use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(Vec3);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(Vec2);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(Vec3);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(Vec3);

const PLAYER_SPEED: f32 = 210.0;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, advance_physics);
    app.add_systems(
      RunFixedMainLoop,
      (
        player_movement.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
      ),
    );
  }
}

fn advance_physics(
  fixed_time: Res<Time<Fixed>>,
  mut query: Query<(
    &mut PhysicalTranslation,
    &mut PreviousPhysicalTranslation,
    &mut AccumulatedInput,
    &Velocity,
  )>,
) {
  for (mut current_physical_translation, mut previous_physical_translation, mut input, velocity) in
    query.iter_mut()
  {
    previous_physical_translation.0 = current_physical_translation.0;
    current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

    input.0 = Vec2::ZERO;
  }
}

fn interpolate_rendered_transform(
  fixed_time: Res<Time<Fixed>>,
  mut query: Query<(
    &mut Transform,
    &PhysicalTranslation,
    &PreviousPhysicalTranslation,
  )>,
) {
  for (mut transform, current_physical_translation, previous_physical_translation) in
    query.iter_mut()
  {
    let previous = previous_physical_translation.0;
    let current = current_physical_translation.0;
    let alpha = fixed_time.overstep_fraction();

    let rendered_translation = previous.lerp(current, alpha);
    transform.translation = rendered_translation;
  }
}

// TODO: ai hand movement
fn player_movement(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut query: Query<(&mut AccumulatedInput, &mut Velocity)>,
) {
  for (mut input, mut velocity) in query.iter_mut() {
    if keyboard_input.pressed(KeyCode::KeyW) {
      input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
      input.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
      input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
      input.x += 1.0;
    }
    velocity.0 = input.extend(0.0).normalize_or_zero() * PLAYER_SPEED;
  }
}
