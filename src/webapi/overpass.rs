use std::{io::{BufRead, BufReader, Read}};

use bevy::prelude::*;
use bevy_tasks::TaskPool;
use bevy::tasks::AsyncComputeTaskPool;

use crate::{map::{get_data_from_string_osm, MapBundle, MapFeature, WorldSpaceRect}, systems::SettingsOverlay};

pub fn build_overpass_query(bounds: Vec<WorldSpaceRect>, overpass_settings: &mut SettingsOverlay) -> String {
    let mut query = String::default();
    let opening = "[out:json];(";
    let closing = ");(._;>;);\nout body geom;";

    for bound in bounds {
        for (category, key) in overpass_settings.get_true_keys_with_category() {
        /*
        If you only want the program to fetch the specific data you want, you can use this code instead of the one below.
            if key == "n/a" {
                continue;
            } else if key == "*" {
                query.push_str(&format!(r#"
                (
                way["{}"]({},{},{},{}); 
                );
                "#, category.to_lowercase(), bound.bottom, bound.right, bound.top, bound.left));
            } else {
                query.push_str(&format!(r#"
                (
                way["{}"="{}"]({},{},{},{}); 
                );
                "#, category.to_lowercase(), key.to_lowercase(), bound.bottom, bound.right, bound.top, bound.left));
            }
        */
            query.push_str(&format!(r#"
            (
            way["{}"]({},{},{},{}); 
            );
            "#, category.to_lowercase(), bound.bottom, bound.right, bound.top, bound.left));
        }
    }

    if !query.is_empty() {
        query.insert_str(0, opening);
        query.push_str(closing);
    } else {
        return "ERR".to_string();
    }
    query
}

pub fn get_overpass_data<'a>(bounds: Vec<WorldSpaceRect>, map_bundle: &mut MapBundle, overpass_settings: &mut SettingsOverlay,
) -> Vec<MapFeature>  {
    info!("Querying Overpass API...");
    if bounds.is_empty() {
        return vec![];
    }
    let query = build_overpass_query(bounds, overpass_settings);
    if query != "ERR" {
        return send_overpass_query(query, map_bundle)
    }
    vec![]
}

pub fn send_overpass_query(query: String, map_bundle: &mut MapBundle,
) -> Vec<MapFeature> {
    if query.is_empty() {
        return vec![];
    }
    let url = "https://overpass-api.de/api/interpreter";
    
    if let Ok(response) = ureq::post(url).send_string(&query) {
        if response.status() == 200 {
            let reader: BufReader<Box<dyn Read + Send + Sync>> = BufReader::new(response.into_reader());
        
            let mut response_body = String::default();
            // Accumulate chunks into a single string
            for line in reader.lines() {
                match line {
                    Ok(part) => response_body.push_str(part.as_str()),
                    Err(e) => {
                        info!("Error reading response: {}", e);
                        return vec![];
                    }
                }
            }

            let map_features = map_bundle.features.clone();
            let features = get_data_from_string_osm(&response_body);
            if features.is_ok() {
                let new_features: Vec<_> = features.unwrap()
                .into_iter()
                .filter(|feature| {
                    !map_features
                        .iter()
                        .any(|existing| existing.id.contains(&feature.id))
                })
                .collect();
                return new_features
            }
            return vec![]
        }
    } else {
        info!("Failed to send query to Overpass API");
    }
    vec![]
}

