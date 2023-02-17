use bevy::prelude::*;
use bird::BirdPlugin;
use physics::PhysicsPlugin;


mod components;
mod physics;
mod bird;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Flappy bird!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_startup_system(setup_system)
    .add_plugin(BirdPlugin)
    .add_plugin(PhysicsPlugin)
    .run();

}

fn setup_system(
    mut commands: Commands)
{
    // camera
    commands.spawn(Camera2dBundle::default());

}