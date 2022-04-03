use bevy::{
    // sprite::collide_aabb::{collide, Collision},
    prelude::*,
};

// use crate::DEBUG_MODE;

#[derive(Component)]
enum Collider {
    // Solid,
    Destructible,
    // Player,
}

#[derive(Component)]
pub struct City {
    id: u8,
}

pub fn spawn_cities(
    mut commands: Commands,
) {
    let cities = vec![-360., -310., -270., -220., -170.];
    let mut city_id = 0;
    for city in cities {
        city_id += 1;
        let center = city;
        // left building
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(center - 11., -195.0, 0.0),
                    scale: Vec3::new(12.0, 24.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::Destructible)
            .insert(City {id: 1});
        // right building
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(center + 12., -197.0, 0.0),
                    scale: Vec3::new(10.0, 20.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::Destructible)
            .insert(City {id: 1});
        // middle building
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(center, -192.0, 0.0),
                    scale: Vec3::new(15.0, 30.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::NAVY,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::Destructible)
            .insert(City {id: city_id});
    }
}

