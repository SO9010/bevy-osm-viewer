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
            // way["building"]({},{},{},{}); 
            );
            (._;>;);
            out body geom;
        "#, bounds.bottom, bounds.right, bounds.top, bounds.left, bounds.bottom, bounds.right, bounds.top, bounds.left);

        // TODO: Need to optimise this: https://users.rust-lang.org/t/optimizing-string-search-code-for-large-files/31992/2 as it crashes if the text file is too large
        if let Ok(response) = ureq::post(url)
            .send_string(&query).unwrap()
            .into_string() {
            if let Ok(mut mb) = map_bundle.get_single_mut() {
                if let Ok(features) = get_data_from_string_osm(response.as_str()) {
                    let new_features: Vec<_> = features.clone()
                        .into_iter()
                        .filter(|feature| !mb.features.iter().any(|existing| existing.id.contains(feature.id.as_str())))
                        .collect();
                    mb.features.extend(new_features);

                } else {
                    info!("Failed to get data from string");
                }
            }
        } else {
            info!("Failed to get response from Overpass API... Assume the response was too large");
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
        //way["building"]({},{},{},{}); 
        );
        (._;>;);
        out body geom;
    "#, bounds.bottom, bounds.right, bounds.top, bounds.left, bounds.bottom, bounds.right, bounds.top, bounds.left);
    
    info!("Query: {}", query);
    
    // TODO: Need to optimise this: https://users.rust-lang.org/t/optimizing-string-search-code-for-large-files/31992/2 as it crashes if the text file is too large
    if let Ok(response) = ureq::post(url)
        .send_string(&query).unwrap()
        .into_string() {
        if let Ok(mut map_bundle) = map_bundle.get_single_mut() {
            if let Ok(features) = get_data_from_string_osm(response.as_str()) {
                let new_features: Vec<_> = features.clone()
                    .into_iter()
                    .filter(|feature| !map_bundle.features.iter().any(|existing| existing.id.contains(feature.id.as_str())))
                    .collect();
                
                info!("got {}", new_features.len());
                map_bundle.features.extend(new_features);
            } else {
                info!("Failed to get data from string");
            }
        }
        } else {
            info!("Failed to get response from Overpass API... Assume the response was too large");
        }

    respawn_map(commands, shapes_query, map_bundle);
}

pub fn get_road_data(commands: Commands, mut map_bundle: Query<&mut MapBundle>,
    // need to convert from OSM to geojson!!!
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,) {
    let query = r#"
        [out:json];
        (
        way["highway"="primary"](52.0,0.145,52.195,0.154); 
        );
        (._;>;);
        out body geom;
    "#;
    
    let url = "https://overpass-api.de/api/interpreter";
    info!("Querying Overpass API...");
    let response = ureq::post(url)
        .send_string(query).unwrap()
        .into_string().unwrap();
    
    if let Ok(mut map_bundle) = map_bundle.get_single_mut() {
        if let Ok(features) = get_data_from_string_osm(response.as_str()) {
            info!("got {}", features.len());
            map_bundle.features.extend(features);
            
        } else {
            info!("Failed to get data from string");
        }
    }
    respawn_map(commands, shapes_query, map_bundle);
}