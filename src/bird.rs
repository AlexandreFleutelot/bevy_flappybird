use bevy::prelude::*;

use bevy::sprite::collide_aabb::collide;
use crate::components::{Bird, AffectedByGravity, Velocity, Movable, PlayerControl, ScoreBox, Ground, Pipe};
use crate::{WINDOW_HEIGHT, BIRD_SPRITE_SIZE, GROUND_SPRITE_SIZE,  PIPE_SPRITE_SIZE, PIPE_SCALE};
use crate::{BIRD_SPRITE, BIRD_SCALE, PLAYER_IMPULSE, ScoreEvent, GameOverEvent};

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(bird_spawn_system)
        .add_system(player_impulse_system)
        .add_system(bird_scoring_system)
        .add_system(bird_bounds_collision)
        .add_system(bird_pipe_collision);
    }
}

fn bird_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>) 
{
    commands
        .spawn(SpriteBundle { 
            texture: asset_server.load(BIRD_SPRITE), 
            transform: Transform { 
                translation: Vec3::new(0.,0., 0.),
                rotation:Quat::IDENTITY, 
                scale: Vec3::new(BIRD_SCALE, BIRD_SCALE , 1.),
            },
            ..Default::default() 
        })
        .insert(Bird)
        .insert(PlayerControl)
        .insert(Movable)
        .insert(AffectedByGravity)
        .insert(Velocity {x:0., y:0.});
}


fn player_impulse_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerControl>>
) 
{
    if let Ok(mut velo) = query.get_single_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velo.y += PLAYER_IMPULSE;
        }
    }
}

fn bird_scoring_system(
    mut commands: Commands,
    score_query: Query<(Entity, &Transform), With<ScoreBox>>,
    mut ev_scored: EventWriter<ScoreEvent>) {
    for (score_box, transform) in score_query.iter()
    {
        let tf_x = transform.translation.x;
        if tf_x < 0. {
            commands.entity(score_box).despawn();
            ev_scored.send(ScoreEvent(1));
        }
    }
}

fn bird_bounds_collision(
    mut bird_query: Query<(&mut Transform, &mut Velocity, With<Bird>), Without<Ground>>,
    ground_query: Query<(&Transform, With<Ground>), Without<Bird>>,
    mut ev_gameover: EventWriter<GameOverEvent>)
{
    if let Ok((mut bird_tf, mut velo, _)) = bird_query.get_single_mut() {
        let bird_bottom = bird_tf.translation.y - BIRD_SPRITE_SIZE.1 / 2.;
        for (ground_tf, _) in ground_query.iter() {
            let ground_level = ground_tf.translation.y + GROUND_SPRITE_SIZE.1 / 2.;
            if bird_bottom < ground_level {
                bird_tf.translation.y = ground_level + BIRD_SPRITE_SIZE.1/2.;
                velo.y = -velo.y / 2.; //slow bouncing on top of screen
                ev_gameover.send(GameOverEvent);
            }
        }

        let bird_top = bird_tf.translation.y + BIRD_SPRITE_SIZE.1 / 2.;
         if bird_top > WINDOW_HEIGHT / 2. {
             bird_tf.translation.y = WINDOW_HEIGHT/2. - BIRD_SPRITE_SIZE.1/2.; //dont go under the ground
        }
    }
}

fn bird_pipe_collision(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    mut ev_gameover: EventWriter<GameOverEvent>)
{
    if let Ok(bird_tf) = bird_query.get_single() {
        for pipe_tf in pipe_query.iter(){
            let collision =  collide(
                bird_tf.translation,
                Vec2::new(BIRD_SPRITE_SIZE.0 * BIRD_SCALE, BIRD_SPRITE_SIZE.1 * BIRD_SCALE),
                pipe_tf.translation,
                Vec2::new(PIPE_SPRITE_SIZE.0 * PIPE_SCALE, PIPE_SPRITE_SIZE.1 * PIPE_SCALE)); 
            
            if let Some(_) = collision {
                ev_gameover.send(GameOverEvent);
            }  
        }
    }
}