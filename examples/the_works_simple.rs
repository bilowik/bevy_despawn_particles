//! Primary purpose of this example is to show the simplest setup to demonstrate the crate.
//! For a better visual example see the example "the_works"
use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DespawnParticlesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, despawn)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_image(asset_server.load("asteroid_round.png")),
        Marker,
    ));
}

fn despawn(
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    entities: Query<Entity, Added<Marker>>,
) {
    if let Ok(entity) = entities.get_single() {
        despawn_particles_event_writer.send(
            DespawnParticlesEvent::builder()
                .with_fade(true)
                .with_shrink(true)
                .with_linvel(150.0)
                .with_angvel([-5.0, -2.5, 2.5, 5.0])
                .with_mass(1.0)
                .with_lifetime(1.0)
                .with_angular_damping(1.0)
                .with_linear_damping(1.0)
                .with_gray(true)
                .build(entity),
        );
    }
}
