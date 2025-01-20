use bevy::{core_pipeline::bloom::Bloom, prelude::*, window::PrimaryWindow};
use bevy_pancam::{DirectionKeys, PanCam};
use bevy_prototype_lyon::entity::Path;

use crate::map::{MapBundle, MapFeature, WorldSpaceRect};

use super::{orientation::CameraRotation, respawn_map, SettingsOverlay};



#[derive(Resource)]
pub struct CameraSettings {
    pub scale: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        },
        PanCam {
            grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
            move_keys: DirectionKeys {      // the keyboard buttons used to move the camera
                up:    vec![KeyCode::ArrowUp], // initalize the struct like this or use the provided methods for
                down:  vec![KeyCode::ArrowDown], // common key combinations
                left:  vec![KeyCode::ArrowLeft],
                right: vec![KeyCode::ArrowRight],
            },
            speed: 400., // the speed for the keyboard movement
            enabled: true, // when false, controls are disabled. See toggle example.
            zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
            min_scale: 0.25, // prevent the camera from zooming too far in
            max_scale: 40., // prevent the camera from zooming too far out
            min_x: f32::NEG_INFINITY, // minimum x position of the camera window
            max_x: f32::INFINITY, // maximum x position of the camera window
            min_y: f32::NEG_INFINITY, // minimum y position of the camera window
            max_y: f32::INFINITY, // maximum y position of the camera window
        },
        Bloom::NATURAL,
        CameraRotation::default(),
    ));
}

pub fn camera_change(
    mut camera_settings: ResMut<CameraSettings>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut overpass_settings: ResMut<SettingsOverlay>,
    commands: Commands, mut map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
) {
    let projection = query.single_mut();
    if projection.is_changed() {
        camera_settings.scale = projection.scale;
        if camera_settings.scale > 3.5 {
            if let Some(category) = overpass_settings.categories.get_mut("Building") {
                category.disabled = true;
                respawn_map(commands, shapes_query, map_bundle, overpass_settings);                
            }
        } else {
            if let Some(category) = overpass_settings.categories.get_mut("Building") {
                if category.disabled {
                    category.disabled = false;
                    respawn_map(commands, shapes_query, map_bundle, overpass_settings);                
                } 
            }
        }

        /*
        
        if camera_settings.scale > 10.0 {
            if let Some(category) = overpass_settings.categories.get_mut("Highway") {
                category.disabled = false;
                if let Some(item) = category.items.get_mut("motorway") {
                    *item = false;
                }
                if let Some(item) = category.items.get_mut("bus_guideway") {
                    *item = false;
                }
                if let Some(item) = category.items.get_mut("primary") {
                    *item = false;
                }
                if let Some(item) = category.items.get_mut("secondary") {
                    *item = false;
                }
            }
        } 
        
        */
    }
}

pub fn camera_space_to_world_space(
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: &Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) -> Option<WorldSpaceRect> {
    if let Ok((_, transform)) = camera_query.get_single() {
        if let Ok(window) = primary_window_query.get_single() {
            let projection = query.single_mut();

            // Get the window size
            let window_width = window.width();
            let window_height = window.height();

            // Get the camera's position
            let camera_translation = transform.translation();

            // Compute the world-space rectangle
            // The reason for not dividing by 2 is to make the rectangle larger, as then it will mean that we can load more data
            let left = camera_translation.x - (window_width * projection.scale);
            let right = camera_translation.x + (window_width * projection.scale);
            let bottom = camera_translation.y - (window_height * projection.scale);
            let top = camera_translation.y + (window_height * projection.scale);
            
            
            return Some(WorldSpaceRect {
                left,
                right,
                bottom,
                top,
            });
        }
    }
    None
}
