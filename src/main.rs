use bevy::{
    prelude::{App, Camera2dBundle, Commands},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
