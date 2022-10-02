use crate::sdk::classes::Vec2;
use crate::sdk::surface::Vertex;
use crate::Vec3;

/// Converts view angles to a forward vector
pub fn forward_angle_vectors(angles: Vec3) -> Vec3 {
    let cp = angles.x.to_radians().cos();
    let sp = angles.x.to_radians().sin();
    let cy = angles.y.to_radians().cos();
    let sy = angles.y.to_radians().sin();
    Vec3 {
        x: cp * cy,
        y: cp * sy,
        z: -sp,
    }
}

pub fn rotate_vertex(origin: Vec2, vertex: Vertex, angle: f32) -> Vertex {
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();

    Vertex::pos(Vec2 {
        x: origin.x + (vertex.position.x - origin.x) * c - (vertex.position.y - origin.y) * s,
        y: origin.y + (vertex.position.x - origin.x) * s + (vertex.position.y - origin.y) * c,
    })
}
