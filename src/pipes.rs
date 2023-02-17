
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use std::f32::consts::PI;

use crate::components::{Pipe, Movable, Velocity};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

const PIPE_SPRITE: &str = "pipe.png";
const PIPE_SCALE: f32 = 1.5;
const PIPE_SPRITE_SIZE : (f32, f32) = (32., 128.);

const PIPE_OPENING_SIZE: f32 = 40.;
const PIPE_SPEED: f32 = -100.;

pub struct PipesPulgin ;
impl Plugin for PipesPulgin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(pipe_spawn_system),
        );
    }
}

fn pipe_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>) 
{
    let mut spawn_tube = |offset:f32, rot: f32| {
        commands
            .spawn(SpriteBundle { 
                texture: asset_server.load(PIPE_SPRITE), 
                transform: Transform { 
                    translation: Vec3::new(WINDOW_WIDTH/2.,offset ,0.), 
                    rotation: Quat::from_rotation_z(rot), 
                    scale: Vec3::new(PIPE_SCALE, PIPE_SCALE, 1.) 
                },         
                ..Default::default()
            })
            .insert(Pipe)
            .insert(Movable)
            .insert(Velocity {x:PIPE_SPEED, y:0.});
    };

    let pipe_offset = PIPE_SPRITE_SIZE.1 * PIPE_SCALE / 2. + PIPE_OPENING_SIZE ;
    spawn_tube(pipe_offset, PI);
    spawn_tube(-pipe_offset, 0.);
}