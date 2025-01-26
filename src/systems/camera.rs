use bevy::{core_pipeline::bloom::Bloom, prelude::*, window::PrimaryWindow};
use bevy_pancam::{DirectionKeys, PanCam};

use crate::map::{MapBundle, WorldSpaceRect};

use super::{orientation::CameraRotation, SettingsOverlay};



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
            max_scale: f32::INFINITY, // prevent the camera from zooming too far out
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
    mut map_bundle: ResMut<MapBundle>,
) {
    // TODO: Need to work on zoning what to spawn in and not to based of camera view.
    // TODO: get the data before it needs to show!
    let projection = query.single_mut();
    if projection.is_changed() {
        camera_settings.scale = projection.scale;
        if camera_settings.scale > 3.5 {
            if let Some(category) = overpass_settings.categories.get_mut("Building") {
                if !category.disabled {
                    category.disabled = true;
                    map_bundle.respawn = true;
                    map_bundle.get_more_data = true;
                }
            }
        } else {
            if let Some(category) = overpass_settings.categories.get_mut("Building") {
                if category.disabled {
                    category.disabled = false;
                    map_bundle.respawn = true;
                    map_bundle.get_more_data = true;
                } 
            }
        }
        /*
        if camera_settings.scale > 10.0 {
            if let Some(category) = overpass_settings.categories.get_mut("Highway") {
                category.set_children(false);
                if let Some((item, _)) = category.items.get_mut("motorway") {
                    *item = true;
                }
                if let Some((item, _)) = category.items.get_mut("bus_guideway") {
                    *item = true;
                }
                if let Some((item, _)) = category.items.get_mut("primary") {
                    *item = true;
                }
                if let Some((item, _)) = category.items.get_mut("secondary") {
                    *item = true;
                }
                map_bundle.get_more_data = true;
            }
        }
        */

    }
}

/// Overflow is the amount of world space that is loaded outside of the window, it is a multiplier of the window size
pub fn camera_space_to_world_space(
    transform: &GlobalTransform,
    window: &Window,
    projection: OrthographicProjection,
    overflow: f32,
) -> Option<WorldSpaceRect> {
    // Get the window size
    let window_width = window.width();
    let window_height = window.height();

    // Get the camera's position
    let camera_translation = transform.translation();

    // Compute the world-space rectangle
    // The reason for not dividing by 2 is to make the rectangle larger, as then it will mean that we can load more data
    let left = camera_translation.x - ((window_width * projection.scale) / 2.0) * overflow;
    let right = camera_translation.x + ((window_width * projection.scale) / 2.0) * overflow;
    let bottom = camera_translation.y - ((window_height * projection.scale) / 2.0)* overflow;
    let top = camera_translation.y + ((window_height * projection.scale) / 2.0) * overflow;
    
    
    return Some(WorldSpaceRect {
        left,
        right,
        bottom,
        top,
    });
}
