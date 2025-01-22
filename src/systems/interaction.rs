use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use bevy_pancam::PanCam;
use bevy_prototype_lyon::prelude::*;

use crate::map::{MapBundle, MapFeature};

use super::{OccupiedScreenSpace, SettingsOverlay};

/// Handles keyboard input and updates map features accordingly.
pub fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut map_bundle: ResMut<MapBundle>,
) {
    if keys.pressed(KeyCode::KeyU) {
        // U is being held down
        map_bundle.get_more_data = true;
    }
}

/// Handles mouse input and updates camera and map features accordingly.
pub fn handle_mouse(
    mut map_bundle: ResMut<MapBundle>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
    mut q_pancam: Query<&mut PanCam>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
    } else if buttons.just_released(MouseButton::Middle) {
        map_bundle.get_more_data = true;
    }
    if let Some(position) = q_windows.single().cursor_position() {
        if position.x <= (occupied_screen_space.left + 15.) {
            for mut pancam in &mut q_pancam {
                pancam.enabled = false;
        }
        } else {
            for mut pancam in &mut q_pancam {
                pancam.enabled = true;
            }
        }
    }
}

#[derive(Resource)]
pub struct PersistentInfoWindows {
    pub windows: HashMap<String, String>,
}

impl Default for PersistentInfoWindows {
    fn default() -> Self {
        PersistentInfoWindows {
            windows: HashMap::new(),
        }
    }
}

/// Checks map information based on mouse input and camera view.
pub fn check_map_info(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    shapes: Query<(&Path, &GlobalTransform, &MapFeature)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    mut persistent_info_windows: ResMut<PersistentInfoWindows>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();
        let window = windows.single();

        if let Some(cursor_pos) = window.cursor_position() {
            let world_position = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
            for (path, _transform, feat) in shapes.iter() {
                let mut vertices: Vec<tess::geom::euclid::Point2D<f32, tess::geom::euclid::UnknownUnit>> = Vec::new();
                for path in path.0.iter() {
                    match path {
                        tess::path::Event::Line { to, from } => {
                            vertices.push(from);
                            vertices.push(to);
                        },
                        _ => continue,
                    };
                }
                if is_point_in_polygon(&world_position, vertices) {
                    persistent_info_windows.windows.insert(
                        feat.id.to_string(),
                        feat.properties.to_string(),
                    );
                
                    // You can add additional logic here to handle the clicks
                    break;
                }
            }
        }
    }
    let mut windows_to_remove = Vec::new();
    for (id, window_state) in persistent_info_windows.windows.iter() {
        egui::Window::new(id.clone())
        .show(contexts.ctx_mut(), |ui| {
            ui.label(&window_state.to_string());
            if ui.button("Close").clicked() {
                windows_to_remove.push(id.clone());
            }
        });
    }
    for id in windows_to_remove {
        persistent_info_windows.windows.remove(&id);
    }
}

pub fn is_point_in_polygon(point: &Vec2, vertices:  Vec<tess::geom::euclid::Point2D<f32, tess::geom::euclid::UnknownUnit>>) -> bool {
    if vertices.len() < 3 {
        return false;
    }
    
    let mut inside = false;
    let mut j = vertices.len() - 1;
    
    for i in 0..vertices.len() {
        if ((vertices[i].y > point.y) != (vertices[j].y > point.y)) &&
            (point.x < (vertices[j].x - vertices[i].x) * (point.y - vertices[i].y) 
                / (vertices[j].y - vertices[i].y) + vertices[i].x)
        {
            inside = !inside;
        }
        j = i;
    }
    
    inside
}