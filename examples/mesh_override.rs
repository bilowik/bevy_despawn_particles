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
        .add_plugin(DespawnParticlesPlugin)
        .add_system(setup.on_startup())
        .add_system(tick)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn(commands, meshes, asset_server);
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
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
                        .with_linvel(100.0..150.0)
                        .with_angvel(-3.0..3.0)
                        .with_lifetime(1.0)
                        .with_angular_damping(5.0)
                        .with_linear_damping(8.0)
                        .build(entity),
                );
            }
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            spawn(commands, meshes, asset_server);
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,

) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("asteroid_round.png"), 
            ..default()
        },
        DespawnMeshOverride::faux_circle(&mut meshes, 64.0, 13),
        Marker
    ));
}
