use bevy::prelude::*;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, GROUND_SPRITE, GROUND_SPRITE_SIZE, GROUND_SCALE, BACKGROUND_SPRITE};
use crate::components::{Ground, Parallax};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(ground_setup)
        .add_system(parallax_system);
    }
}

fn ground_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>) 
{
    commands.spawn(SpriteBundle { 
        texture: asset_server.load(GROUND_SPRITE), 
        transform: Transform { 
            translation: Vec3::new(WINDOW_WIDTH+160.,-WINDOW_HEIGHT/2. + GROUND_SPRITE_SIZE.1 /3. ,1.), 
            rotation: Quat::IDENTITY, 
            scale: Vec3::new(GROUND_SCALE, GROUND_SCALE, 1.) 
        },         
        ..Default::default()
    })
    .insert(Ground)
    .insert(Parallax { velocity_x: 100., loop_x: 22. });

    commands.spawn(SpriteBundle { 
        texture: asset_server.load(BACKGROUND_SPRITE), 
        transform: Transform { 
            translation: Vec3::new(20.,0. ,0.), 
            rotation: Quat::IDENTITY, 
            scale: Vec3::new(1., 1., 1.) 
        },         
        ..Default::default()
    })
    .insert(Parallax { velocity_x: 100., loop_x: 286. });
}   

fn parallax_system(time: Res<Time>, mut query: Query<(&Parallax, &mut Transform)>) {
    query.iter_mut().for_each(|(parallax, mut transform)| {
        let offset_x = parallax.loop_x / 2.0;
        transform.translation.x = -((-(transform.translation.x - offset_x)
            + parallax.velocity_x * time.delta_seconds())
            % parallax.loop_x)
            + offset_x;
    });
}

