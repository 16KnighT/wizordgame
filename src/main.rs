use bevy::{prelude::*, render::camera};
use std::f32::consts::FRAC_PI_2;

pub const CAMERA_SPEED: f32 = 15.0;
pub const CAMERA_ROTATE_DURATION: f32 = 0.1;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct CameraRotateOverTime {
    timer: Timer,
    direction: f32,
    unlocked: bool,
}

impl Default for CameraRotateOverTime {
    fn default() -> CameraRotateOverTime {
        let mut camera_rotate = CameraRotateOverTime {
            timer: Timer::from_seconds(CAMERA_ROTATE_DURATION, TimerMode::Once),
            direction: 0.0,
            unlocked: true,
        };
        camera_rotate.timer.pause();
        return camera_rotate;
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene_setup)
        .add_systems(Update, controller)
        .add_systems(Update, camera_rotation_helper)
        .init_resource::<CameraRotateOverTime>()
        .run();
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //spawns a light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });

    //Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ALICE_BLUE,
        brightness: 1.5,
    });
    
    //spawns a camera
    let camera = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 15.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).id();

    let player = commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, ..default() })),
            material: materials.add(StandardMaterial {
                    base_color: Color::GOLD,
                    ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    )).id();

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0, ..default() })),
            ..default()
        }
    );

    commands.entity(player).push_children(&[camera]);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                base_color: Color::OLIVE,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..default()
        }
    );
}

pub fn controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_rotater: ResMut<CameraRotateOverTime>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(1.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(-1.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(1.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(-1.0, 0.0, -1.0);
    }

    if camera_rotater.unlocked {
        if keyboard_input.just_pressed(KeyCode::E) {
            camera_rotater.direction = 1.0;
            camera_rotater.unlocked = false;
            camera_rotater.timer.reset();
            camera_rotater.timer.unpause();
        } else if keyboard_input.just_pressed(KeyCode::Q) {
            camera_rotater.direction = -1.0;
            camera_rotater.unlocked = false;
            camera_rotater.timer.reset();
            camera_rotater.timer.unpause();
        }
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += direction * CAMERA_SPEED * time.delta_seconds();

}

pub fn camera_rotation_helper (
    mut camera_rotater: ResMut<CameraRotateOverTime>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = player_transform_query.single_mut();

    if !(camera_rotater.timer.finished() || camera_rotater.timer.paused()) {
        camera_rotater.timer.tick(time.delta());
        player.rotate_y((FRAC_PI_2 * time.delta_seconds() *camera_rotater.direction)/CAMERA_ROTATE_DURATION);
    }

    if camera_rotater.timer.just_finished() {
        camera_rotater.unlocked = true;
    }
}