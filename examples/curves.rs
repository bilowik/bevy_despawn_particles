use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;

#[derive(Component, Default)]
pub struct Linear;

#[derive(Component, Default)]
pub struct Log;

#[derive(Component, Default)]
pub struct Exp;

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
    spawn(commands, asset_server);
}

static CURVES: [Curve; 3] = [Curve::Linear, Curve::Exp, Curve::Log];

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    linear: Query<Entity, With<Linear>>,
    log: Query<Entity, With<Log>>,
    exp: Query<Entity, With<Exp>>,
    mut counter: Local<usize>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Some(((linear_entity, log_entity), exp_entity)) = linear.get_single()
            .ok()
            .zip(log.get_single().ok())
            .zip(exp.get_single().ok())
        {
            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    //.with_fade(Some(Curve::Linear))
                    .with_shrink(Some(Curve::Linear))
                    .with_lifetime(2.0)
                    .build(linear_entity),
            );

            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    //.with_fade(Some(Curve::Log))
                    .with_shrink(Some(Curve::Log))
                    .with_lifetime(2.0)
                    .build(log_entity),
            );

            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    //.with_fade(Some(Curve::Exp))
                    .with_shrink(Some(Curve::Exp))
                    .with_lifetime(2.0)
                    .build(exp_entity),
            );
            timer.0 = Timer::from_seconds(3.0, TimerMode::Once);
            timer.0.reset();
            *counter = *counter + 1;
            if *counter >= CURVES.len() {
                *counter = 0;
            }
        } else {
            spawn(commands, asset_server);
            timer.0 = Timer::from_seconds(2.5, TimerMode::Once);
        }
    }
}


fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("asteroid_round.png"),
            transform: Transform::from_translation(Vec3::new(-128.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Linear);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("asteroid_round.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Log);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("asteroid_round.png"),
            transform: Transform::from_translation(Vec3::new(128.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Exp);
}
