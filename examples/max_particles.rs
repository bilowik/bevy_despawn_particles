/// This showcases a potentially better way to handle breaking apart a circular texture-based
/// sprite.
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
        .add_plugins(DespawnParticlesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, tick)
        .insert_resource(DespawnParticlesConfig {
            max_particles: 320,
        })
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn(commands, asset_server);
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    marker: Query<Entity, With<Marker>>,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if !marker.is_empty() {
            for entity in marker.iter() {
                despawn_particles_event_writer.send(
                    DespawnParticlesEvent::builder()
                        .with_fade(false)
                        .with_shrink(false)
                        .with_linvel(50.0..65.0)
                        .with_angvel(-3.0..3.0)
                        .with_lifetime(15.0)
                        .with_target_num_particles(128)
                        .build(entity),
                );
            }
            timer.0 = Timer::from_seconds(1.5, TimerMode::Once);
            timer.0.reset();
        } else {
            spawn(commands, asset_server);
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("asteroid_round.png"),
            ..default()
        },
        Marker,
    ));
}
