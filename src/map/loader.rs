use std::{fs::File, io::BufReader};

use bevy::prelude::*;
use geojson::GeoJson;
use serde::{Deserialize, Serialize};

use super::MapFeature;

/// Parses OSM data from a string and returns a vector of map features.
pub fn get_data_from_string_osm(data: &str) -> Result<Vec<MapFeature>, Box<dyn std::error::Error>> {
    let response: OverpassResponse = serde_json::from_str(data)?;

    let mut features = Vec::new();

    for way in response.elements {
        // Ensure geometry exists
        let geometry = way.geometry;
        if !geometry.is_empty() {
            let points: Vec<Vec2> = geometry
                .iter()
                .map(|coords| Vec2::new(coords.lon as f32, coords.lat as f32))
                .collect();

            features.push(MapFeature {
                id: way.id.to_string(),
                properties: way.tags.unwrap_or_default(),
                geometry: vec![points.clone()],
            });
        }
    }


    Ok(features)
}

/// Parses OSM data from a string and returns a vector of map features. This takes in geojson data.
pub fn get_map_data(file_path: &str) -> Result<Vec<MapFeature>, Box<dyn std::error::Error>> {
    // Open and read the GeoJSON file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Parse the GeoJSON
    let geojson = GeoJson::from_reader(reader)?;

    let mut features = Vec::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if let Some(geometry) = feature.geometry {
                let mut coordinates = Vec::new();
                let mut road_coords = Vec::new();

                match geometry.value {
                    geojson::Value::Polygon(poly) => {
                        for ring in poly {
                            let points: Vec<Vec2> = ring
                                .iter()
                                .map(|pos| Vec2::new(pos[0] as f32, pos[1] as f32))
                                .collect();
                            coordinates.push(points);
                        }
                    }
                    geojson::Value::LineString(line) => {
                        let points: Vec<Vec2> = line
                            .iter()
                            .map(|pos| Vec2::new(pos[0] as f32, pos[1] as f32))
                            .collect();
                        road_coords.push(points);
                    }
                    geojson::Value::MultiPolygon(multi_poly) => {
                        for poly in multi_poly {
                            for ring in poly {
                                let points: Vec<Vec2> = ring
                                    .iter()
                                    .map(|pos| Vec2::new(pos[0] as f32, pos[1] as f32))
                                    .collect();
                                coordinates.push(points);
                            }
                        }
                    }
                    _ => continue,
                }

                features.push(MapFeature {
                    id: feature
                        .id
                        .map_or_else(|| String::from("unknown"), |id| format!("{:?}", id)),
                    properties: serde_json::Value::Object(feature.properties.unwrap_or_default()),
                    geometry: coordinates,
                });
            }
        }
    }

    Ok(features)
}


// Overpass API, thanks to: https://transform.tools/json-to-rust-serde
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OverpassResponse {
    pub version: Option<f64>,
    pub generator: Option<String>,
    pub osm3s: Option<Osm3s>,
    pub elements: Vec<Section>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Osm3s {
    #[serde(rename = "timestamp_osm_base")]
    pub timestamp_osm_base: Option<String>,
    #[serde(rename = "timestamp_areas_base")]
    pub timestamp_areas_base: Option<String>,
    pub copyright: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Section {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub tags: Option<serde_json::Value>,
    pub bounds: Option<Bounds>,
    #[serde(default)]
    pub nodes: Vec<i64>,
    #[serde(default)]
    pub geometry: Vec<Geometry>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Bounds {
    pub minlat: f64,
    pub minlon: f64,
    pub maxlat: f64,
    pub maxlon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Geometry {
    pub lat: f64,
    pub lon: f64,
}