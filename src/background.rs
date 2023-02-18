use bevy::prelude::*;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, GROUND_SPRITE, GROUND_SPRITE_SIZE, GROUND_SCALE, GROUND_SLIDE_SPEED};
use crate::components::{Movable, Velocity, Ground};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(ground_setup)
        .add_system(ground_slide_system);
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
                translation: Vec3::new(offset,-WINDOW_HEIGHT/2. + GROUND_SPRITE_SIZE.1 /3. ,1.), 
                rotation: Quat::IDENTITY, 
                scale: Vec3::new(GROUND_SCALE, GROUND_SCALE, 1.) 
            },         
            ..Default::default()
        })
        .insert(Ground)
        .insert(Movable)
        .insert(Velocity {x:GROUND_SLIDE_SPEED, y:0.});
    };
    spawn_background(0.);
    spawn_background(WINDOW_WIDTH)
}   

fn ground_slide_system(
    mut query: Query<&mut Transform, With<Ground>>)
{
    for mut tf in query.iter_mut() {
        if tf.translation.x < -WINDOW_WIDTH {
            tf.translation.x = WINDOW_WIDTH;
        }
    }
}


