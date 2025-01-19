use std::{io::{BufRead, BufReader, Read}, thread};

use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path;
use crossbeam_channel::bounded;

use crate::{map::{get_data_from_string_osm, MapBundle, MapFeature, WorldSpaceRect}, systems::{respawn_map, SettingsOverlay}};

pub fn build_overpass_query(bounds: Vec<WorldSpaceRect>, overpass_settings: ResMut<SettingsOverlay>) -> String {
    let mut query = String::default();
    // Opening
    if let categories = overpass_settings.get_true_keys_with_category() {
        if !categories.is_empty() {
            query.push_str("[out:json];(");
            for bound in bounds {
                for (category, key) in overpass_settings.get_true_keys_with_category() {
                    if key == "*" {
                        query.push_str(&format!(r#"
                        (
                        way["{}"]({},{},{},{}); 
                        );
                        "#, category.to_lowercase(), bound.bottom, bound.right, bound.top, bound.left));
                    } else if key == "n/a" {
                    } else {
                        query.push_str(&format!(r#"
                        (
                        way["{}"="{}"]({},{},{},{}); 
                        );
                        "#, category.to_lowercase(), key.to_lowercase(), bound.bottom, bound.right, bound.top, bound.left));
                    }
                }
            }
            // Close
            query.push_str(");(._;>;);\nout body geom;");
        }
    }
    query
}

pub fn get_overpass_data(bounds: Vec<WorldSpaceRect>, commands: Commands, map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>, overpass_settings: ResMut<SettingsOverlay>,
) {
    info!("Querying Overpass API...");
    if bounds.is_empty() {
        return;
    }
    send_overpass_query(build_overpass_query(bounds, overpass_settings), commands, map_bundle, shapes_query);
}

// TODO: PLEASE OH PLEASE MAKE THIS MULTITHREADED WITH ASYNC!
pub fn send_overpass_query(query: String, commands: Commands, mut map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
) {
    if query.is_empty() {
        return;
    }
    let url = "https://overpass-api.de/api/interpreter";
    info!("query: {}", query);
    let (tx, rx) = bounded::<BufReader<Box<dyn Read + Send + Sync>>>(1);

    thread::spawn(move || {
        if let Ok(response) = ureq::post(url).send_string(&query) {
            if response.status() == 200 {
                let reader: BufReader<Box<dyn Read + Send + Sync>> = BufReader::new(response.into_reader());
                tx.send(reader).unwrap();
            }
        } else {
            info!("Failed to send query to Overpass API");
        }
    });

    if let Ok(reader) = rx.recv() {
        let mut response_body = String::default();
        // Accumulate chunks into a single string
        for line in reader.lines() {
            match line {
                Ok(part) => response_body.push_str(part.as_str()),
                Err(e) => {
                    info!("Error reading response: {}", e);
                    return;
                }
            }
        }

        // Deserialize the accumulated string
        let (tx, rx) = bounded::<Vec<MapFeature>>(1);
        let rpsn = response_body.clone();

        let map_features = if let Ok(map_bundle) = map_bundle.get_single() {
            map_bundle.features.clone()
        } else {
            Vec::new()
        };

        thread::spawn(move || {
            let features = get_data_from_string_osm(&rpsn);
            if features.is_ok() {
                let new_features: Vec<_> = features.unwrap()
                .into_iter()
                .filter(|feature| {
                    !map_features
                        .iter()
                        .any(|existing| existing.id.contains(&feature.id))
                })
                .collect();
                let _ = tx.send(new_features);
            }
        });

        if let Ok(features) = rx.recv() {
            if let Ok(mut map_bundle) = map_bundle.get_single_mut() {
                map_bundle.features.extend(features);
            }
        }

        respawn_map(commands, shapes_query, map_bundle);
    }
}

