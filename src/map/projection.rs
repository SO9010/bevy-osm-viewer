use std::f32::consts::PI;

use bevy::math::Vec2;

use super::WorldSpaceRect;

pub fn lat_lon_to_tile_mercator(lat_deg: f64, lon_deg: f64, zoom: i32) -> (i32, i32) {
    let n = (1 << zoom) as f64;

    let x_tile = (n * (lon_deg + 180.0) / 360.0) as i32;

    let lat_rad = lat_deg.to_radians();
    let y_tile = (n * (1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / PI as f64) / 2.0) as i32;

    (x_tile, y_tile)
} 

pub fn tile_to_lat_lon(x_tile: i32, y_tile: i32, zoom: i32) -> (f64, f64) {
    let n = (1 << zoom) as f64;

    let lon_deg = x_tile as f64 / n * 360.0 - 180.0;

    let lat_rad = (std::f64::consts::PI * (1.0 - 2.0 * y_tile as f64 / n)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    (lat_deg, lon_deg)
}

pub fn lat_lon_to_world_mercator(lat: f32, lon: f32, scale: f32, ref_long: f32, ref_lat: f32) -> Vec2 {
    // Get an offset
    let offset_long = lon - ref_long;
    let offset_lat = lat - ref_lat;

    // Apply the projection
    let x = scale * offset_long.to_radians();
    let y = scale * (std::f32::consts::PI / 4.0 + offset_lat.to_radians() / 2.0).tan().ln();

    Vec2::new(x, y)
} 

pub fn world_mercator_to_lat_lon(x: f32, y: f32, scale: f32, ref_long: f32, ref_lat: f32) -> (f32, f32) {
    // Reverse the projection for longitude
    let lon = x / scale;
    
    // Reverse the projection for latitude
    let lat = (2.0 * ((y / scale).exp().atan()) - std::f32::consts::PI / 2.0).to_degrees();
    
    // Add the offsets back to reference longitude and latitude
    let lon = lon.to_degrees() + ref_long;
    let lat = lat + ref_lat;

    (lat, lon)
}

pub fn world_space_rect_to_lat_long(rect: WorldSpaceRect, scale: f32, ref_long: f32, ref_lat: f32) -> WorldSpaceRect {
    
        let (lat1, lon1) = world_mercator_to_lat_lon(rect.right, rect.bottom, scale, ref_long, ref_lat);
        let (lat2, lon2) = world_mercator_to_lat_lon(rect.left, rect.top, scale, ref_long, ref_lat);
    
        
        // this gives { left: 0.13576442, right: 0.14162578, bottom: 52.197693, top: 52.19229 }
    WorldSpaceRect {
        left: lon1,
        right: lon2,
        bottom: lat1,
        top: lat2,
    }
}

pub fn bounding_box_to_tiles(bbox: WorldSpaceRect, zoom: i32) -> Vec<(i32, i32)> {
    let (min_lat, min_lon) = (bbox.bottom, bbox.left);
    let (max_lat, max_lon) = (bbox.top, bbox.right);

    let (min_x_tile, min_y_tile) = lat_lon_to_tile_mercator(min_lat as f64, min_lon as f64, zoom);
    let (max_x_tile, max_y_tile) = lat_lon_to_tile_mercator(max_lat as f64, max_lon as f64, zoom);

    let mut tiles = Vec::new();
    for x in min_x_tile..=max_x_tile {
        for y in min_y_tile..=max_y_tile {
            tiles.push((x, y));
        }
    }

    tiles
}