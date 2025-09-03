mod core;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::math::primitives::{Cuboid, Plane3d};
use crate::core::*;
use crate::core::input::{ KeyBinding };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let binding = KeyBinding::from_string("Ctrl + A");
    if let Some(binding) = binding {
        println!("binding: {}", binding);
    }

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        Visibility::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 16.0 / 9.0,
        }),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.7, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        Rotates,
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}

#[derive(Component)]
struct Rotates;

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    let dt = time.delta().as_secs_f32();
    for mut transform in &mut query {
        transform.rotate_y(1.0 * dt);
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let dt = time.delta().as_secs_f32();
    const SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 0.002;

    // Detect movement flags based on input
    let keyboard_movement = detect_keyboard_input(&keys);
    let (mouse_movement, y_rotation) = detect_mouse_input(&window_query, &mut mouse_motion_events, &buttons);

    // Update camera position based on movement flags
    update_camera(
        keyboard_movement | mouse_movement,
        y_rotation * ROTATION_SPEED,
        &mut query,
        SPEED,
        dt,
    );
}

/// Detect movement flags based on keyboard input
fn detect_keyboard_input(keys: &Res<ButtonInput<KeyCode>>) -> u8 {
    // Define movement bits (bitflags could also be used for clarity)
    const MOVE_FORWARD: u8 = 1 << 0;
    const MOVE_BACKWARD: u8 = 1 << 1;
    const MOVE_LEFT: u8 = 1 << 2;
    const MOVE_RIGHT: u8 = 1 << 3;

    let mut movement_flags = 0;

    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyZ) {
        movement_flags |= MOVE_FORWARD;
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        movement_flags |= MOVE_BACKWARD;
    }
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyQ) {
        movement_flags |= MOVE_LEFT;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        movement_flags |= MOVE_RIGHT;
    }

    movement_flags
}

/// Detect movement flags based on mouse position near screen edges
fn detect_mouse_input(
    window_query: &Query<&Window>,
    mouse_motion_events: &mut EventReader<MouseMotion>,
    buttons: &Res<ButtonInput<MouseButton>>
) -> (u8, f32) {
    const MOVE_FORWARD: u8 = 1 << 0;
    const MOVE_BACKWARD: u8 = 1 << 1;
    const MOVE_LEFT: u8 = 1 << 2;
    const MOVE_RIGHT: u8 = 1 << 3;

    let mut movement_flags = 0;
    let mut y_rotation = 0.0;

    if buttons.pressed(MouseButton::Middle) {
        for event in mouse_motion_events.read() {
            println!("Mouse motion delta: {:?}", event.delta.x);
            y_rotation -= event.delta.x;
        }
    }
    else
    {
        if let Ok(window) = window_query.single() {
            const BORDER_THRESHOLD: f32 = 50.0;
            if let Some(cursor_position) = window.cursor_position() {
                let width = window.resolution.width() as f32;
                let height = window.resolution.height() as f32;

                if cursor_position.y >= height - BORDER_THRESHOLD {
                    movement_flags |= MOVE_BACKWARD;
                }
                if cursor_position.y <= BORDER_THRESHOLD {
                    movement_flags |= MOVE_FORWARD;
                }
                if cursor_position.x <= BORDER_THRESHOLD {
                    movement_flags |= MOVE_LEFT;
                }
                if cursor_position.x >= width - BORDER_THRESHOLD {
                    movement_flags |= MOVE_RIGHT;
                }
            }
        }
    }

    (movement_flags, y_rotation)
}


/// Update the camera's position based on movement flags
fn update_camera(
    movement_flags: u8,
    y_rotation: f32,
    query: &mut Query<&mut Transform, With<Camera3d>>,
    speed: f32,
    dt: f32,
) {
    // Define movement bits
    const MOVE_FORWARD: u8 = 1 << 0;
    const MOVE_BACKWARD: u8 = 1 << 1;
    const MOVE_LEFT: u8 = 1 << 2;
    const MOVE_RIGHT: u8 = 1 << 3;

    for mut transform in query.iter_mut() {
        // Compute forward and right vectors projected on the X-Z plane
        let forward = transform.forward();
        let right = transform.right();

        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        let mut translation = transform.translation;

        // Apply camera movement based on movement flags
        if movement_flags & MOVE_FORWARD != 0 {
            translation += forward_xz * speed * dt;
        }
        if movement_flags & MOVE_BACKWARD != 0 {
            translation -= forward_xz * speed * dt;
        }
        if movement_flags & MOVE_LEFT != 0 {
            translation -= right_xz * speed * dt;
        }
        if movement_flags & MOVE_RIGHT != 0 {
            translation += right_xz * speed * dt;
        }

        const SMOOTHNESS: f32 = 0.9;
        transform.translation = transform.translation.lerp(translation, SMOOTHNESS);


        // Apply Y-axis camera rotation
        if y_rotation.abs() > f32::EPSILON {
            transform.rotate_y(y_rotation * dt);
        }
    }
}


