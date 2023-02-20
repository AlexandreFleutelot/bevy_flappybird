use bevy::prelude::*;

use bird::BirdPlugin;
use gui::GuiPlugin;
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

const START_MENU_SPRITE: &str = "sprites/message.png";
const GAMEOVER_SPRITE: &str = "sprites/gameover.png";

const MOVES_SPEED: f32 = 10.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct GameData {
    pub score: u32,
}

mod gui;
mod components;
mod background;
mod bird;
mod pipes;
mod physics;

pub struct ScoreEvent(pub u32);
pub struct GameOverEvent(Entity);

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .insert_resource(GameData { score: 0 })
    .add_event::<ScoreEvent>()
    .add_event::<GameOverEvent>()
    .add_startup_system(setup_system)
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
    .add_plugin(BirdPlugin)
    .add_plugin(PhysicsPlugin)
    .add_plugin(PipesPulgin)
    .add_plugin(BackgroundPlugin)
    .add_plugin(GuiPlugin)
    .add_state(GameState::Menu)
    .run();
}

fn setup_system(
    mut commands: Commands)
{
    // camera
    commands.spawn(Camera2dBundle::default());

}

