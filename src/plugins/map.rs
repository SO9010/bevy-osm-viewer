use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use crate::systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map)
            .add_systems(Update, (check_map_info, handle_mouse, handle_keyboard));
        if cfg!(debug_assertions) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_systems(Startup, (debug_draw_fps, debug_draw_entity_no))
                .add_systems(Update, (text_update_fps, count_entities));
        }
    }
}
