use std::io::{BufRead, BufReader};

use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path;

use crate::{map::{get_data_from_string_osm, MapBundle, MapFeature, WorldSpaceRect}, systems::respawn_map};
pub fn get(commands: Commands, map_bundle: Query<&mut MapBundle>,
    // need to convert from OSM to geojson!!!
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,) {
}

// TODO: PLEASE OH PLEASE MAKE THIS MULTITHREADED WITH ASYNC!
pub fn send_overpass_queries(bounds: Vec<WorldSpaceRect>, commands: Commands, mut map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
) {
    let url = "https://overpass-api.de/api/interpreter";
    info!("Querying Overpass API...");

    for bounds in bounds {
        let query = format!(r#"
            [out:json];
            (
            way["highway"]({},{},{},{}); 
            way["building"]({},{},{},{}); 
            );
            (._;>;);
            out body geom;
        "#, bounds.bottom, bounds.right, bounds.top, bounds.left, bounds.bottom, bounds.right, bounds.top, bounds.left);

        
    if let Ok(response) = ureq::post(url).send_string(&query) {
        if response.status() == 200 {
            let mut response_body = String::new();
            let reader = BufReader::new(response.into_reader());

            // Accumulate chunks into a single string
            for line in reader.lines() {
                match line {
                    Ok(part) => response_body.push_str(&part.as_str()),
                    Err(e) => {
                        info!("Error reading response: {}", e);
                        return;
                    }
                }
            }

            // Deserialize the accumulated string
            match get_data_from_string_osm(&response_body) {
                Ok(features) => {
                    if let Ok(mut map_bundle) = map_bundle.get_single_mut() {
                        let new_features: Vec<_> = features
                            .into_iter()
                            .filter(|feature| {
                                !map_bundle
                                    .features
                                    .iter()
                                    .any(|existing| existing.id.contains(&feature.id))
                            })
                            .collect();

                        info!("Got {} new features", new_features.len());
                        map_bundle.features.extend(new_features);
                    }
                }
                Err(e) => {
                    info!("Failed to parse response: {}", e);
                }
            }
        } else {
            info!("Failed to get a successful response from Overpass API.");
        }
    } else {
        info!("Failed to query Overpass API.");
    }
    }
    respawn_map(commands, shapes_query, map_bundle);
}


pub fn send_overpass_query(bounds: WorldSpaceRect, commands: Commands, mut map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
) {
    let url = "https://overpass-api.de/api/interpreter";
    info!("Querying Overpass API...");
    
    let query = format!(r#"
        [out:json];
        (
        way["highway"]({},{},{},{}); 
        // way["building"]({},{},{},{}); 
        );
        (._;>;);
        out body geom;
    "#, bounds.bottom, bounds.right, bounds.top, bounds.left, bounds.bottom, bounds.right, bounds.top, bounds.left);
    
    info!("Query: {}", query);
    
    if let Ok(response) = ureq::post(url).send_string(&query) {
        if response.status() == 200 {
            let mut response_body = String::new();
            let reader = BufReader::new(response.into_reader());

            // Accumulate chunks into a single string
            for line in reader.lines() {
                match line {
                    Ok(part) => response_body.push_str(&part.as_str()),
                    Err(e) => {
                        info!("Error reading response: {}", e);
                        return;
                    }
                }
            }

            // Deserialize the accumulated string
            match get_data_from_string_osm(&response_body) {
                Ok(features) => {
                    if let Ok(mut map_bundle) = map_bundle.get_single_mut() {
                        let new_features: Vec<_> = features
                            .into_iter()
                            .filter(|feature| {
                                !map_bundle
                                    .features
                                    .iter()
                                    .any(|existing| existing.id.contains(&feature.id))
                            })
                            .collect();

                        info!("Got {} new features", new_features.len());
                        map_bundle.features.extend(new_features);
                    }
                }
                Err(e) => {
                    info!("Failed to parse response: {}", e);
                }
            }
        } else {
            info!("Failed to get a successful response from Overpass API.");
        }
    } else {
        info!("Failed to query Overpass API.");
    }

    respawn_map(commands, shapes_query, map_bundle);
}
