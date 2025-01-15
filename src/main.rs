use bevy::app::*;
use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;

use systems::setup_camera;

use plugins::MapPlugin;
mod map;
mod systems;
mod plugins;
mod webapi;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, PanCamPlugin))
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::from(Srgba { red: 0.071, green: 0.071, blue: 0.071, alpha: 1.0 })))
        .add_plugins(MapPlugin)
        .run();
}
