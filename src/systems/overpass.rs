use bevy::{
    prelude::*, window::PrimaryWindow,
};
use bevy_prototype_lyon::entity::Path;

use crate::map::{MapBundle, MapFeature};

use super::{SettingsOverlay};

pub fn spawn_starting_point(
    mut map_bundle: ResMut<MapBundle>,
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
    map_bundle.get_more_data = true;
}