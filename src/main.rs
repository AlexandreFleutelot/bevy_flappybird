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

const BIRD_SPRITE: &str = "sprites/bird.png";
const BIRD_ANIMATION_SPEED: f32 = 10.0;
const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);
const PLAYER_IMPULSE: f32 = 50.;

const PIPE_SPRITE: &str = "sprites/pipe-green.png";
const PIPE_SCALE: f32 = 1.0;
const PIPE_SPRITE_SIZE : (f32, f32) = (52., 320.);
const PIPE_OPENING_SIZE: f32 = 40.;
const PIPE_SPEED: f32 = -100.;

const START_MENU_SPRITE: &str = "sprites/message.png";
const GAMEOVER_SPRITE: &str = "sprites/gameover.png";

const BACKGROUND_SPRITE: &str = "sprites/background-day.png";

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

#[derive(Resource)]
pub struct AudioHandles {
    pub wing: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub point: Handle<AudioSource>,
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
    mut commands: Commands,
    asset_server: Res<AssetServer>)
{
    // camera
    commands.spawn(Camera2dBundle::default());

    //audio
    commands.insert_resource(AudioHandles {
        wing: asset_server.load("audio/flap.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        point: asset_server.load("audio/point.ogg"),
    });

}