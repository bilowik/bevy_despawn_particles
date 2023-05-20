use bevy::prelude::*;


#[derive(Component, Default, Reflect, FromReflect, Copy, Clone)]
#[reflect(Component)]
pub struct Velocity {
    pub angvel: f32,
    pub linvel: Vec2,
}

#[derive(Resource)]
pub struct Friction { 
    pub lin: f32,
    pub ang: f32,
}

impl Default for Friction {
    fn default() -> Self {
        Friction {
            lin: 1.0,
            ang: 1.0,
        }
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
            last_run: 0.0
        }
    }
}

pub(crate) fn phys_tick(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    mut phys_timer: Local<PhysTimer>,
    time: Res<Time>,
    friction: Res<Friction>,
) {
    if phys_timer.timer.just_finished() {
        let elapsed = time.elapsed_seconds() - phys_timer.last_run;
        phys_timer.last_run = time.elapsed_seconds();
        for (mut t, mut v) in query.iter_mut() {
            t.translation += (v.linvel * elapsed).extend(0.0);
            t.rotation = t.rotation + Quat::from_rotation_x(v.angvel * elapsed);
            v.linvel = v.linvel - (v.linvel * elapsed * friction.lin);
            v.angvel = v.angvel - (v.angvel * elapsed * friction.ang);
        }
    }
}
