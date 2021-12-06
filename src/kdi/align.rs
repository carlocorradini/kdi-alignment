use csv::{ReaderBuilder, Trim};
use gtfs_structures::Gtfs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Read;
use std::{fmt::Display, fs::File};
use zip::ZipArchive;

use super::enums::{KdiDirectionEnum, KdiFareEnum, KdiSupportedEnum, KdiTransportEnum};
use super::structs::{KdiFare, KdiFareRule, KdiLocation, KdiRoute, KdiTrip};

#[derive(PartialEq)]
pub enum TT {
    Urban,
    ExtraUrban,
}

impl Display for TT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TT::ExtraUrban => write!(f, "EU"),
            TT::Urban => write!(f, "U"),
        }
    }
}

fn to_correct_id(tt: &TT, id: &String) -> String {
    format!("{}_{}", tt, id)
}

pub fn align_location_zone<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    zones: &'b mut Vec<KdiLocation>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KdiZone {
        #[serde(rename(deserialize = "ZONE_ID"))]
        pub id: String,
        #[serde(rename(deserialize = "ZONE_NAME"))]
        pub name: String,
        #[serde(rename(deserialize = "ZONE_LAT"))]
        pub latitude: f64,
        #[serde(rename(deserialize = "ZONE_LON"))]
        pub longitude: f64,
    }

    let mut zones_string: String = String::new();

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/zones_extraurbano.txt"
        } else {
            "tariffegtfsurbano/zones_urbano.txt"
        })?
        .read_to_string(&mut zones_string)?;

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(zones_string.as_bytes())
        .deserialize()
    {
        let zone: KdiZone = result?;
        zones.push(KdiLocation {
            id: format!("ZONE_{}", to_correct_id(&tt, &zone.id)),
            name: zone.name,
            latitude: zone.latitude,
            longitude: zone.longitude,
        });
    }

    zones.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_fare<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    fares: &'b mut Vec<KdiFare>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    let mut fares_cash_string: String = String::new();
    let mut fares_cartascalare_string: String = String::new();
    let mut fares_mobile_string: String = String::new();

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano.txt"
        })?
        .read_to_string(&mut fares_cash_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano_cartascalare.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano_cartascalare.txt"
        })?
        .read_to_string(&mut fares_cartascalare_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano_mobile.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano_mobile.txt"
        })?
        .read_to_string(&mut fares_mobile_string)?;

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fares_cash_string.as_bytes())
        .deserialize()
    {
        let fare: KdiFare = result?;
        fares.push(KdiFare {
            id: to_correct_id(&tt, &fare.id),
            ftype: KdiFareEnum::Cash,
            ..fare
        });
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fares_cartascalare_string.as_bytes())
        .deserialize()
    {
        let fare: KdiFare = result?;
        fares.push(KdiFare {
            id: to_correct_id(&tt, &fare.id),
            ftype: KdiFareEnum::Cartascalare,
            ..fare
        });
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fares_mobile_string.as_bytes())
        .deserialize()
    {
        let fare: KdiFare = result?;
        fares.push(KdiFare {
            id: to_correct_id(&tt, &fare.id),
            ftype: KdiFareEnum::Mobile,
            ..fare
        });
    }

    fares.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_trip<'a, 'b>(
    gtfs: &'a Gtfs,
    trips: &'b mut Vec<KdiTrip<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, trip) in &gtfs.trips {
        trips.push(KdiTrip {
            id: to_correct_id(&tt, &trip.id),
            route_id: to_correct_id(&tt, &trip.route_id),
            calendar_id: to_correct_id(&tt, &trip.service_id),
            name: trip.trip_headsign.as_ref().unwrap(),
            direction: KdiDirectionEnum::from(trip.direction_id.unwrap()),
            weelchair: KdiSupportedEnum::from(trip.wheelchair_accessible),
            bike: KdiSupportedEnum::from(trip.bikes_allowed),
        })
    }

    trips.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_route<'a, 'b>(
    gtfs: &'a Gtfs,
    routes: &'b mut Vec<KdiRoute<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, route) in &gtfs.routes {
        routes.push(KdiRoute {
            id: to_correct_id(&tt, &route.id),
            agency_id: &route.agency_id.as_ref().unwrap(),
            short_name: &route.short_name,
            long_name: &route.long_name,
            transport: KdiTransportEnum::from(route.route_type),
        });
    }

    routes.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_fare_rule<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    fare_rules: &'b mut Vec<KdiFareRule>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    let mut fare_rules_cash_string: String = String::new();
    let mut fare_rules_cartascalare_string: String = String::new();
    let mut fare_rules_mobile_string: String = String::new();

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_rules_extraurbano.txt"
        } else {
            "tariffegtfsurbano/fare_rules_urbano.txt"
        })?
        .read_to_string(&mut fare_rules_cash_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_rules_extraurbano_cartascalare.txt"
        } else {
            "tariffegtfsurbano/fare_rules_urbano_cartascalare.txt"
        })?
        .read_to_string(&mut fare_rules_cartascalare_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_rules_extraurbano_mobile.txt"
        } else {
            "tariffegtfsurbano/fare_rules_urbano_mobile.txt"
        })?
        .read_to_string(&mut fare_rules_mobile_string)?;

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_rules_cash_string.as_bytes())
        .deserialize()
    {
        let fare_rule: KdiFareRule = result?;
        fare_rules.push(KdiFareRule {
            fare_id: to_correct_id(&tt, &fare_rule.fare_id),
            origin: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin)),
            destination: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination)),
        });
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_rules_cartascalare_string.as_bytes())
        .deserialize()
    {
        let fare_rule: KdiFareRule = result?;
        fare_rules.push(KdiFareRule {
            fare_id: to_correct_id(&tt, &fare_rule.fare_id),
            origin: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin)),
            destination: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination)),
        });
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_rules_mobile_string.as_bytes())
        .deserialize()
    {
        let fare_rule: KdiFareRule = result?;
        fare_rules.push(KdiFareRule {
            fare_id: to_correct_id(&tt, &fare_rule.fare_id),
            origin: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin)),
            destination: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination)),
        });
    }

    fare_rules.sort_by(|a, b| a.fare_id.cmp(&b.fare_id));

    Ok(())
}
