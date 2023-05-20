use bevy::prelude::*;

/// Used to get the angle between two points where the reference point is source
/// IE: Imagine source is (0,0) and target is some (x, y) on the coordinate axis, the angle
/// calculated here is 0 radians where target is (x, y > 0) and is exactly PI radians 
/// where target is (x, y < 0) and so on
pub fn angle_between(source: Vec2, target: Vec2) -> f32 {
    f32::atan2(target.x - source.x, target.y - source.y)
}

/// NOTE: Ignores the z value
pub fn angle_between3(source: Vec3, target: Vec3) -> f32 {
    angle_between(source.truncate(), target.truncate())
}
