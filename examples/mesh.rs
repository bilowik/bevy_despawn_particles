use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::render_resource::PipelineCache};
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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32.0, 32.0)))).into(),
            material: color_materials.add(ColorMaterial::from(Color::PURPLE)),
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
    cached_pipeline: Res<PipelineCache>,

) {
    for pipeline in cached_pipeline.pipelines() {
        println!("{:?}", pipeline.descriptor);
    }

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(entity) = marker.get_single() {
            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    .with_fade(true)
                    .build(entity),
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
