use bevy::{
    sprite::MaterialMesh2dBundle,
    // math::{Quat, Vec2},
    prelude::*,
};

use crate::TIME_STEP;
// use crate::DEBUG_MODE;

#[derive(Component)]
pub struct Cannon{
    speed: f32,
}

pub fn spawn_cannon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Box::default())).into(),
        transform: Transform {
            translation: Vec3::new(385.0, -205.0, 0.0),
            scale: Vec3::new(10.0, 5.0, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..Default::default()
    })
    .insert(Cannon {speed: 2.});
}

// pub fn launch_cannon(
//     mut query: Query<(&Cannon, &mut Transform)>,
// ) {
//     // let cannon = query.single_mut();
//     info!("launching cannon!")
// }

pub fn move_cannon(
    mut query: Query<(&Cannon, &mut Transform)>
) {
    let (cannon, mut transform) = query.single_mut();

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += -1. * cannon.speed * TIME_STEP;

    // * barrel rotation
    // transform.rotation = Quat::from_rotation_z(angle);
}