// Also acts as a test to ensure that the alpha of the underlying sprite is being utilized
// correctly
use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

pub struct MyTimer(pub Timer);

impl Default for MyTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DespawnParticlesPlugin)
        .add_system(setup.on_startup())
        .add_system(tick)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("asteroid_round.png"),
            ..default()
        })
        .insert(Marker);
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    marker: Query<Entity, With<Marker>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(entity) = marker.get_single() {
            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder(entity)
                    .with_angvel(-5.0..=5.0)
                    .with_linvel(150.0..=350.0)
                    .with_lifetime(1.0)
                    .with_linear_damping(1.0)
                    .with_angular_damping(1.0)
                    .with_mass(1.0..=15.0)
                    .build(),
            );
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("asteroid_round.png"),
                    ..default()
                })
                .insert(Marker);
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}
