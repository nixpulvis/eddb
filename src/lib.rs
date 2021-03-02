use std::io;
use std::fs::File;
use std::path::Path;
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
            if !callback(system.unwrap()) {
                break;
            }
        }
    }

    pub fn each_json(file_path: &str, callback: &mut dyn FnMut(System) -> bool) {
        let file = File::open(file_path).unwrap();
        for system in serde_json::from_reader::<_, Vec<System>>(file).unwrap() {
            if !callback(system) {
                break;
            }
        }
    }
}

// TODO: Error type.

// TODO: Remove `csv` function in place of more general `new` (see below)
// Dump::new(path)  // check filetype and create generic reader.
// Dump::into_iter() -> CsvIterator | JsonIterator somehow.
pub struct Dump(csv::Reader<File>);

impl Dump {
    pub fn csv<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader = csv::Reader::from_reader(file);
        Ok(Dump(reader))
    }
}

impl<'a> IntoIterator for &'a mut Dump {
    type Item = Result<System, csv::Error>;
    type IntoIter = CsvSystemIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.0.deserialize().into_iter();
        CsvSystemIterator(iter)
    }
}

pub struct CsvSystemIterator<'a>(csv::DeserializeRecordsIter<'a, File, System>);

impl<'a> Iterator for CsvSystemIterator<'a> {
    type Item = Result<System, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// TODO: Can we trick serde into assuming our JSON is just an array ([...]) of objects to parse
// to we can iterator without building a full Vec ahead of time?
pub struct JsonSystemIterator(std::vec::IntoIter<System>);

impl JsonSystemIterator {
    pub fn from_file(reader: &mut File) -> Result<Self, serde_json::Error> {
        let vec = serde_json::from_reader::<_, Vec<System>>(reader)?;
        Ok(JsonSystemIterator(vec.into_iter()))
    }
}

impl Iterator for JsonSystemIterator {
    type Item = System;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// /// > This file will not be updated right now.
// struct Body {}
