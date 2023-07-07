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

pub fn split_triangle(v: [Vec3; 3], depth: usize, output: &mut Vec<[Vec3; 3]>) {
    if depth == 0 {
        output.push(v);
    } else {
        // Get the lengths of the sides.
        let sides = [
            v[2].distance(v[3]),
            v[3].distance(v[1]),
            v[1].distance(v[2]),
        ];

        // Get the idx of the point across from the longest side
        let longest_idx =
            sides.iter().enumerate().fold(
                0usize,
                |acc, (idx, v)| {
                    if *v >= sides[acc] {
                        idx
                    } else {
                        acc
                    }
                },
            );

        // Get the halfway point of this longest side, which is between the two other points
        let p_mid = (0..3).into_iter().fold(Vec3::ZERO, |acc, curr_idx| {
            if longest_idx == curr_idx {
                // Skip, we are ignoring our selected index
                acc
            } else {
                acc + v[curr_idx]
            }
        }) / 2.0;

        // Create the two new triangles
        let mut triangles = (0..3)
            .into_iter()
            .filter(|idx| *idx != longest_idx)
            .map(|idx| [v[longest_idx], p_mid, v[idx]])
            .collect::<Vec<_>>();

        // Split the next two triangles
        for _ in 0..2 {
            split_triangle(triangles.pop().unwrap(), depth - 1, output);
        }
    }
}
