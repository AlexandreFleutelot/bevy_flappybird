use bevy::prelude::*;

use crate::{components::{Bird, AffectedByGravity, Velocity, Movable, PlayerControl}, BIRD_SPRITE, BIRD_SCALE, PLAYER_IMPULSE};

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(bird_spawn_system)
        .add_system(player_impulse_system);
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

