use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use planet::PlanetPlugin;
use player::{Player, PlayerPlugin};

mod planet;
mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((PlanetPlugin, PlayerPlugin))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update_camera)
        .run();
}

#[derive(Bundle)]
struct PlayerCameraBundle {
    player_camera: PlayerCamera,
    camera: Camera2dBundle,
}

#[derive(Component)]
struct PlayerCamera;

fn setup(mut commands: Commands) {
    commands.spawn(PlayerCameraBundle {
        player_camera: PlayerCamera,
        camera: Camera2dBundle {
            transform: Transform::from_xyz(1200., 0., 1.),
            projection: OrthographicProjection {
                scale: 1.,
                ..default()
            },
            ..default()
        },
    });
}

fn update_camera(
    player_position_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    time: Res<Time>,
) {
    let player_position = player_position_query.get_single().unwrap().extend(1.);
    let mut transform = camera_query.get_single_mut().unwrap();

    transform.translation = transform
        .translation
        .lerp(player_position, time.delta_seconds() * 10.);
}
