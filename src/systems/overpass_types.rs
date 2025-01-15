use bevy::prelude::*;
/*
Overpass tags: 
    -- highway -- 
        motorway, truck, primary, seconary, tertairy (Major roads)
        residential, unclassified, serivce (Local roads)
        path, footway, cycleway, bridleway, track (Non-motorised)
    -- building --
        residential, commersial, industrail, publicm retail
        house, apartment, garage, school, hospital
    -- landuse -- 
        residential, commercial, industrial, retail, forest, farmland
    -- natural --
        water, wood, tree, rock, beach, peak, grass
    -- waterway --
        river, stream, canal, drain, ditch
    -- amenity -- (point of interests)
        school, college, university, kindergarten (education)
        hospital, clinic, doctors, dentist, pharmacy, veterinary (healthcare)
        restaurant, cafe, fast_food, bar, pub, food_court, ice_cream, biergarten (food/drink)
        bank, atm, marketplace, post_office, money_transfer, bureau_de_change, car_wash, car_rental, charging_station (Retail/Services)
        cinema, theatre, library, arts_centre, community_centre, fountain, playground, park (recreational)
        parking, bicycle_parking, bus_station, taxi, ferry_terminal, car_sharing (transport)
        townhall, courthouse, police, fire_station, prison, public_bath, social_facility, waste_disposal, recycling (public)
        place_of_worship
        shelter, hotel, hostel, guest_house, camp_site, caravan_site (accomodation)
        telephone, internet_cafe, post_box (communication)
        bench, drinking_water, toilets, shower, animal_shelter, sanitary_dump_station (misc)
    -- Transport Infrastructure --
        railway
            rail, subway, tram
        aeroway
            aerodrome, runway
        public_transport
            stop_position, station, platform
    -- boundary --
        administrative, national_park
    -- power --
        line, tower, substation
    -- manmade --
        pier, breakwater, bridge, tunnel, embankment
    -- leisure --
        park, pitch, garden, playground
    -- barrier --
        fence, wall, gate
    -- historic --
        monument, memorial, castle, ruins, archaeological_site
    -- toursim --
        hotel, camp_site, viewpoint
*/


#[derive(Component)]
pub struct SettingsOverlay {
    pub highwat: Option<Highway>,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Highway {
    // Roads
    pub motorway: bool,
    pub truck: bool,
    pub primary: bool,
    pub secondary: bool,
    pub tertiary: bool,
    pub residential: bool,
    // Link roads
    pub motorway_link: bool,
    pub trunk_link: bool,
    pub primary_link: bool,
    pub secondary_link: bool,
    pub tertiary_link: bool,
    // Special roads
    pub living_street: bool,
    pub service: bool,
    pub pedestrian: bool,
    pub track: bool,
    pub bus_guideway: bool,
    pub escape: bool,
    pub raceway: bool,
    pub road: bool,
    pub busway: bool,
    // Paths
    pub footway: bool,
    pub bridleway: bool,
    pub steps: bool,
    pub corridor: bool,
    pub path: bool,
    pub via_ferrata: bool,
    // Lifecycle
    pub proposed: bool,
    pub construction: bool,
    // Other 
    pub crossing: bool,
    pub cyclist_waiting_aid: bool,
    pub elevator: bool,
    pub emergency_bay: bool,
    pub emergency_access_point: bool,
    pub give_way: bool,
    pub ladder: bool,
    pub milestone: bool,
    pub mini_roundabout: bool,
    pub motorway_junction: bool,
    pub passing_place: bool,
    pub platform: bool,
    pub rest_area: bool,
    pub services: bool,
    pub speed_camera: bool,
    pub speed_display: bool,
    pub stop: bool,
    pub street_lamp: bool,
    pub toll_gantry: bool,
    pub traffic_mirror: bool,
    pub traffic_signals: bool,
    pub trailhead: bool,
    pub turning_circle: bool,
    pub turning_loop: bool,
}


#[derive(Component, Clone, Debug, PartialEq)]
pub struct Building {
    // Residential
    pub apartments: bool,
    pub barracks: bool,
    pub bungalow: bool,
    pub cabin: bool,
    pub detached: bool,
    pub annexe: bool,
    pub dormitory: bool,
    pub farm: bool,
    pub ger: bool,
    pub hotel: bool,
    pub house: bool,
    pub houseboat: bool,
    pub residential: bool,
    pub semidetached_house: bool,
    pub static_caravan: bool,
    pub stilt_house: bool,
    pub terrace: bool,
    pub tree_house: bool,
    pub trullo: bool,

    // Commercial
    pub commercial: bool,
    pub industrial: bool,
    pub kiosk: bool,
    pub office: bool,
    pub retail: bool,
    pub supermarket: bool,
    pub warehouse: bool,

    // Religious
    pub religious: bool,
    pub cathedral: bool,
    pub chapel: bool,
    pub church: bool,
    pub kingdom_hall: bool,
    pub monastery: bool,
    pub mosque: bool,
    pub presbytery: bool,
    pub shrine: bool,
    pub synagogue: bool,
    pub temple: bool,

    // Civic/Amenity
    pub bakehouse: bool,
    pub bridge: bool,
    pub civic: bool,
    pub college: bool,
    pub fire_station: bool,
    pub government: bool,
    pub gatehouse: bool,
    pub hospital: bool,
    pub kindergarten: bool,
    pub museum: bool,
    pub public: bool,
    pub school: bool,
    pub toilets: bool,
    pub train_station: bool,
    pub transportation: bool,
    pub university: bool,

    // Agricultural/Plant Production
    pub barn: bool,
    pub conservatory: bool,
    pub cowshed: bool,
    pub farm_auxiliary: bool,
    pub greenhouse: bool,
    pub slurry_tank: bool,
    pub stable: bool,
    pub sty: bool,
    pub livestock: bool,

    // Sports
    pub grandstand: bool,
    pub pavilion: bool,
    pub riding_hall: bool,
    pub sports_hall: bool,
    pub sports_centre: bool,
    pub stadium: bool,

    // Storage
    pub allotment_house: bool,
    pub boathouse: bool,
    pub hangar: bool,
    pub hut: bool,
    pub shed: bool,

    // Cars
    pub carport: bool,
    pub garage: bool,
    pub garages: bool,
    pub parking: bool,

    // Power/Technical Buildings
    pub digester: bool,
    pub service: bool,
    pub tech_cab: bool,
    pub transformer_tower: bool,
    pub water_tower: bool,
    pub storage_tank: bool,
    pub silo: bool,

    // Other Buildings
    pub beach_hut: bool,
    pub bunker: bool,
    pub castle: bool,
    pub construction: bool,
    pub container: bool,
    pub guardhouse: bool,
    pub military: bool,
    pub outbuilding: bool,
}

pub struct Amenity {
    // Food and Drink
    pub bar: bool,
    pub biergarten: bool,
    pub cafe: bool,
    pub fast_food: bool,
    pub food_court: bool,
    pub ice_cream: bool,
    pub r#pub: bool,
    pub restaurant: bool,

    // Education
    pub college: bool,
    pub dancing_school: bool,
    pub driving_school: bool,
    pub first_aid_school: bool,
    pub kindergarten: bool,
    pub language_school: bool,
    pub library: bool,
    pub music_school: bool,
    pub school: bool,
    pub traffic_park: bool,
    pub university: bool,
    pub research_institute: bool,
    pub training: bool,
    pub toy_library: bool,
    pub surf_school: bool,

    // Transportation
    pub bicycle_parking: bool,
    pub bicycle_repair_station: bool,
    pub bicycle_rental: bool,
    pub bicycle_wash: bool,
    pub boat_rental: bool,
    pub boat_sharing: bool,
    pub bus_station: bool,
    pub car_rental: bool,
    pub car_sharing: bool,
    pub car_wash: bool,
    pub compressed_air: bool,
    pub vehicle_inspection: bool,
    pub charging_station: bool,
    pub driver_training: bool,
    pub ferry_terminal: bool,
    pub fuel: bool,
    pub grit_bin: bool,
    pub motorcycle_parking: bool,
    pub parking: bool,
    pub parking_entrance: bool,
    pub parking_space: bool,
    pub taxi: bool,
    pub weighbridge: bool,

    // Financial
    pub atm: bool,
    pub bank: bool,
    pub bureau_de_change: bool,
    pub money_transfer: bool,
    pub payment_centre: bool,
    pub payment_terminal: bool,

    // Healthcare
    pub baby_hatch: bool,
    pub clinic: bool,
    pub dentist: bool,
    pub doctors: bool,
    pub hospital: bool,
    pub nursing_home: bool,
    pub pharmacy: bool,
    pub social_facility: bool,
    pub veterinary: bool,

    // Entertainment, Arts & Culture
    pub arts_centre: bool,
    pub brothel: bool,
    pub casino: bool,
    pub cinema: bool,
    pub community_centre: bool,
    pub conference_centre: bool,
    pub events_venue: bool,
    pub exhibition_centre: bool,
    pub fountain: bool,
    pub gambling: bool,
    pub love_hotel: bool,
    pub music_venue: bool,
    pub nightclub: bool,
    pub planetarium: bool,
    pub public_bookcase: bool,
    pub social_centre: bool,
    pub stage: bool,
    pub stripclub: bool,
    pub studio: bool,
    pub swingerclub: bool,
    pub theatre: bool,

    // Public Service
    pub courthouse: bool,
    pub fire_station: bool,
    pub police: bool,
    pub post_box: bool,
    pub post_depot: bool,
    pub post_office: bool,
    pub prison: bool,
    pub ranger_station: bool,
    pub townhall: bool,

    // Facilities
    pub bbq: bool,
    pub bench: bool,
    pub dog_toilet: bool,
    pub dressing_room: bool,
    pub drinking_water: bool,
    pub give_box: bool,
    pub lounge: bool,
    pub mailroom: bool,
    pub parcel_locker: bool,
    pub shelter: bool,
    pub shower: bool,
    pub telephone: bool,
    pub toilets: bool,
    pub water_point: bool,
    pub watering_place: bool,

    // Waste Management
    pub sanitary_dump_station: bool,
    pub recycling: bool,
    pub waste_basket: bool,
    pub waste_disposal: bool,
    pub waste_transfer_station: bool,

    // Others
    pub animal_boarding: bool,
    pub animal_breeding: bool,
    pub animal_shelter: bool,
    pub animal_training: bool,
    pub baking_oven: bool,
    pub clock: bool,
    pub crematorium: bool,
    pub dive_centre: bool,
    pub funeral_hall: bool,
    pub grave_yard: bool,
    pub hunting_stand: bool,
    pub internet_cafe: bool,
    pub kitchen: bool,
    pub kneipp_water_cure: bool,
    pub lounger: bool,
    pub marketplace: bool,
    pub monastery: bool,
    pub mortuary: bool,
    pub photo_booth: bool,
    pub place_of_mourning: bool,
    pub place_of_worship: bool,
    pub public_bath: bool,
    pub public_building: bool,
    pub refugee_site: bool,
    pub vending_machine: bool,
    pub user_defined: bool,
}