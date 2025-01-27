
use bevy::{color::palettes::css::BLACK, prelude::*, utils::HashMap, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use geo::{BoundingRect, Intersects};
use rstar::{RTree, AABB};
use tess::path::polygon;

use crate::{map::{get_map_data, world_space_rect_to_lat_long, MapBundle, MapFeature, WorldSpaceRect, SCALE, STARTING_LONG_LAT}, webapi::get_overpass_data};
use super::{camera_space_to_world_space, SettingsOverlay};

pub fn respawn_map(
    mut commands: Commands,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    overpass_settings: Res<SettingsOverlay>,
    mut map_bundle: ResMut<MapBundle>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    if map_bundle.respawn {
        info!("Respawning map...");
        map_bundle.respawn = false;

        for (entity, _, _, _) in shapes_query.iter() {
            commands.entity(entity).despawn_recursive(); // Use despawn_recursive instead of despawn
        }

        let mut batch_commands_closed: Vec<(ShapeBundle, Fill, Stroke, MapFeature)> = Vec::new();
        let mut batch_commands_open: Vec<(ShapeBundle, Stroke, MapFeature)> = Vec::new();


        // Determine the viewport bounds
        let (_, camera_transform) = camera_query.single();
        let window = primary_window_query.single();
        let viewport = camera_space_to_world_space(camera_transform, window, query.single().clone(), 2.0).unwrap();

        let viewport_rect = world_space_rect_to_lat_long(viewport, SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y);
        let left = viewport_rect.left.min(viewport_rect.right);
        let right = viewport_rect.left.max(viewport_rect.right);
        let bottom = viewport_rect.bottom.min(viewport_rect.top);
        let top = viewport_rect.bottom.max(viewport_rect.top);
        let viewport_aabb = AABB::from_corners(
            [bottom as f64, left as f64],
            [top as f64, right as f64],
        );
        let intersection_candidates = map_bundle.features.locate_in_envelope_intersecting(&viewport_aabb).collect::<Vec<_>>();

        let disabled_setting = overpass_settings.get_disabled_categories();
        let enabled_setting = overpass_settings.get_true_keys_with_category_with_individual();

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
        
        for feature in intersection_candidates {
            let mut fill_color= Some(Srgba { red: 0.4, green: 0.400, blue: 0.400, alpha: 1.0 });
            let mut stroke_color = Srgba { red: 0.50, green: 0.500, blue: 0.500, alpha: 1.0 };
            let mut line_width = 1.0;
            let mut elevation = 1.0;
            for ((cat, key), _) in &feature_groups {
                if key != "*" {
                    if feature.properties.get(cat.to_lowercase()).map_or(false, |v| *v == *key.to_lowercase()) {
                        let color = overpass_settings.categories.get(cat).unwrap().items.get(key).unwrap().1;
                        fill_color = Some(Srgba { red: (color.r() as f32) / 255., green: (color.g() as f32) / 255., blue: (color.b() as f32) / 255., alpha: 1.0 });
                        stroke_color = Srgba { red: (color.r() as f32) / 210., green: (color.g() as f32) / 210., blue: (color.b() as f32) / 210., alpha: 1.0 };
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

                        let mut points = feature.get_in_world_space();
                        points.pop();                            
                            
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
            }
        }

        commands.spawn_batch(batch_commands_closed);
        commands.spawn_batch(batch_commands_open);
    }
}

fn is_feature_in_viewport(feature: &MapFeature, viewport: &WorldSpaceRect) -> bool {
    let viewport_rect = geo::Rect::new(
        geo::Coord { x: viewport.left as f64, y: viewport.bottom as f64 },
        geo::Coord { x: viewport.right as f64, y: viewport.top as f64 },
    );
    feature.geometry.intersects(&viewport_rect)
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
        let (camera, camera_transform) = query.single();
        let window = primary_window_query.single();

        if let Some(viewport) = camera_space_to_world_space(camera_transform, window, ortho_projection_query.single().clone(), 1.25) {
            // Here we need to go through the bounding boxes and check if we have already gotten this bounding box 
            if !map_bundle.map_points.spatial_index.is_covered(&viewport) {
                let (tx, rx) = bounded::<Vec<MapFeature>>(10);
                let tx_clone = tx.clone();
                let mut map_bundle_clone = map_bundle.clone();
                let mut overpass_settings_clone = overpass_settings.clone();
                map_bundle.map_points.spatial_index.insert(viewport.clone());
                let converted_bounding_box = world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y);
                

                std::thread::spawn(move || {
                    //tx.send(get_map_data("green-belt.geojson").unwrap());

                    tx.send(get_overpass_data(vec![converted_bounding_box], &mut map_bundle_clone, &mut overpass_settings_clone));
                });

                let shape = shapes::RoundedPolygon {
                    points: vec![
                        Vec2::new(viewport.left, viewport.bottom),
                        Vec2::new(viewport.right, viewport.bottom),
                        Vec2::new(viewport.right, viewport.top),
                        Vec2::new(viewport.left, viewport.top),
                    ],
                    radius: 25.0,
                    closed: true,
                };
                commands.spawn((ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_xyz(0.0, 0.0, -0.1),
                    ..default()
                },
                    Fill::color(Srgba {red: 0.071, green: 0.071, blue: 0.071, alpha: 1.0 })
                ));
                commands.insert_resource(MapReceiver(rx));

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
        for feature in &v {
            map_bundle.features.insert(feature.clone());
        }
        map_bundle.respawn = true;
    }
}