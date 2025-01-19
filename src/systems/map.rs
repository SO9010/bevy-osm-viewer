use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use crate::{map::{world_space_rect_to_lat_long, MapBundle, MapFeature, SCALE, STARTING_LONG_LAT}, webapi::get_overpass_data};

use super::{camera_space_to_world_space, SettingsOverlay};
pub fn spawn_map(mut commands: Commands) {
    let map_bundle: MapBundle = MapBundle::new(STARTING_LONG_LAT.x, STARTING_LONG_LAT.y, SCALE);
    commands.spawn(map_bundle);
}

pub fn respawn_map(mut commands: Commands, shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>, mut map_bundle: Query<&mut MapBundle>,) {
    for (entity, _, _, _) in shapes_query.iter() {
        commands.entity(entity).despawn_recursive(); // Use despawn_recursive instead of despawn
    }        
    // I think this spawns far too many entityies!
    if let Ok(map_bundle) = map_bundle.get_single_mut() {
        for feature in &map_bundle.features {
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
                        Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 2.5), // Shadow stroke alpha set to 1.0
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
                
                if feature.properties.get("landuse").is_some() {
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default()
                        },
                        Fill::color(Srgba { red: 0.341, green: 0., blue: 0.341, alpha: 1.0 }),
                        Stroke::new(Srgba { red: 0.400, green: 0., blue: 0.400, alpha: 1.0 }, 1.0),
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
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }), // Shadow alpha set to 1.0
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0), // Shadow stroke alpha set to 1.0
                        ));
                        
                    });
                } else if feature.properties.get("sport").is_some() {
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 1.750),
                            ..default()
                        },
                        Fill::color(Srgba { red: 0., green: 0.341, blue: 0., alpha: 1.0 }),
                        Stroke::new(Srgba { red: 0., green: 0.400, blue: 0.400, alpha: 1.0 }, 1.0),
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
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }), // Shadow alpha set to 1.0
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0), // Shadow stroke alpha set to 1.0
                        ));
                    });
                } else if feature.properties.get("leisure").is_some() {
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 1.50),
                            ..default()
                        },
                        Fill::color(Srgba { red: 0., green: 0.341, blue: 0., alpha: 1.0 }),
                        Stroke::new(Srgba { red: 0., green: 0.400, blue: 0.400, alpha: 1.0 }, 1.0),
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
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }), // Shadow alpha set to 1.0
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0), // Shadow stroke alpha set to 1.0
                        ));
                    });
                    //
                } else if feature.properties.get("amenity").is_some() {
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 10.0),
                            ..default()
                        },
                        Fill::color(Srgba { red: 0., green: 0.341, blue: 0.341, alpha: 1.0 }),
                        Stroke::new(Srgba { red: 0., green: 0.400, blue: 0.400, alpha: 1.0 }, 1.0),
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
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }), // Shadow alpha set to 1.0
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0), // Shadow stroke alpha set to 1.0
                        ));
                    });
                } else if feature.properties.get("building").is_some() {
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            transform: Transform::from_xyz(0.0, 0.0, 20.0),
                            ..default()
                        },
                        Fill::color(Srgba { red: 0.341, green: 0.341, blue: 0.341, alpha: 1.0 }),
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
                            Fill::color(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }), // Shadow alpha set to 1.0
                            Stroke::new(Srgba { red: 0., green: 0., blue: 0., alpha: 0.5 }, 1.0), // Shadow stroke alpha set to 1.0
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
