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
    color_materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_meshes(commands, color_materials, meshes);
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    color_materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    marker: Query<Entity, With<Marker>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if !marker.is_empty() {
            for entity in marker.iter() {
                despawn_particles_event_writer.send(
                    DespawnParticlesEvent::builder()
                        .with_fade(false)
                        .with_shrink(false)
                        .with_linvel(150.0..250.0)
                        .with_angvel(0.0)
                        .with_lifetime(1.0)
                        .with_angular_damping(5.0)
                        .with_linear_damping(8.0)
                        .build(entity),
                );
            }
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            spawn_meshes(commands, color_materials, meshes);
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}

fn spawn_meshes(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn(ColorMesh2dBundle {
            material: color_materials.add(ColorMaterial::from(Color::BLUE)),
            mesh: meshes
                .add(Mesh::from(shape::RegularPolygon::new(128.0, 3)))
                .into(),
            transform: Transform {
                translation: Vec3::new(-256.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Marker);
    commands
        .spawn(ColorMesh2dBundle {
            material: color_materials.add(ColorMaterial::from(Color::BLUE)),
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(128.0, 128.0))))
                .into(),
            transform: Transform {
                translation: Vec3::new(256.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Marker);
}
