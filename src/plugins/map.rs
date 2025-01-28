use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use crate::{map::{MapBundle, SCALE, STARTING_LONG_LAT}, systems::*, systems::InteractionPlugin};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapBundle::new(STARTING_LONG_LAT.x, STARTING_LONG_LAT.y, SCALE))
            .add_systems(Startup, spawn_starting_point)
            .add_systems(Update, check_map_info)
            .add_plugins(InteractionPlugin)
            .add_systems(Update, camera_change)
            .add_systems(Update, (bbox_system, respawn_map, respawn_selection))
            .add_systems(FixedUpdate, read_map_receiver)
            .insert_resource(PersistentInfoWindows::default())
            .insert_resource(MapBundle::new(STARTING_LONG_LAT.x, STARTING_LONG_LAT.y, SCALE))
            .add_plugins(SettingsPlugin);
        if cfg!(debug_assertions) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_systems(Startup, (debug_draw_fps, debug_draw_entity_no))
                .add_systems(Update, (text_update_fps, count_entities));
        }
    }
}
