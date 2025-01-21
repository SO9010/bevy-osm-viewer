use bevy::{
    prelude::*, window::PrimaryWindow,
};
use bevy_prototype_lyon::entity::Path;

use crate::map::{MapBundle, MapFeature};

use super::{bbox_system, SettingsOverlay};

pub fn spawn_starting_point(commands: Commands,
    map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&mut OrthographicProjection, With<Camera>>,
    mut overpass_settings: ResMut<SettingsOverlay>,
) 
{
    if let Some(category) = overpass_settings.categories.get_mut("Highway") {
        category.all = true;
        category.set_children(true);
    }
    if let Some(category) = overpass_settings.categories.get_mut("Building") {
        category.all = true;
        category.set_children(true);
    }
    bbox_system(commands, map_bundle, &camera_query, &primary_window_query, query, shapes_query, overpass_settings);
}