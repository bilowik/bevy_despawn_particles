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


// Inlining the below two functions to avoid an allocation for the returned array
// I could be wrong, but we cannot return a value that is stored on the stack bc on return the 
// stack ptr moves back to its previous position, meaning that Rust must be allocating these arrays
// on the heap in order to return them.

#[inline(always)]
pub fn float32x3_triangle_centroid(tri: [[f32; 3]; 3]) -> [f32; 3] {
    // We could collcet into a SmallVec but would it be worth the simpler code if we are just gonna
    // change it back into a [f32; 3] anyway?
    let mut centroid = [0.0; 3];
    for idx in 0..3 {
        centroid[idx] = (tri[0][idx] + tri[1][idx] + tri[2][idx]) / 3.0;
    }
    centroid
    //(0..3).map(|idx| (tri[0][idx] + tri[1][idx] + tri[2][idx]) / 3.0).collect()
}


#[inline(always)]
pub fn float32x3_sub(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

#[allow(unused)]
pub fn debug_meshes(meshes: &[Mesh]) {
    for mesh in meshes {
        println!("Pos: {:?}", mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()); 
    }
}
