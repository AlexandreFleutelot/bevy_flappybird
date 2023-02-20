use bevy::prelude::*;


use crate::components::{Velocity, Movable, AffectedByGravity};
use crate::{MOVES_SPEED};

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
        .add_system(apply_gravity);
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




