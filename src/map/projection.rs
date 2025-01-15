use bevy::math::Vec2;

use super::WorldSpaceRect;

pub fn lat_lon_to_mercator(lat: f32, lon: f32, scale: f32, ref_long: f32, ref_lat: f32) -> Vec2 {
    // Get an offset
    let offset_long = lon - ref_long;
    let offset_lat = lat - ref_lat;

    // Apply the projection
    let x = scale * offset_long.to_radians();
    let y = scale * (std::f32::consts::PI / 4.0 + offset_lat.to_radians() / 2.0).tan().ln();

    Vec2::new(x, y)
} 

pub fn mercator_to_lat_lon(x: f32, y: f32, scale: f32, ref_long: f32, ref_lat: f32) -> (f32, f32) {
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
    
        let (lat1, lon1) = mercator_to_lat_lon(rect.right, rect.bottom, scale, ref_long, ref_lat);
        let (lat2, lon2) = mercator_to_lat_lon(rect.left, rect.top, scale, ref_long, ref_lat);
    
        
        // this gives { left: 0.13576442, right: 0.14162578, bottom: 52.197693, top: 52.19229 }
    WorldSpaceRect {
        left: lon1,
        right: lon2,
        bottom: lat1,
        top: lat2,
    }
}