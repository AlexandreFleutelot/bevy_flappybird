
use bevy::prelude::*;

use bird::BirdPlugin;
use physics::PhysicsPlugin;
use pipes::PipesPulgin;
use background::BackgroundPlugin;

const WINDOW_WIDTH: f32 = 300.0;
const WINDOW_HEIGHT: f32 = 400.0;

const GROUND_SPRITE: &str = "sprites/base.png";
const GROUND_SCALE: f32 = 1.0;
const GROUND_SPRITE_SIZE: (f32, f32) = (336., 112.);
const GROUND_SLIDE_SPEED: f32 = -100.;

const BIRD_SPRITE: &str = "sprites/redbird-midflap.png";
const BIRD_SPRITE_SIZE: (f32, f32) = (34. ,24.);
const BIRD_SCALE: f32 = 1.0;
const PLAYER_IMPULSE: f32 = 50.;

const PIPE_SPRITE: &str = "sprites/pipe-green.png";
const PIPE_SCALE: f32 = 1.0;
const PIPE_SPRITE_SIZE : (f32, f32) = (52., 320.);
const PIPE_OPENING_SIZE: f32 = 40.;
const PIPE_SPEED: f32 = -100.;

const MOVES_SPEED: f32 = 10.;

mod components;
mod background;
mod bird;
mod pipes;
mod physics;

pub struct ScoreEvent(pub u32);
pub struct GameOverEvent;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Flappy bird!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_event::<ScoreEvent>()
    .add_event::<GameOverEvent>()
    .add_startup_system(setup_system)
    .add_plugin(BirdPlugin)
    .add_plugin(PhysicsPlugin)
    .add_plugin(PipesPulgin)
    .add_plugin(BackgroundPlugin)
    .add_system(score_system)
    .add_system(gameover_system)
    .run();

}

fn setup_system(
    mut commands: Commands)
{
    // camera
    commands.spawn(Camera2dBundle::default());

}


fn score_system(
    mut ev_scored: EventReader<ScoreEvent>,
) {
    for ev in ev_scored.iter() {
        println!("Scored {:?}", ev.0);
    }
}

fn gameover_system(
    mut ev_gameover: EventReader<GameOverEvent>,
) {
    for _ in ev_gameover.iter() {
        println!("GameOver");
    }
}