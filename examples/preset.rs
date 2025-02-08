use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

pub struct MyTimer(pub Timer);

#[derive(Resource)]
pub struct MyPreset(pub DespawnParticlesPreset);

impl Default for MyPreset {
    fn default() -> Self {
        Self(
            DespawnParticlesPreset::new()
                .with_linvel(100.0..180.0)
                .with_angvel(-5.0..5.0)
                .with_fade(true)
                .with_gray(true)
                .with_shrink(true),
        )
    }
}

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
        .init_resource::<MyPreset>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_image(asset_server.load("asteroid_round.png")),
        Marker,
    ));
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    marker: Query<Entity, With<Marker>>,
    preset: Res<MyPreset>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(entity) = marker.get_single() {
            despawn_particles_event_writer.send(preset.0.clone().build(entity));
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            commands.spawn((
                Sprite::from_image(asset_server.load("asteroid_round.png")),
                Marker,
            ));
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}
