use bevy::math::{Quat, Vec2};

pub fn rotate_sprite(
    &mut Transform: Mut<Transform>,

) {
    // let pos = transform.translation.truncate();
    // let target = event.position;
    // let angle = (target - pos).angle_between(pos);
    transform.rotation = Quat::from_rotation_z(angle);
}