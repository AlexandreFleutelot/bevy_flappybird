use bevy::prelude::*;

use bevy::sprite::collide_aabb::collide;
use crate::components::{Bird, AffectedByGravity, Velocity, Movable, PlayerControl, ScoreBox, Ground, Pipe};
use crate::{WINDOW_HEIGHT, BIRD_SIZE, GROUND_SPRITE_SIZE,  PIPE_SPRITE_SIZE, PIPE_SCALE, PLAYER_IMPULSE, BIRD_ANIMATION_SPEED, BIRD_SPRITE, PIPE_SPEED};
use crate::{ScoreEvent, GameOverEvent, GameState, AudioHandles};

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
            .with_system(spawn_bird_system)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(player_impulse_system)
            .with_system(bird_scoring_system)
            .with_system(bird_bounds_collision)
            .with_system(bird_pipe_collision)
        )
        .add_system(animate_bird)
        .add_system(rotate_bird);
    }
}

fn spawn_bird_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) 
{
        let bird_texture = asset_server.load(BIRD_SPRITE);
        let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
            bird_texture,
            BIRD_SIZE,
            4,
            1,
            None,
            None,
        ));

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_xyz(0.0, 0.0, 1.),
                ..Default::default()
            },
        ))
        .insert(Bird)
        .insert(PlayerControl)
        .insert(Movable)
        .insert(AffectedByGravity)
        .insert(Velocity {x:0., y:0.});

}


fn player_impulse_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerControl>>,
    audio_handles: Res<AudioHandles>, audio: Res<Audio>
) {
    if let Ok(mut velo) = query.get_single_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velo.y += PLAYER_IMPULSE;
            audio.play(audio_handles.wing.clone());
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
    mut bird_query: Query<(&mut Transform, &mut Velocity, Entity, With<Bird>), Without<Ground>>,
    ground_query: Query<(&Transform, With<Ground>), Without<Bird>>,
    mut ev_gameover: EventWriter<GameOverEvent>)
{
    if let Ok((mut bird_tf, mut velo, piaf, _)) = bird_query.get_single_mut() {
        let bird_bottom = bird_tf.translation.y - BIRD_SIZE[1] / 2.;
        for (ground_tf, _) in ground_query.iter() {
            let ground_level = ground_tf.translation.y + GROUND_SPRITE_SIZE.1 / 2.;
            if bird_bottom < ground_level {
                bird_tf.translation.y = ground_level + BIRD_SIZE[1]/2.;
                velo.y = -velo.y / 2.; //slow bouncing on top of screen
                ev_gameover.send(GameOverEvent(piaf));
            }
        }

        let bird_top = bird_tf.translation.y + BIRD_SIZE[1] / 2.;
         if bird_top > WINDOW_HEIGHT / 2. {
             bird_tf.translation.y = WINDOW_HEIGHT/2. - BIRD_SIZE[1]/2.; //dont go under the ground
        }
    }
}

fn bird_pipe_collision(
    bird_query: Query<(&Transform, Entity), With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    mut ev_gameover: EventWriter<GameOverEvent>)
{
    if let Ok((bird_tf, piaf)) = bird_query.get_single() {
        for pipe_tf in pipe_query.iter(){
            let collision =  collide(
                bird_tf.translation,
                BIRD_SIZE,
                pipe_tf.translation,
                Vec2::new(PIPE_SPRITE_SIZE.0 * PIPE_SCALE, PIPE_SPRITE_SIZE.1 * PIPE_SCALE)); 
            
            if let Some(_) = collision {
                ev_gameover.send(GameOverEvent(piaf));
            }  
        }
    }
}

fn animate_bird(mut bird: Query<&mut TextureAtlasSprite, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        bird.index = (time.elapsed_seconds() * BIRD_ANIMATION_SPEED) as usize % 4;
    }
}

fn rotate_bird(mut bird_query: Query<(&mut Transform, &Velocity), With<Bird>>){
    for (mut bird_tf, bird_vel) in bird_query.iter_mut() {
        let angle = (bird_vel.y.clone() / -PIPE_SPEED).atan();
        bird_tf.rotation = Quat::from_rotation_z(angle);
    }
}
