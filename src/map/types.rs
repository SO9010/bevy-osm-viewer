use bevy::prelude::*;
use super::projection::lat_lon_to_mercator;

// E.g Cambridge as the Starting point, make this a global entity/constant
pub const STARTING_LONG_LAT: Vec2 = Vec2::new(0.1494117, 52.192_37);
pub const SCALE: f32 = 10000000.0;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MapFeature {
    pub id: String,
    pub properties: serde_json::Value, // Use serde_json for flexible properties such as buidling type
    pub geometry: Vec<Vec<Vec2>>,      // Nested Vec2 to represent Polygon coordinates
    pub road: Vec<Vec<Vec2>>,       // Roads are a special case of polygons, they are not closed
}

#[derive(Component, Clone, Debug)]
pub struct RefrencePoint {
    pub long: f32, // Longitude of the map's reference point
    pub lat: f32,  // Latitude of the map's reference point
}

impl RefrencePoint {
    pub fn new(long: f32, lat: f32) -> Self {
        Self { long, lat }
    }

    pub fn get_long_lat(&self) -> Vec2 {
        Vec2::new(self.long, self.lat)
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct WorldSpaceRect {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl WorldSpaceRect {
    pub fn intersects(&self, other: &WorldSpaceRect) -> bool {
        self.left < other.right && self.right > other.left && self.bottom < other.top && self.top > other.bottom
    }
    // This will split the current rect into multiple rects, it really struggles with getting anyhting if it is overflowing to the left.
    pub fn split(&self, other: &WorldSpaceRect) -> Option<Vec<WorldSpaceRect>> {
        if !self.intersects(other) {
            return None;
        }

        let mut result = Vec::new();

        // Define the intersection boundaries
        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        let bottom = self.bottom.max(other.bottom);
        let top = self.top.min(other.top);

        // Add the left region
        if self.left < left {
            result.push(WorldSpaceRect {
                left: self.left,
                right: left,
                bottom: self.bottom,
                top: self.top,
            });
        }

        // Add the right region
        if self.right > right {
            result.push(WorldSpaceRect {
                left: right,
                right: self.right,
                bottom: self.bottom,
                top: self.top,
            });
        }

        // Add the bottom region
        if self.bottom < bottom {
            result.push(WorldSpaceRect {
                left,
                right,
                bottom: self.bottom,
                top: bottom,
            });
        }

        // Add the top region
        if self.top > top {
            result.push(WorldSpaceRect {
                left,
                right,
                bottom: top,
                top: self.top,
            });
        }

        Some(result)
    }
}

#[derive(Component, Clone, Debug)]
pub struct MapPoints {
    pub bounding_boxes: Vec<WorldSpaceRect>, // Bounding box of the map, this will be what will be used to select a section of the map to load. It is a polygon of longs and lats
    pub refrencee_point: RefrencePoint, // Refrence point of the map, this is used to calculate the scale and offset
}

#[derive(Component, Clone, Debug)]
pub struct MapBundle {
    pub features: Vec<MapFeature>,

    pub map_points: MapPoints, // Map points of the map, this is used to calculate the scale and offset

    pub scale: f32, // Global scale for rendering (used for Mercator projection)
}


impl MapBundle {
    pub fn new(long: f32, lat: f32, scale: f32) -> Self {
        Self {
            features: Vec::new(),
            map_points: MapPoints {
                bounding_boxes: Vec::new(),
                refrencee_point: RefrencePoint::new(long, lat),
            },
            scale,
        }
    }

    // Method to apply a Mercator projection to a coordinate, otherwise the coordinates will be too small to be rendered
    pub fn lat_lon_to_mercator(&self, lat: f32, lon: f32) -> Vec2 {
        lat_lon_to_mercator(lat, lon, self.scale, self.map_points.refrencee_point.long, self.map_points.refrencee_point.lat)
    }
}