use bevy::prelude::*;

use bird::BirdPlugin;
use components::ScoreText;
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
pub struct GameOverEvent(Entity);

#[derive(Resource)]
struct Scoreboard {
    score: u32,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .insert_resource(Scoreboard { score: 0 })
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
    .add_system(score_system)
    .add_system(gameover_system)
    .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>)
{
    // camera
    commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("fonts/flappybird.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("4", text_style)
        .with_alignment( TextAlignment::TOP_CENTER),
        transform: Transform::from_xyz(0., WINDOW_HEIGHT/2.-10., 1.),
        ..Default::default()
    })
    .insert(ScoreText);

}

fn score_system(
    mut ev_scored: EventReader<ScoreEvent>,
    mut score_board: ResMut<Scoreboard>,
    mut score_query: Query<&mut Text, With<ScoreText>>
) {
    for ev in ev_scored.iter() {
        score_board.score += ev.0;
    }

    for mut text in &mut score_query {
        text.sections[0].value = score_board.score.to_string();
    }
}

fn gameover_system(
    mut ev_gameover: EventReader<GameOverEvent>,
) {
    for ev in ev_gameover.iter() {
        println!("GameOver {:?}", ev.0);
    }
}