/// Breaks apart meshes that do not have textures, also showcases that use of the
/// DespawnMeshOverride to make Circle meshes break apart cleaner.
use bevy::prelude::*;
use bevy_color::palettes::basic::BLUE;
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
        .run();
}

fn setup(
    mut commands: Commands,
    color_materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2d::default());
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
    commands.spawn((
        MeshMaterial2d(color_materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 1.0, 0.5)))),
        Mesh2d(meshes.add(Mesh::from(RegularPolygon::new(128.0, 3)))),
        Transform {
            translation: Vec3::new(-320.0, 0.0, 0.0),
            ..default()
        },
        Marker,
    ));

    commands.spawn((
        MeshMaterial2d(color_materials.add(ColorMaterial::from(Color::from(BLUE)))),
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(
            128.0 * 2.0f32.sqrt(),
            128.0 * 2.0f32.sqrt(),
        )))),
        Transform {
            translation: Vec3::new(320.0, 0.0, 0.0),
            ..default()
        },
        Marker,
    ));

    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, -192.0, 0.0),
            ..default()
        },
        MeshMaterial2d(color_materials.add(ColorMaterial::from(Color::from(BLUE)))),
        Mesh2d(meshes.add(Mesh::from(Circle::new(128.0)))),
        Marker,
    ));

    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 192.0, 0.0),
            ..default()
        },
        MeshMaterial2d(color_materials.add(ColorMaterial::from(Color::from(BLUE)))),
        Mesh2d(meshes.add(Mesh::from(Circle::new(128.0)))),
        Marker,
        DespawnMeshOverride::faux_circle(&mut meshes, 128.0, 13),
    ));
}
