use bevy::{
    // sprite::collide_aabb::{collide, Collision},
    prelude::*,
};

use crate::DEBUG_MODE;

#[derive(Component)]
enum Collider {
    Solid,
    // Destructible,
    // Player,
}

#[derive(Component)]
pub struct Wall;

pub fn spawn_walls(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let wall_thickness= 10.0;
    let bounds = Vec2::new(760.0, 425.0);
    // // left
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(-bounds.x / 2.0, 0.0, -1.0),
    //             scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
    //             ..Default::default()
    //         },
    //         sprite: Sprite {
    //             color: wall_color,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid)
    //     .insert(Wall);
    // // right
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
    //             scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
    //             ..Default::default()
    //         },
    //         sprite: Sprite {
    //             color: wall_color,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid)
    //     .insert(Wall);

    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(Wall);
}




