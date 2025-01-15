use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use crate::{map::{world_space_rect_to_lat_long, MapBundle, MapFeature, WorldSpaceRect, SCALE, STARTING_LONG_LAT}, webapi::{send_overpass_queries, send_overpass_query}};

use super::camera_space_to_world_space;
// TODO: look at this: https://www.reddit.com/r/bevy/comments/1dfvmba/how_can_i_link_an_entity_to_a_specific/
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
                    // Hide for now, it looks quite messy, we should have a menu on the side to select what to show.
                    /*
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
                    */
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
) {
    // Ok so we want to get the camera space taken up on the screen, this will then be used to get the bounding box
    // However, we dont want to grab this multiple times, so we will need to check multiple things,
    // 2. If we have already gotten this bounding box, or if we have a section of the screen that is already covered 
    // 3. We want to get a bounding box which is a bit bigger than the screen so that we get smooth movement.
    // 4. We need to make a max size that we can queery for or we need to consider optimisation, such as dififerent weights for the roads depending on the size of the screen.
    // 5. We want to chunk it because when it gets too big it will take a long time to load and it will actually crash.
    if let Some(viewport) = camera_space_to_world_space(camera_query, primary_window_query, query) {
        // Converted to long and lat
        if let Ok(mut bundle) = map_bundle.get_single_mut() {
            // Here we need to go through the bounding boxes and check if we have already gotten this bounding box 
            if !bundle.map_points.bounding_boxes.contains(&viewport) {
                /*
                let matching_bboxes = bundle.map_points.bounding_boxes
                .iter()
                .filter(|bbox| bbox.intersects(&viewport))
                .collect::<Vec<&WorldSpaceRect>>();
                if !matching_bboxes.is_empty() {
                    info!("Bounding box intersects with another bounding box");
                    info!("Got {} intersections", matching_bboxes.len());
                    // Filter through multiple times to ensure no duplicates
                    let split_viewports = matching_bboxes.iter().flat_map(|bbox| viewport.split(bbox).unwrap()).collect::<Vec<WorldSpaceRect>>();
                    
                    bundle.map_points.bounding_boxes.extend(split_viewports.clone());
                    let converted_vec = split_viewports.iter().map(|viewport| world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y)).collect();
                    send_overpass_queries(converted_vec, commands, map_bundle, shapes_query);
                    return;
                }
                */
                // No intersections so just request the whole viewport
                bundle.map_points.bounding_boxes.push(viewport.clone());
                let converted_bounding_box = world_space_rect_to_lat_long(viewport.clone(), SCALE, STARTING_LONG_LAT.x, STARTING_LONG_LAT.y);
                send_overpass_query(converted_bounding_box, commands, map_bundle, shapes_query);
            }
        }
    }
}
