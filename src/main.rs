
use bevy::prelude::*;

use bird::BirdPlugin;
use physics::PhysicsPlugin;
use pipes::PipesPulgin;
use background::BackgroundPlugin;

mod components;
mod background;
mod physics;
mod bird;
mod pipes;



const WINDOW_WIDTH: f32 = 300.0;
const WINDOW_HEIGHT: f32 = 400.0;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Flappy bird!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_startup_system(setup_system)
    .add_plugin(BirdPlugin)
    .add_plugin(PhysicsPlugin)
    .add_plugin(PipesPulgin)
    .add_plugin(BackgroundPlugin)
    .run();

}

fn setup_system(
    mut commands: Commands)
{
    // camera
    commands.spawn(Camera2dBundle::default());

}