use bevy::{
    sprite::MaterialMesh2dBundle,
    prelude::shape::Box,
    prelude::shape::Icosphere,
    prelude::*,
};
use bevy_rapier2d::prelude::*;

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

#[derive(Bundle)]
pub struct CannonBallBundle {

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
        mesh: meshes.add(Mesh::from(Box::default())).into(),
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
        mesh: meshes.add(Mesh::from(Box::default())).into(),
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
    mut rigid_bodies: Query<(&mut RigidBodyForcesComponent, &mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent)>
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

            let collider = ColliderBundle {
                shape: ColliderShape::ball(0.5).into(),
                mass_properties: ColliderMassProps::Density(200.0).into(),
                material: ColliderMaterial {
                    restitution: 0.7,
                    ..Default::default()
                }.into(),
                ..Default::default()
            };
            let sprite = MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Icosphere::default())).into(),
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(5.0, 5.0, 0.0),
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::from(Color::ORANGE)),
                    ..Default::default()
                };


            commands
                .spawn_bundle(RigidBodyBundle {
                position: Vec2::new(0.0, 10.0).into(),
                forces: RigidBodyForces {
                    force: Vec2::new(1000.0, 1000.0).into(),
                    torque: 140.0,
                    // gravity_scale: 2.0,
                    ..Default::default()
                }.into(),
                ..Default::default()
            })
            .insert_bundle(sprite)
            .insert_bundle(collider)
            .insert(CannonBall {
                velocity: -50.0 * transform.translation.normalize()
            });

            barrel.time_last_fired = game_state.elapsed_time;
        }
    }


    for (mut rb_forces, mut rb_vel, rb_mprops) in rigid_bodies.iter_mut() {
        // Apply forces.
        rb_forces.force = Vec2::new(1.0, 2.0).into();
        rb_forces.torque = 0.4;

        // Apply impulses.
        rb_vel.apply_impulse(rb_mprops, Vec2::new(100.0, 200.0).into());
        rb_vel.apply_torque_impulse(rb_mprops, 80.0);
    }

    // for (mut cannonball, mut transform) in cannonball_query.iter_mut() {
    //     cannonball.force
    //      transform.translation += cannonball.velocity * TIME_STEP;

    // }



}
