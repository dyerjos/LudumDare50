
use bevy::{
    // sprite::collide_aabb::{collide, Collision},
    prelude::*,
};
use bevy_inspector_egui::Inspectable;

use crate::TIME_STEP;
// use crate::DEBUG_MODE;

#[derive(Component)]
pub struct Player{
    speed: f32,
}

#[derive(Inspectable, Default)]
pub struct PlayerData {
    player: Option<Entity>,
}

pub fn spawn_player(
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
) {
    let player = commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -205.0, 0.0),
                scale: Vec3::new(10.0, 5.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 100.0})
        .id();
    player_data.player = Some(player);

}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();
    // let mut direction = Vec2::ZERO;
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Firing cannon!")
    }
    // if direction == Vec2::ZERO {
    //     return;
    // }
    // if keyboard_input.pressed(KeyCode::Up) {
    //     direction.y += 1.0;
    //     // this will be cannon up
    // }
    // if keyboard_input.pressed(KeyCode::Down) {
    //     direction.y -= 1.0;
    //     // this will be cannon down
    // }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction * player.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(370.).max(-370.);
}
