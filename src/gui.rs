use bevy::prelude::*;

use crate::components::{ScoreText, Menu, Pipe, ScoreBox, Bird, GameOver};
use crate::{WINDOW_HEIGHT,START_MENU_SPRITE, GAMEOVER_SPRITE};
use crate::{GameOverEvent, ScoreEvent, GameData, GameState, AudioHandles};

pub struct GuiPlugin;
impl Plugin for GuiPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(score_setup)
            
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu))
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(close_menu))

            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(gameover_system)
                    .with_system(score_system))

            .add_system_set(
                SystemSet::on_enter(GameState::GameOver)
                    .with_system(setup_gameover))
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(close_gameover))
            .add_system(game_states_system);
    }
}

fn score_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/flappybird.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("0", text_style)
        .with_alignment( TextAlignment::TOP_CENTER),
        transform: Transform::from_xyz(0., WINDOW_HEIGHT/2.-10., 1.),
        ..Default::default()
    })
    .insert(ScoreText);
}

fn score_system(
    mut ev_scored: EventReader<ScoreEvent>,
    mut game_data: ResMut<GameData>,
    mut scoreboard_query: Query<&mut Text, With<ScoreText>>,
    audio_handles: Res<AudioHandles>, audio: Res<Audio>,
) {
    for ev in ev_scored.iter() {
        game_data.score += ev.0;
        audio.play(audio_handles.point.clone());
        if let Ok(mut text) = scoreboard_query.get_single_mut() {
            text.sections[0].value = game_data.score.to_string();
        }
    }
}


fn gameover_system(
    mut commands: Commands,
    mut ev_gameover: EventReader<GameOverEvent>,
    mut game_state: ResMut<State<GameState>>,
    pipe_query: Query<Entity, With<Pipe>>,
    scorebox_query: Query<Entity, With<ScoreBox>>,
    bird_query: Query<Entity, With<Bird>>,
    audio_handles: Res<AudioHandles>, audio: Res<Audio>,
) {
    for _ in ev_gameover.iter() {
        game_state.set(GameState::GameOver).unwrap();

        for ent in pipe_query.iter() {
            commands.entity(ent).despawn();
        }
        for ent in scorebox_query.iter() {
            commands.entity(ent).despawn();
        }
        for ent in bird_query.iter() {
            commands.entity(ent).despawn();
        }
        
        audio.play(audio_handles.hit.clone());
    }
}

fn game_states_system(
    kb: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    match game_state.current() {
        GameState::Menu => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Playing).unwrap();
            }
        },
        GameState::Playing => {},
        GameState::GameOver => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Menu).unwrap();
            }
        },
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scoreboard_query: Query<&mut Text, With<ScoreText>>,
) {
    commands.spawn(SpriteBundle { 
        texture: asset_server.load(START_MENU_SPRITE), 
        transform: Transform::from_xyz(0., 0., 10.),
        ..Default::default()
    })
    .insert(Menu);

    if let Ok(mut text) = scoreboard_query.get_single_mut() {
        text.sections[0].value = "0".to_string();
    }
}

fn close_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>
) {
    for ent in menu_query.iter() {
        commands.entity(ent).despawn();
    }
}

fn setup_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(SpriteBundle { 
        texture: asset_server.load(GAMEOVER_SPRITE), 
        transform: Transform::from_xyz(0., 0., 10.),
        ..Default::default()
    })
    .insert(GameOver);
}

fn close_gameover(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOver>>,
    mut game_data: ResMut<GameData>,
) {
    for ent in menu_query.iter() {
        commands.entity(ent).despawn();
    }
    game_data.score = 0;
}

