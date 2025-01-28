use bevy::{prelude::*, state::commands, utils::HashMap, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use bevy_pancam::PanCam;
use bevy_prototype_lyon::prelude::*;

use crate::map::{MapBundle, MapFeature};

use super::OccupiedScreenSpace;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_keyboard, handle_mouse, draw_selection_box))
            .add_systems(Startup, init_selection_box)
            .insert_resource(SelectionBox::default());
    }
}

#[derive(Resource)]
pub struct SelectionBox {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
}

impl Default for SelectionBox {
    fn default() -> Self {
        SelectionBox {
            start: None,
            end: None,
        }
    }
}

impl SelectionBox {
    pub fn as_rect(&self) -> Option<geo::Rect<f32>> {
        if self.start.is_some() && self.end.is_some() {
            Some(geo::Rect::new(geo::coord! {
                x: self.start.unwrap().x, y: self.start.unwrap().y
            }, geo::coord! {
                x: self.end.unwrap().x, y: self.end.unwrap().y
            }))
        } else {
            None
        }
    }
}

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
    mut selection_box: ResMut<SelectionBox>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
    mut q_pancam: Query<&mut PanCam>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut persistent_info_windows: ResMut<PersistentInfoWindows>,
) {
    let window = q_windows.single();
    let (camera, camera_transform) = camera.single();

    if buttons.just_pressed(MouseButton::Middle) {
    } else if buttons.just_released(MouseButton::Middle) {
        map_bundle.get_more_data = true;
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if cursor_pos.x.is_finite() && cursor_pos.y.is_finite() {
                if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    if world_position.x.is_finite() && world_position.y.is_finite() {
                        selection_box.start = Some(world_position);
                        selection_box.end = Some(world_position);
                    }
                }
            }
        }
    } else if buttons.just_released(MouseButton::Left) {
        selection_box.start = None;
        selection_box.end = None;
    } else if buttons.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if cursor_pos.x.is_finite() && cursor_pos.y.is_finite() {
                if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    if world_position.x.is_finite() && world_position.y.is_finite() {
                        selection_box.end = Some(world_position);
                    }
                }
            }
        }
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


#[derive(Component)]
pub struct SelectionBoxSelector;

pub fn init_selection_box(mut commands: Commands) {
    commands.spawn(SelectionBoxSelector);
}
pub fn draw_selection_box(
    mut commands: Commands,
    selection: Res<SelectionBox>,
    query: Query<Entity, With<SelectionBoxSelector>>,
) {

    if selection.is_changed() {

        
        if let Some(start) = selection.start {
            if let Some(end) = selection.end {
                if start == end {
                    return;
                }
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                let shape = shapes::RoundedPolygon {
                    points: vec![
                        Vec2::new(start.x, start.y),
                        Vec2::new(end.x, start.y),
                        Vec2::new(end.x, end.y),
                        Vec2::new(start.x, end.y),
                    ],
                    radius: 5.0,
                    closed: true,
                };
    
                commands.spawn((ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_xyz(0.0, 0.0, 1000.),
                    ..default()
                },
                Fill::color(Srgba { red: 0.0, green: 0.00, blue: 0.500, alpha: 0.25 }),
                Stroke::new(Srgba { red: 0.0, green: 0.00, blue: 0.500, alpha: 0.5 }, 2.5 as f32),
                SelectionBoxSelector
                ));
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
            ui.label(window_state.to_string());
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