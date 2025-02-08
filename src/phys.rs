use bevy_ecs::{
    component::Component,
    reflect::ReflectComponent,
    system::{Local, Query, Res, Resource},
};
use bevy_reflect::Reflect;
use bevy_time::{Time, Timer, TimerMode};

use bevy_math::{Quat, Vec2};
use bevy_transform::components::Transform;

#[derive(Component, Default, Reflect, Copy, Clone)]
#[reflect(Component)]
pub struct Velocity {
    pub angvel: f32,
    pub linvel: Vec2,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Damping {
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct AdditionalMassProperties(f32);

// This helps us avoid less conditional code compilation
impl From<f32> for AdditionalMassProperties {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

#[derive(Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0.0, -150.0))
    }
}

#[derive(Resource)]
pub struct PhysTimeStep(pub f32);

impl Default for PhysTimeStep {
    fn default() -> Self {
        Self(1.0 / 60.0)
    }
}

pub(crate) struct PhysTimer {
    pub timer: Timer,
    pub last_run: f32,
}

impl Default for PhysTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating),
            last_run: 0.0,
        }
    }
}

pub(crate) fn phys_tick(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &Damping,
        &AdditionalMassProperties,
    )>,
    mut phys_timer: Local<PhysTimer>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    phys_timer.timer.tick(time.delta());
    if phys_timer.timer.just_finished() {
        let elapsed = time.elapsed_secs() - phys_timer.last_run;
        phys_timer.last_run = time.elapsed_secs();
        query.par_iter_mut().for_each(|(mut t, mut v, d, m)| {
            v.linvel *= 1.0 / (1.0 + (elapsed * d.linear_damping));
            v.angvel *= 1.0 / (1.0 + (elapsed * d.angular_damping));

            // [m.0.clamp(0.0, 1.0).ceil()] returns 1.0 if mass is non-zero, otherwise 0.0.
            // If an object has 0 mass, then gravity should not apply to it.
            v.linvel += gravity.0 * elapsed * m.0.clamp(0.0, 1.0).ceil();

            t.translation += (v.linvel * elapsed).extend(0.0);
            t.rotation = t.rotation * Quat::from_rotation_z(v.angvel * elapsed);
        });
    }
}
