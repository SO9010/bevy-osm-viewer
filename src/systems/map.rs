use std::collections::HashSet;

use bevy::{prelude::*, text::cosmic_text::ttf_parser::feat, utils::HashMap, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use crate::{map::{world_space_rect_to_lat_long, MapBundle, MapFeature, SCALE, STARTING_LONG_LAT}, webapi::get_overpass_data};

use super::{camera_space_to_world_space, SettingsOverlay};
pub fn spawn_map(mut commands: Commands) {
    let map_bundle: MapBundle = MapBundle::new(STARTING_LONG_LAT.x, STARTING_LONG_LAT.y, SCALE);
    commands.spawn(map_bundle);
}

pub fn respawn_map(
    mut commands: Commands,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    mut map_bundle: Query<&mut MapBundle>,
    overpass_settings: ResMut<SettingsOverlay>,
) {
    for (entity, _, _, _) in shapes_query.iter() {
        commands.entity(entity).despawn_recursive(); // Use despawn_recursive instead of despawn
    }

    if let Ok(map_bundle) = map_bundle.get_single_mut() {
        let disabled_setting: HashSet<String> = overpass_settings.get_disabled_categories().into_iter().collect();
        let enabled_setting = overpass_settings.get_true_keys_with_category();

        // Group features by category and key, the string is thing to look for
        let mut feature_groups: HashMap<String, Vec<&MapFeature>> = HashMap::new();

        for feature in &map_bundle.features {
            for (cat, key) in &enabled_setting {
                if !disabled_setting.contains(&cat.to_lowercase()) {
                    let key = if key == "*" { cat.clone() } else { format!("\"{}\":\"{}\"", cat.to_lowercase(), key.to_lowercase()) };
                    feature_groups.entry(key).or_default().push(feature);
                }
            }
        }

        for (key, features) in feature_groups {
            for feature in features {
                // Check if the feature's category is disabled
                if feature.properties.get("building").is_some() && disabled_setting.contains(&"building".to_string()) {
                    continue;
                }

                for line in &feature.road {
                    let points: Vec<_> = line
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

                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 5.0),
                            ..default()
                        },
                        Stroke::new(Srgba { red: 0.400, green: 0.400, blue: 0.400, alpha: 1.0 }, 2.5),
                        MapFeature {
                            id: feature.id.clone(),
                            properties: feature.properties.clone(),
                            geometry: feature.geometry.clone(),
                            road: feature.road.clone(),
                        }
                    )).with_children(|parent| {
                        parent.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                transform: Transform::from_xyz(2.5, -2.5, 1.0),
                                ..default()
                            },
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 2.5),
                        ));
                    });
                }

                for polygon in &feature.geometry {
                    let points: Vec<_> = polygon
                        .iter()
                        .map(|point| {
                            let projected = map_bundle.lat_lon_to_mercator(point.y, point.x);
                            Vec2::new(projected.x, projected.y)
                        })
                        .collect();

                    let shape = shapes::Polygon {
                        points: points.clone(),
                        closed: true,
                    };

                    let fill_color = if key.contains("Building") {
                        Srgba { red: 0.341, green: 0.341, blue: 0.341, alpha: 1.0 }
                    } else {
                        continue;
                    };

                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default()
                        },
                        Fill::color(fill_color),
                        Stroke::new(Srgba { red: 0.400, green: 0.400, blue: 0.400, alpha: 1.0 }, 1.0),
                        MapFeature {
                            id: feature.id.clone(),
                            properties: feature.properties.clone(),
                            geometry: feature.geometry.clone(),
                            road: feature.road.clone(),
                        }
                    )).with_children(|parent| {
                        parent.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                transform: Transform::from_xyz(2.5, -2.5, 1.0),
                                ..default()
                            },
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }),
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0),
                        ));
                    });
                }
            }
        }
    }
}

pub fn bbox_system(
    commands: Commands,
    mut map_bundle: Query<&mut MapBundle>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: &Query<&Window, With<PrimaryWindow>>,
    query: Query<&mut OrthographicProjection, With<Camera>>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    overpass_settings: ResMut<SettingsOverlay>,
) {
    if let Some(viewport) = camera_space_to_world_space(camera_query, primary_window_query, query) {
        if let Ok(mut bundle) = map_bundle.get_single_mut() {
            // Here we need to go through the bounding boxes and check if we have already gotten this bounding box 
            if !bundle.map_points.spatial_index.is_covered(&viewport) {
                //let split_viewports = bundle.map_points.spatial_index.split(&viewport.clone());
                //if split_viewports.is_empty() {
                    bundle.map_points.spatial_index.insert(viewport.clone());
                    let converted_bounding_box = world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y);
                    get_overpass_data(vec![converted_bounding_box], commands, map_bundle, shapes_query, overpass_settings);    
                //} else {
                //    bundle.map_points.spatial_index.insert_vec(split_viewports.clone());
                //    let converted_vec = split_viewports.iter()
                //        .map(|viewport| world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y))
                //        .collect::<Vec<_>>();
                //    get_overpass_data(converted_vec, commands, map_bundle, shapes_query);
                //}
            }
        } else {
            error!("Failed to get mutable reference to map_bundle");
        }
    } else {
        error!("Failed to convert camera space to world space");
    }
}
