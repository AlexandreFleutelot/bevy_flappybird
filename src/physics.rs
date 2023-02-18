use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Velocity, Movable, AffectedByGravity, Ground, Bird, Pipe};
use crate::{WINDOW_HEIGHT, BIRD_SPRITE_SIZE, GROUND_SPRITE_SIZE, MOVES_SPEED, BIRD_SCALE, PIPE_SPRITE_SIZE, PIPE_SCALE};

#[derive(Resource)]
pub struct Gravity {
    x: f32,
    y: f32
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Gravity {x:0., y:-9.81})
        .add_system(move_system)
        .add_system(apply_gravity)
        .add_system(bird_bounds_collision)
        .add_system(bird_pipe_collision);
    }
}

fn move_system(
    mut query: Query<(&Velocity, &mut Transform), With<Movable>>,
    time: Res<Time>) 
{
    for (velocity, mut transform) in query.iter_mut() 
    {
        let translation = &mut transform.translation;
        let delta = time.delta().as_secs_f32();
        translation.x += velocity.x * delta;
        translation.y += velocity.y * delta;
    }
}

fn apply_gravity(
    gravity : Res<Gravity>,
    mut query: Query<&mut Velocity, With<AffectedByGravity>>,
    time: Res<Time>)
{
    for mut velo in query.iter_mut() 
    {
        velo.x += gravity.x * time.delta().as_secs_f32() * MOVES_SPEED;
        velo.y += gravity.y * time.delta().as_secs_f32() * MOVES_SPEED;
    }
}

fn bird_bounds_collision(
    mut bird_query: Query<(&mut Transform, &mut Velocity, With<Bird>), Without<Ground>>,
    ground_query: Query<(&Transform, With<Ground>), Without<Bird>>)
{
    if let Ok((mut bird_tf, mut velo, _)) = bird_query.get_single_mut() {
        let bird_bottom = bird_tf.translation.y - BIRD_SPRITE_SIZE.1 / 2.;
        for (ground_tf, _) in ground_query.iter() {
            let ground_level = ground_tf.translation.y + GROUND_SPRITE_SIZE.1 / 2.;
            if bird_bottom < ground_level {
                println!("Game over (ground)")
            }
        }

        let bird_top = bird_tf.translation.y + BIRD_SPRITE_SIZE.1 / 2.;
         if bird_top > WINDOW_HEIGHT / 2. {
            println!("too high");
             bird_tf.translation.y = WINDOW_HEIGHT/2. - BIRD_SPRITE_SIZE.1/2.;
             velo.y = -velo.y / 2.; //slow bouncing on top of screen
        }
    }
}

fn bird_pipe_collision(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>)
{
    if let Ok(bird_tf) = bird_query.get_single() {
        for pipe_tf in pipe_query.iter(){
            let collision =  collide(
                bird_tf.translation,
                Vec2::new(BIRD_SPRITE_SIZE.0 * BIRD_SCALE, BIRD_SPRITE_SIZE.1 * BIRD_SCALE),
                pipe_tf.translation,
                Vec2::new(PIPE_SPRITE_SIZE.0 * PIPE_SCALE, PIPE_SPRITE_SIZE.1 * PIPE_SCALE)); 
            
            if let Some(_) = collision {
                println!("hit pipe!");
            }  
        }
    }
}