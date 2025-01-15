use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Loc {
    pub latitude: f64,
    pub longitude: f64,
}

impl Loc {
    pub fn new() -> Loc {
        let lat_and_long = get_loc().unwrap();
        Loc {
            latitude: lat_and_long.latitude,
            longitude: lat_and_long.longitude,
        }
    }
    
}

fn get_loc() -> Result<Loc, ureq::Error> {
    let response: String = ureq::get("http://ip-api.com/json")
        .call()?
        .into_string()?;
    
    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    
    let latitude = json["lat"].as_f64().unwrap_or(0.0);
    let longitude = json["lon"].as_f64().unwrap_or(0.0);
    
    Ok(Loc {
        latitude,
        longitude,
    })
}