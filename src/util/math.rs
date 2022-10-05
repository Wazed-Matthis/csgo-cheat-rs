use crate::sdk::classes::Vec2;
use crate::sdk::surface::Vertex;
use crate::Vec3;

/// Converts view angles to a forward vector
pub fn heading(angles: Vec3) -> Vec3 {
    let x_rad = angles.x.to_radians();
    let y_rad = angles.y.to_radians();

    let cp = x_rad.cos();
    let sp = x_rad.sin();
    let cy = y_rad.cos();
    let sy = y_rad.sin();
    Vec3 {
        x: cp * cy,
        y: cp * sy,
        z: -sp,
    }
}

pub fn calculate_angle_to_entity(entity: Vec3, local_origin: Vec3) -> (f32, f32) {
    let delta = entity - local_origin;
    (
        delta.y.atan2(delta.x).to_degrees(),
        (-delta.z).atan2(delta.x.hypot(delta.y)).to_degrees(),
    )
}

pub fn rotate_vertex(origin: Vec2, vertex: Vertex, angle: f32) -> Vertex {
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();

    Vertex::pos(Vec2 {
        x: origin.x + (vertex.position.x - origin.x) * c - (vertex.position.y - origin.y) * s,
        y: origin.y + (vertex.position.x - origin.x) * s + (vertex.position.y - origin.y) * c,
    })
}
