use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::planet::Planet;

const GRAVITATIONAL_CONSTANT: f32 = 10.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(FixedUpdate, apply_gravity);
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    mesh: ColorMesh2dBundle,
    rigid_body: RigidBody,
    collider: Collider,
    position: Position,
    locked_axes: LockedAxes,
    external_force: ExternalForce,
    mass: Mass,
    gravity: GravityScale,
    linear_velocity: LinearVelocity,
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PlayerBundle {
        player: Player,
        mesh: ColorMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::ball(5.),
        position: Position(Vec2::new(1200., 0.)),
        locked_axes: LockedAxes::new().lock_rotation(),
        external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
        mass: Mass(100.),
        gravity: GravityScale(0.),
        linear_velocity: LinearVelocity(Vec2::new(0., 320.)),
    });
}

fn apply_gravity(
    mut player_query: Query<(&Player, &Position, &Mass, &mut ExternalForce)>,
    planet_query: Query<(&Planet, &Position, &Mass)>,
) {
    let (_, player_position, player_mass, mut external_force) =
        player_query.get_single_mut().unwrap();
    let (_, planet_position, planet_mass) = planet_query.get_single().unwrap();

    let grav_direction = planet_position.0 - player_position.0;
    let distance_squared = grav_direction.length_squared();

    let force = GRAVITATIONAL_CONSTANT * ((player_mass.0 * planet_mass.0) / distance_squared);

    let direction_norm = grav_direction.normalize();
    let force_vec = direction_norm * force;

    external_force.apply_force(force_vec);
}
