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
        .add_systems(PostUpdate, update_camera.after(PhysicsSet::Sync))
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

    let dir = player_position - transform.translation;

    transform.translation += dir * 2. * time.delta_seconds();
}
