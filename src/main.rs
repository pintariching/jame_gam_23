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
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, update_camera)
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
                scale: 2.,
                ..default()
            },
            ..default()
        },
    });
}

fn update_camera(
    player_position_query: Query<&Position, With<Player>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<PlayerCamera>>,
    time: Res<Time>,
) {
    let player_position = player_position_query.get_single().unwrap();
    let (mut projection, mut transform) = camera_query.get_single_mut().unwrap();

    let target = player_position.extend(1.);
    let dir = (target - transform.translation).normalize_or_zero();

    transform.translation += dir * 500. * time.delta_seconds();
    projection.scale = 1.;
}
