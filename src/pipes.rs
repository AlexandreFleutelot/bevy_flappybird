
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use std::f32::consts::PI;
use rand::Rng;

use crate::components::{Pipe, Movable, Velocity, ScoreBox};
use crate::{WINDOW_WIDTH, PIPE_SPRITE, PIPE_SCALE, PIPE_SPEED, PIPE_OPENING_SIZE, PIPE_SPRITE_SIZE};

pub struct PipesPulgin ;
impl Plugin for PipesPulgin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(pipe_spawn_system),
        )
        .add_system(pipe_despawn_system);
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

    let mut rng = rand::thread_rng();
    let rnd =  rng.gen_range(-10.0..60.0);
    let pipe_offset = PIPE_SPRITE_SIZE.1 * PIPE_SCALE / 2. + PIPE_OPENING_SIZE;

    spawn_tube(pipe_offset + rnd, PI);
    spawn_tube(-pipe_offset + rnd, 0.);

    //scoring box
    commands
        .spawn(Transform{ 
            translation: Vec3::new(WINDOW_WIDTH/2.,0.,0.),
            ..Default::default()})
        .insert(ScoreBox)
        .insert(Movable)
        .insert(Velocity {x:PIPE_SPEED, y:0.});
}

fn pipe_despawn_system(
    mut commands: Commands,
    pipe_query: Query<(Entity, &Transform), With<Pipe>>)
{
    for (pipe, transform) in pipe_query.iter()
    {
        let tf = transform.translation;
        if tf.x < -WINDOW_WIDTH/2. - 50. {
            commands.entity(pipe).despawn();
        }
    }
}