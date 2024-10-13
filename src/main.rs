use bevy::core_pipeline::bloom::BloomSettings;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use avian2d::prelude::*;
//use bevy::input::mouse::MouseMotion;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 5.;

/// Camera lerp factor.
const CAM_LERP_FACTOR: f32 = 2.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
enum Test {
    Yes,
    No,
}

//#[derive(Component)]



fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        .add_systems(Startup, (setup_scene, setup_camera))
        .add_systems(Update, (update_player, update_camera).chain())
        .run();
}




fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Player
    commands.spawn((
        Test::Yes,
        Player,
        RigidBody::Dynamic,
        Collider::circle(10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Sphere::new(10.0)).into(),
            material: materials.add(Color::srgb(3.0, 3.0, 3.0)),
            ..default()
        },
    ));

    // Debug Square
    commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20.0, 20.0)).into(),
            material: materials.add(Color::srgb(2.0, 0.1, 0.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform {
                translation: vec3(40.0, 0.0, 40.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(20.0, 20.0),
    ));
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    //let Some(info) = world.components().get_info().unwrap();
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1,
            contacts.entity2,

        );
    }
}

fn _setup_instructions(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Move the light with ZQSD or WASD.\nThe camera will smoothly track the light.",
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}



/// Update the camera position by tracking the player.
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

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}

/// Update the player position with keyboard inputs.
fn update_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    _windows: Query<&Window, With<PrimaryWindow>>,
    _windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let forward = kb_input.pressed(KeyCode::KeyW);
    let backward = kb_input(KeyCode::KeyS);
    let left = kb_input(KeyCode::KeyA);
    let right = kb_input(KeyCode::KeyD);

    // Decellerate the Player
    if player.x.abs() > 0.0 {
        player.x *= 0.95;
    }

    if player.y.abs() > 0.0 {
        player.y *= 0.95;
    }

    if forward {
        //direction.y += 1.;
        if left {
            player.y += (2. as f32).sqrt()
        }
        else if right {

        }
        else {
            player.y += 1. * PLAYER_SPEED;
        }

    }

    if kb_input.pressed(KeyCode::KeyS) {
        //direction.y -= 1.;
        player.y += -1. * PLAYER_SPEED;
    }

    /*
    if kb_input.pressed(KeyCode::KeyA) {
        //direction.x -= 1.;
        player.x += -1. * PLAYER_SPEED;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        //direction.x += 1.;
        player.x += 1. * PLAYER_SPEED;
    }
    */


    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    //let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    //player.x += move_delta.extend(0.);
}
fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
fn get_window_size(
    windows: Query<&Window, With<PrimaryWindow>>,
)  -> [f32; 2] {
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    if let Ok(window) = windows.get_single() {
        a = window.width();
        b = window.height();
        //println!("Current resolution: {}x{}", width, height);
    }
    [a, b]
}
