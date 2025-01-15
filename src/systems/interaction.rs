use bevy::{input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use crate::map::{MapBundle, MapFeature};

use super::bbox_system;

pub fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    if keys.pressed(KeyCode::KeyU) {
        // U is being held down
        bbox_system(commands, map_bundle, &camera_query, &primary_window_query, query, shapes_query);
    }
}

pub fn handle_mouse(
    commands: Commands,
    map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&mut OrthographicProjection, With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
    } else if buttons.just_released(MouseButton::Middle) {
        bbox_system(commands, map_bundle, &camera_query, &primary_window_query, query, shapes_query);
    }
}

pub fn check_map_info(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    shapes: Query<(&Path, &GlobalTransform, &MapFeature)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
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
                    println!("Clicked on shape at position: {:?}", feat.properties);
                    // You can add additional logic here to handle the clicks
                    break;
                }
            }
        }
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