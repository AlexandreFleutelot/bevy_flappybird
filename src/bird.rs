use bevy::prelude::*;

use crate::WINDOW_HEIGHT;
use crate::components::{Bird, AffectedByGravity, Velocity, Movable, PlayerControl, Ground};
use crate::background::GROUND_SPRITE_SIZE;

const BIRD_SPRITE: &str = "sprites/redbird-midflap.png";
const BIRD_SPRITE_SIZE: (f32, f32) = (34. ,24.);
const BIRD_SCALE: f32 = 1.0;

const PLAYER_IMPULSE: f32 = 50.;

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(bird_spawn_system)
        .add_system(player_impulse_system)
        .add_system(bird_bounds_collision);
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

fn bird_bounds_collision(
    mut bird_query: Query<(&mut Transform, &mut Velocity, With<Bird>), Without<Ground>>,
    ground_query: Query<(&Transform, With<Ground>), Without<Bird>>)
{
    if let Ok((mut bird_tf, mut velo, _)) = bird_query.get_single_mut() {
        let bird_bottom = bird_tf.translation.y - BIRD_SPRITE_SIZE.1 / 2.;
        for (ground_tf, _) in ground_query.iter() 
        {
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