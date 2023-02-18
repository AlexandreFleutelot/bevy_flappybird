use std::thread::spawn;

use bevy::prelude::*;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

const GROUND_SPRITE: &str = "sprites/base.png";
const GROUND_SCALE: f32 = 1.0;
const GROUND_SPRITE_SIZE: (f32, f32) = (336., 112.);

const GROUND_SLIDE_SPEED: f32 = 100.;


pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(ground_setup);
    }
}

fn ground_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>) 
{
    let mut spawn_background = |offset| {
        commands.spawn(SpriteBundle { 
            texture: asset_server.load(GROUND_SPRITE), 
            transform: Transform { 
                translation: Vec3::new(offset,-WINDOW_HEIGHT/2. + GROUND_SPRITE_SIZE.1 /3. ,0.), 
                rotation: Quat::IDENTITY, 
                scale: Vec3::new(GROUND_SCALE, GROUND_SCALE, 1.) 
            },         
            ..Default::default()
        });
    };
    spawn_background(0.);
    spawn_background(WINDOW_WIDTH)
}   

fn ground_slide_system()
{
    todo!();
}