use bevy::prelude::*;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct ScoreBox;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct PlayerControl;

#[derive(Component)]
pub struct AffectedByGravity;

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}