
use bevy::{
    // diagnostic::FrameTimeDiagnosticsPlugin,
    // diagnostic::LogDiagnosticsPlugin,
    // diagnostic::EntityCountDiagnosticsPlugin,
    sprite::MaterialMesh2dBundle,
    prelude::*,
};

#[derive(Component)]
pub struct Player;


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(1.)),
        material: materials.add(ColorMaterial::from(Color::CRIMSON)),
        ..Default::default()
    })
    .insert(Player);
}

pub fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
        // this will be cannon up
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
        // this will be cannon down
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if keys.just_pressed(KeyCode::Space) {
        info!("Firing cannon!")
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

