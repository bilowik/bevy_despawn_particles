An event-based plugin for the Bevy game engine that provides a simple way to add a despawn effect for 2D sprites. 
Contains a basic physics implementation or a feature for bevy_rapier integration.

```rust
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
                DespawnParticlesEvent::builder()
                    .with_fade(true)
                    .with_shrink(true)
                    .with_linvel(150.0..300.0)
                    .with_angvel([-5.0, -2.5, 2.5, 5.0])
                    .with_mass(1.0)
                    .with_lifetime(0.5..1.0)
                    .with_angular_damping(1.0)
                    .with_linear_damping(1.0)
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
```


## Examples
All the following examples can be found in the examples directory of this repository.

| `cargo run --release --example the_works`| 
|:--:|
| This example utilizes most of the parameters available. The particles fade and shrink, have a mass affected by gravity, shoot outwards and have some amount of angular velocity, and dampening.  |
|![Screencast from 2023-06-28 21-02-54](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/34e41811-261d-494d-92fd-2ef1002185fd)|

___

| `cargo run --release --example fade` | 
|:--:|
| In this example the particles are stationary and just fade, giving the visual effect of the entire sprite just fading away |
|![Screencast from 2023-06-28 21-06-27](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/4625ec7a-14b4-465b-8767-64ffa5de61c5)|
___

| `cargo run --release --example shrink` |
|:--:|
| In this example the particles are stationary and just shrink in place |
|![Screencast from 2023-06-28 21-08-21](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/28c916b6-7c28-49d8-98e1-28730ebd40d9)|


| `cargo run --release --example velocity` |
|:--:|
| In this example the particles shoot outwards from the sprite |
|![Screencast from 2023-06-28 21-17-34](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/84279fc8-e823-474a-9549-e84dfad6cb9c)|

