mod astroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

use astroids::AstroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins((
            CameraPlugin,
            DefaultPlugins,
            SpaceshipPlugin,
            MovementPlugin,
            DebugPlugin,
            AstroidPlugin
        ))
        .run();
}
