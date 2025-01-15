use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::MapFeature;

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

            // Check if this is a road
            let is_road = way
                .tags
                .as_ref()
                .and_then(|tags| tags.get("highway"))
                .is_some();

            features.push(MapFeature {
                id: way.id.to_string(),
                properties: way.tags.unwrap_or_default(),
                geometry: if is_road { vec![points.clone()] } else { vec![points.clone()] },
                road: if is_road { vec![points] } else { Vec::new() },
            });
        }
    }

    Ok(features)
}// Overpass API, thanks to: https://transform.tools/json-to-rust-serde

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverpassResponse {
    pub version: Option<f64>,
    pub generator: Option<String>,
    pub osm3s: Option<Osm3s>,
    pub elements: Vec<Section>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Osm3s {
    #[serde(rename = "timestamp_osm_base")]
    pub timestamp_osm_base: Option<String>,
    #[serde(rename = "timestamp_areas_base")]
    pub timestamp_areas_base: Option<String>,
    pub copyright: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
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
pub struct Bounds {
    pub minlat: f64,
    pub minlon: f64,
    pub maxlat: f64,
    pub maxlon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub lat: f64,
    pub lon: f64,
}


#[derive(Debug, Deserialize)]
struct Way {
    #[serde(rename = "type")]
    way_type: String,
    id: u64,
    geometry: Option<Vec<Coordinates>>,
    tags: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Coordinates {
    lat: f64,
    lon: f64,
}