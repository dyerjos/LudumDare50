use bevy::{
    sprite::MaterialMesh2dBundle,
    // math::{Quat, Vec2},
    prelude::*,
};

use crate::TIME_STEP;
// use crate::DEBUG_MODE;
use crate::GameState;

const CANNON_SPEED: f32 = 2.0;
const GRAVITY: f32 = 10.0;

#[derive(Component)]
pub struct Cannon {
    id: u16,
    // time_last_fired: f64,
    // speed: f32,
}


#[derive(Component)]
pub struct Barrel {
    id: u16,
    angle: f32,
    angle_change: f32,
    time_last_fired: f64,
}
#[derive(Component)]
pub struct CannonBall {
    // id: u16,
    velocity: Vec3,
}

pub fn spawn_cannon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    game_state.number_of_cannons += 1;
    info!("spawning cannon {}", game_state.number_of_cannons);

    // * cannon body
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
    .insert(Cannon {
        // speed: 2.,
        id: game_state.number_of_cannons,
    });

    // * rotating cannon barrel
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Box::default())).into(),
        transform: Transform {
            translation: Vec3::new(385.0, -200.0, 0.0),
            rotation: Quat::from_rotation_z(0.0),
            scale: Vec3::new(8.0, 5.0, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..Default::default()
    })
    .insert(Barrel {
        id: game_state.number_of_cannons,
        angle: 0.0,
        angle_change: -0.001,
        time_last_fired: 0.,
    });
}
// (Without<Cannon>, Without<CannonBall>, Without<Barrel>)
pub fn cannon_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cannon_query: Query<(&mut Cannon, &mut Transform), (Without<CannonBall>, Without<Barrel>)>,
    mut cannonball_query: Query<(&mut CannonBall, &mut Transform), (Without<Cannon>, Without<Barrel>)>,
    mut barrel_query: Query<(&mut Barrel, &mut Transform), (Without<Cannon>, Without<CannonBall>)>,
    game_state: Res<GameState>,
) {
    for (cannon, mut transform) in cannon_query.iter_mut() {
        // * move selected cannon
        let translation = &mut transform.translation;
        translation.x += -1. * CANNON_SPEED * TIME_STEP;
    }

    for (mut barrel, mut transform) in barrel_query.iter_mut() {
        // * move selected barrel
        let translation = &mut transform.translation;
        translation.x += -1. * CANNON_SPEED * TIME_STEP;

        // * barrel rotation
        let new_angle: f32;
        if barrel.angle == -1.2 {
            barrel.angle_change = 0.001
        } else if barrel.angle == 0.0 {
            barrel.angle_change = -0.001
        }
        new_angle = (barrel.angle + barrel.angle_change).clamp(-1.2, 0.0);
        barrel.angle = new_angle;
        transform.rotation = Quat::from_rotation_z(new_angle);

        // * check if cannon can fire again
        if barrel.time_last_fired + 1. <= game_state.elapsed_time {
            info!("cannon {} fires a shot!", barrel.id);
            // * spawn cannonball here
            // let barrel_pos = transform.translation.truncate();
            // let base_pos = transform.with_rotation(-transform.rotation);
            // let angle = (barrel_pos - base_pos).angle_between(base_pos);

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(5.0, 5.0, 0.0),
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::from(Color::ORANGE)),
                    ..Default::default()
                })
                .insert(CannonBall {
                    velocity: -50.0 * transform.translation.normalize()
                });

            barrel.time_last_fired = game_state.elapsed_time;
        }
    }


    for (cannonball, mut transform) in cannonball_query.iter_mut() {

        transform.translation += cannonball.velocity * TIME_STEP;
    }

}

// pub fn firing_solution(distance: f32, ) {
//     info!("figure this out yo")
// }