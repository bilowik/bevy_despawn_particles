use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;
use bevy_variable_property::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

pub struct MyTimer(pub Timer);

pub struct RandomSize(pub Property<Vec2>);

pub struct RandomScale(pub Property<Vec3>);

impl Default for RandomSize {
    fn default() -> Self {
        Self((Vec2::splat(96.)..Vec2::splat(512.)).into())
    }
}

impl Default for RandomScale {
    fn default() -> Self {
        Self((Vec3::splat(0.3)..Vec3::splat(3.0)).into())
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
        .add_plugin(DespawnParticlesPlugin)
        .add_system(setup.on_startup())
        .add_system(tick)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn(
        commands,
        asset_server,
        RandomSize::default().0.get_value(),
        RandomScale::default().0.get_value(),
    );
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    marker: Query<Entity, With<Marker>>,
    size: Local<RandomSize>,
    scale: Local<RandomScale>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(entity) = marker.get_single() {
            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    .with_fade(true)
                    .with_linvel(150.0)
                    .with_angvel(-1.2..1.2)
                    .with_lifetime(1.0)
                    .with_angular_damping(5.0)
                    .with_linear_damping(0.5)
                    .with_gray(true)
                    .build(entity),
            );
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            spawn(
                commands,
                asset_server,
                size.0.get_value(),
                scale.0.get_value(),
            );
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, size: Vec2, scale: Vec3) {
    commands
        .spawn(SpriteBundle {
            transform: Transform { scale, ..default() },
            texture: asset_server.load("asteroid_round.png"),
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            ..default()
        })
        .insert(Marker);
}
