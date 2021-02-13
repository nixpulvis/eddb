use std::fs::File;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

mod serde_utils;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// EDDB's representation of a solar system
#[derive(Deserialize, Debug)]
pub struct System {
    pub name: String,
    pub id: u64,
    pub edsm_id: Option<u64>,
    pub ed_system_address: Option<u64>,
    pub simbad_ref: Option<String>,

    #[serde(flatten)]
    pub coords: Coordinate,
    #[serde(deserialize_with = "serde_utils::bool_or_bit")]
    pub is_populated: bool,
    pub population: Option<u64>,
    #[serde(deserialize_with = "serde_utils::bool_or_bit")]
    pub needs_permit: bool,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,

    pub government: Option<String>,
    pub government_id: Option<u64>,

    pub allegiance: Option<String>,
    pub allegiance_id: Option<u64>,

    pub security: Option<String>,
    pub security_id: Option<u64>,

    pub primary_economy: Option<String>,
    pub primary_economy_id: Option<u64>,

    pub power: Option<String>,
    pub power_state: Option<String>,
    pub power_state_id: Option<u64>,

    pub controlling_minor_faction: Option<String>,
    pub controlling_minor_faction_id: Option<u64>,

    pub reserve_type: Option<String>,
    pub reserve_type_id: Option<u64>,
}

impl System {
    pub fn each_csv(file_path: &str, callback: &mut dyn FnMut(System) -> bool) {
        let file = File::open(file_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        for system in reader.deserialize().into_iter() {
            if callback(system.unwrap()) {
                break;
            }
        }
    }

    pub fn each_json(file_path: &str, callback: &mut dyn FnMut(System) -> bool) {
        let file = File::open(file_path).unwrap();
        for system in serde_json::from_reader::<_, Vec<System>>(file).unwrap() {
            if callback(system) {
                break;
            }
        }
    }
}

// /// > This file will not be updated right now.
// struct Body {}
