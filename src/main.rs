use bevy::app::*;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
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
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "OSM Viewer".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }), ShapePlugin, PanCamPlugin))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::from(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: 1.0 })))
        .add_plugins(MapPlugin)
        .run();
}

