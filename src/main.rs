use bevy::prelude::*;

const WORLD_RADIUS: i32 = 12;
const BLOCK_SIZE: f32 = 1.0;

#[derive(Component)]
struct Voxel;

#[derive(Component)]
struct PlayerCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minecraft Rust Edition".into(),
                resolution: (960_f32, 540_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, generate_world))
        .add_systems(Update, (look_controls, movement_controls))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE));
    let grass = materials.add(Color::srgb(0.25, 0.58, 0.22));
    let dirt = materials.add(Color::srgb(0.42, 0.28, 0.16));

    for x in -WORLD_RADIUS..=WORLD_RADIUS {
        for z in -WORLD_RADIUS..=WORLD_RADIUS {
            let height = terrain_height(x, z);
            for y in 0..=height {
                let material = if y == height { grass.clone() } else { dirt.clone() };
                commands.spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE,
                    ),
                    Voxel,
                ));
            }
        }
    }
}

fn terrain_height(x: i32, z: i32) -> i32 {
    let wave = ((x as f32 * 0.32).sin() + (z as f32 * 0.27).cos()) * 1.4;
    (2.0 + wave).round().clamp(0.0, 4.0) as i32
}

fn look_controls(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    if !mouse.pressed(MouseButton::Left) { return; }
    let mut delta = Vec2::ZERO;
    for event in motion.read() { delta += event.delta; }
    if delta == Vec2::ZERO { return; }

    for mut transform in &mut camera {
        let sensitivity = 0.0025 * time.delta_secs().max(0.016) / 0.016;
        let yaw = Quat::from_rotation_y(-delta.x * sensitivity);
        let right = transform.rotation * Vec3::X;
        let pitch = Quat::from_axis_angle(right, -delta.y * sensitivity);
        transform.rotation = yaw * pitch * transform.rotation;
    }
}

fn movement_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    for mut transform in &mut camera {
        let mut direction = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();
        if keys.pressed(KeyCode::KeyW) { direction += *forward; }
        if keys.pressed(KeyCode::KeyS) { direction -= *forward; }
        if keys.pressed(KeyCode::KeyD) { direction += *right; }
        if keys.pressed(KeyCode::KeyA) { direction -= *right; }
        if keys.pressed(KeyCode::Space) { direction += Vec3::Y; }
        if keys.pressed(KeyCode::ShiftLeft) { direction -= Vec3::Y; }
        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * 5.0 * time.delta_secs();
        }
    }
}
