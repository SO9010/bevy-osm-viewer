
use std::default;

use bevy::{prelude::*, state::commands, text::cosmic_text::ttf_parser::feat, utils::HashMap, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use crossbeam_channel::{bounded, Receiver};

use crate::{map::{self, world_space_rect_to_lat_long, MapBundle, MapFeature, SCALE, STARTING_LONG_LAT}, webapi::get_overpass_data};
use super::{camera_space_to_world_space, SettingsOverlay};
use bevy_tasks::{AsyncComputeTaskPool, TaskPool};

pub fn respawn_map(
    mut commands: Commands,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    overpass_settings: Res<SettingsOverlay>,
    mut map_bundle: ResMut<MapBundle>,
) {
    // We should spawn batch standard squares when zoomed out futher than a certain amount.
    // We can make the road bigger based off how many lanes its reported to have.
    // We should only spawn the entities we want to see!
    if map_bundle.respawn {
        info!("Respawning map...");
        map_bundle.respawn = false;
        let mut batch_commands_closed: Vec<(ShapeBundle, Fill, Stroke, MapFeature)> = Vec::new();
        let mut batch_commands_open: Vec<(ShapeBundle, Stroke, MapFeature)> = Vec::new();
    
        for (entity, _, _, _) in shapes_query.iter() {
            commands.entity(entity).despawn_recursive(); // Use despawn_recursive instead of despawn
        }
    
        let disabled_setting = overpass_settings.get_disabled_categories();
        let enabled_setting = overpass_settings.get_true_keys_with_category();
    
        // Group features by category and key, the string is thing to look for
        // (cat, key)
        let mut feature_groups: HashMap<(String, String), Vec<&MapFeature>> = HashMap::new();
    
        for feature in &map_bundle.features {
            for (cat, key) in &enabled_setting {
                if !disabled_setting.contains(cat) {
                    feature_groups.entry((cat.to_string(), key.to_string())).or_default().push(feature);
                }
            }
        }
        
        for feature in &map_bundle.features {
            let mut skip_poly = true;
            let mut fill_color= Some(Srgba { red: 0.4, green: 0.400, blue: 0.400, alpha: 1.0 });
            let mut stroke_color = Srgba { red: 0.400, green: 0.400, blue: 0.400, alpha: 1.0 };
            let mut line_width = 1.0;
            let width_multiplier = 3.5;
            let mut elevation = 1.0;
            for ((cat, key), _) in &feature_groups {
                if key != "*" {
                    if feature.properties.get(cat.to_lowercase()).map_or(false, |v| *v == *key.to_lowercase()) {
                        let color = overpass_settings.categories.get(cat).unwrap().items.get(key).unwrap().1;
                        fill_color = Some(Srgba { red: (color.r() as f32) / 255., green: (color.g() as f32) / 255., blue: (color.b() as f32) / 255., alpha: 1.0 });
                        stroke_color = Srgba { red: (color.r() as f32) / 255., green: (color.g() as f32) / 255., blue: (color.b() as f32) / 255., alpha: 1.0 };
                        if cat == "Highway" || cat == "Railway" {
                            fill_color = None;
                            line_width = 2.5;
                            elevation = 0.;
    
                            // When zoomed out we should make the primary roads bigger, and the motorways even bigger.
                            if feature.properties.get("highway").map_or(false, |v| v == "residential" || v == "primary" || v == "secondary" || v == "tertiary") {
                                line_width = 5.5;
                            }
                            
    
                            let _ = feature.properties.get("est_width").map_or((), |v| {
                                // line_width = v.as_str().unwrap().replace("\"", "").parse::<f64>().unwrap() as f64;
                            });
                        }
                        skip_poly = false;
                    }
                } else {
                    if feature.properties.get(cat.to_lowercase()).is_some() {
                        if cat == "Highway" || cat == "Railway" {
                            fill_color = None;
                            line_width = 2.5;
                            elevation = 0.;
    
                            if feature.properties.get("highway").map_or(false, |v| v == "residential" || v == "primary" || v == "secondary" || v == "tertiary") {
                                line_width = 5.5;
                            }
                            
    
                            let _ = feature.properties.get("est_width").map_or((), |v| {
                                // line_width = v.as_str().unwrap().replace("\"", "").parse::<f64>().unwrap() as f64;
                            });
    
                        }
                        skip_poly = false;
                    }
                }
            }
            for polygon in &feature.geometry {
                if skip_poly {
                    continue;
                }
                let points: Vec<_> = polygon
                    .iter()
                    .map(|point| {
                        let projected = map_bundle.lat_lon_to_mercator(point.y, point.x);
                        Vec2::new(projected.x, projected.y)
                    })
                    .collect();
    
                let shape = shapes::Polygon {
                    points: points.clone(),
                    closed: false,
                };
    
                if let Some(fill) = fill_color {
                    batch_commands_closed.push((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, elevation),
                            ..default()
                        },
                        Fill::color(fill),
                        Stroke::new(stroke_color, line_width as f32),
                        feature.clone(),
                    ));
                } else {
                    batch_commands_open.push((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, elevation),
                            ..default()
                        },
                        Stroke::new(stroke_color, line_width as f32),
                        feature.clone(),
                    ));
                }
            }
        }
        commands.spawn_batch(batch_commands_closed);
        commands.spawn_batch(batch_commands_open);
    }
}

#[derive(Resource, Deref)]
pub struct MapReceiver(Receiver<Vec<MapFeature>>);

pub fn bbox_system(
    mut commands: Commands,
    query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ortho_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut map_bundle: ResMut<MapBundle>,
    overpass_settings: ResMut<SettingsOverlay>,
) {
    if map_bundle.get_more_data {
        map_bundle.get_more_data = false;
        if let Some(viewport) = camera_space_to_world_space(query, primary_window_query, ortho_projection_query) {
            // Here we need to go through the bounding boxes and check if we have already gotten this bounding box 
            if !map_bundle.map_points.spatial_index.is_covered(&viewport) {
                //let split_viewports = bundle.map_points.spatial_index.split(&viewport.clone());
                //if split_viewports.is_empty() {
                map_bundle.map_points.spatial_index.insert(viewport.clone());
                let converted_bounding_box = world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y);
                let mut map_bundle_clone = map_bundle.clone();
                let mut overpass_settings_clone = overpass_settings.clone();
                let (tx, rx) = bounded::<Vec<MapFeature>>(10);
                std::thread::spawn(move || {
                    tx.send(get_overpass_data(vec![converted_bounding_box], &mut map_bundle_clone, &mut overpass_settings_clone));
                });
                commands.insert_resource(MapReceiver(rx));
                //} else {
                //    bundle.map_points.spatial_index.insert_vec(split_viewports.clone());
                //    let converted_vec = split_viewports.iter()
                //        .map(|viewport| world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y))
                //        .collect::<Vec<_>>();
                //    get_overpass_data(converted_vec, commands, map_bundle, shapes_query);
                //}
            } else {
                error!("Failed to convert camera space to world space");
            }
        }
    }
}

pub fn read_map_receiver(
    map_receiver: Res<MapReceiver>,
    mut map_bundle: ResMut<MapBundle>,
) {
    if let Ok(v) = map_receiver.0.try_recv() {
        map_bundle.features.extend(v);
        map_bundle.respawn = true;
    }
}