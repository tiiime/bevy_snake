use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 720.,
            height: 720.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_world)
        .run()
}
 
fn setup_world(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
