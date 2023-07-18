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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn(Camera2dBundle::default());
    spawn(commands, atlases, asset_server);
    
}

fn tick(
    mut timer: Local<MyTimer>,
    time: Res<Time>,
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    marker: Query<Entity, With<Marker>>,
    atlases: ResMut<Assets<TextureAtlas>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Ok(entity) = marker.get_single() {
            despawn_particles_event_writer.send(
                DespawnParticlesEvent::builder()
                    .with_fade(true)
                    .with_linvel(50.0)
                    .with_angvel(-0.2..0.2)
                    .with_lifetime(1.0)
                    .with_angular_damping(5.0)
                    .with_linear_damping(10.0)
                    .build(entity),
            );
            timer.0 = Timer::from_seconds(1.2, TimerMode::Once);
            timer.0.reset();
        } else {
            spawn(commands, atlases, asset_server);
            timer.0 = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}


fn spawn(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                ..default()
            },
            texture_atlas: atlases.add(TextureAtlas::from_grid(
                    asset_server.load("asteroid_sheet_test.png"),
                    Vec2::splat(64.),
                    1,
                    4,
                    None,
                    None,
            )),
            ..default()
        })
        .insert(Marker);

}
