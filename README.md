An event-based plugin for the Bevy game engine that provides a simple way to add a despawn effect for 2D sprites. 
Contains a basic physics implementation or a feature for bevy_rapier integration.

```rust
use bevy::prelude::*;
use bevy_despawn_particles::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DespawnParticlesPlugin)
        .add_system(setup.on_startup())
        .add_system(despawn)
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

fn despawn(
    mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
    entities: Query<Entity, Added<Marker>>,
) {
    if let Ok(entity) = entities.get_single() {
        despawn_particles_event_writer.send(
            DespawnParticlesEvent::builder()
                .with_fade(true) // The particles will fade as they get closer to expiration
                .with_shrink(true) // The particles will shrink as they get closer to expiration
                .with_linvel(150.0..300.0) // Random velocity between 150.0 and 300.0
                .with_angvel([-5.0, -2.5, 2.5, 5.0]) // Random angular velocity from the given list
                .with_mass(1.0) // Always 1.0
                .with_lifetime(0.3..1.0) // Random lifetime between 0.3 and 1.0
                .with_angular_damping(1.0) // Always 1.0, angular 'friction' that decelerates the particle
                .with_linear_damping(1.0) // Always 1.0, linear 'friction' that decelerates the particle
                .build(entity),
        );
    }
}

```


## Examples
All the following examples can be found in the examples directory of this repository.

| `cargo run --release --example the_works`| 
|:--:|
| This example utilizes most of the parameters available. The particles fade and shrink, have a mass affected by gravity, shoot outwards and have some amount of angular velocity, and dampening.  |
|![works](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/4b16ddc7-d923-44d3-8142-b2588ad4b410)|


___

| `cargo run --release --example fade` | 
|:--:|
| In this example the particles are stationary and just fade, giving the visual effect of the entire sprite just fading away |
|![fade](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/26156dd4-d1d6-4744-b331-d71582db659a)|

___

| `cargo run --release --example shrink` |
|:--:|
| In this example the particles are stationary and just shrink in place |
|![shrink](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/bf08c9cf-283d-41d6-8997-993e77eccb04)|



| `cargo run --release --example velocity` |
|:--:|
| In this example the particles shoot outwards from the sprite |
|![velocity](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/064cb841-278d-4d07-ab68-c708922d332b)|

| `cargo run --release --example mesh` |
|:--:|
| Can be utilized on a Mesh. Also includes usage of the faux circle mesh to replace the arguably unappealing triangles that typically make up a Cirlcle mesh |
|![Screencast from 2023-07-19 20-55-22](https://github.com/bilowik/bevy_despawn_particles/assets/43679332/3e973f76-6d71-4fd0-ae67-b5cdeab8cef3)|

