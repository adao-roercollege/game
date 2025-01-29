use bevy::prelude::*;

struct EntityPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

const PLAYER_SPEED: f32 = 210.0;
const CAMERA_DECAY_RATE: f32 = 4.0;

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
    .add_plugins(EntityPlugin)
    .add_systems(Startup, gen_world)
    .add_systems(FixedUpdate, advance_physics)
    .add_systems(
      RunFixedMainLoop,
      (
        player_movement.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
      ),
    )
    .run();
}

impl Plugin for EntityPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_systems(RunFixedMainLoop, update_camera);
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

  commands.spawn((Camera2d, Camera { ..default() }));
}

fn gen_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
    MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    Transform::from_xyz(0., 0., -2.),
  ));
}

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

    println!("{}", velocity.0)
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
