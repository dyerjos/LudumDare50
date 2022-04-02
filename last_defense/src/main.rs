use bevy::{
    core::FixedTimestep,
    // diagnostic::FrameTimeDiagnosticsPlugin,
    // diagnostic::LogDiagnosticsPlugin,
    // diagnostic::EntityCountDiagnosticsPlugin,
    prelude::*,
};
use bevy_inspector_egui::InspectorPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

mod players;
mod walls;
mod cannons;

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Last Defender".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "wgpu=warn,bevy_ecs=info".to_string(),
        }) // * This overrides log levels for local development. comment out for production
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_startup_system(players::spawn_player)
        .add_startup_system(walls::spawn_walls)
        .add_startup_system(cannons::spawn_cannon)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(players::move_player)
                .with_system(cannons::move_cannon)
        )
        .add_plugin(InspectorPlugin::<players::PlayerData>::new())
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 2.5;
    commands.spawn_bundle(camera_bundle);
}
