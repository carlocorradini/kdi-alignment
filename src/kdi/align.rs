use chrono::{NaiveDate, NaiveTime};
use csv::{ReaderBuilder, Trim};
use gtfs_structures::Gtfs;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fmt::Display, fs::File};
use zip::ZipArchive;

use super::enums::{
    KdiDirectionEnum, KdiExceptionEnum, KdiFareEnum, KdiSupportedEnum, KdiTransportEnum,
};
use super::kml::Kml;
use super::structs::{
    KdiCalendar, KdiCalendarException, KdiFare, KdiFareRule, KdiLocation, KdiPublicTransportStop,
    KdiRoute, KdiStopTime, KdiTrip,
};

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

fn to_correct_id(tt: &TT, id: &str) -> String {
    format!("{}_{}", tt, id)
}

pub fn align_location_zone(
    archive: &mut ZipArchive<File>,
    locations: &mut Vec<KdiLocation>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct KdiZone {
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
        locations.push(KdiLocation {
            id: format!("ZONE_{}", to_correct_id(&tt, &zone.id)),
            name: zone.name,
            latitude: zone.latitude,
            longitude: zone.longitude,
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_location_public_transport_stop(
    gtfs: &Gtfs,
    locations: &mut Vec<KdiLocation>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for stop in gtfs.stops.values() {
        locations.push(KdiLocation {
            id: to_correct_id(&tt, &stop.id),
            name: stop.name.clone(),
            latitude: stop.latitude.unwrap(),
            longitude: stop.longitude.unwrap(),
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_location_car_sharing(
    car_sharing: &Kml,
    locations: &mut Vec<KdiLocation>,
) -> Result<(), Box<dyn Error>> {
    for (i, placemark) in car_sharing.document.folder.placemarks.iter().enumerate() {
        let mut datas = placemark.extended_data.schema_data.simple_datas.iter();
        let coordinate: Vec<_> = placemark
            .point
            .coordinates
            .split(",")
            .map(|c| c.parse::<f64>().unwrap())
            .collect();
        assert!(coordinate.len() == 2);

        locations.push(KdiLocation {
            id: format!("CS_{}", i),
            name: datas.find(|d| d.name == "nomepos").unwrap().value.clone(),
            latitude: coordinate[1],
            longitude: coordinate[0],
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_location_centro_in_bici(
    centro_in_bici: &Kml,
    locations: &mut Vec<KdiLocation>,
) -> Result<(), Box<dyn Error>> {
    for (i, placemark) in centro_in_bici.document.folder.placemarks.iter().enumerate() {
        let mut datas = placemark.extended_data.schema_data.simple_datas.iter();
        let coordinate: Vec<_> = placemark
            .point
            .coordinates
            .split(",")
            .map(|c| c.parse::<f64>().unwrap())
            .collect();
        assert!(coordinate.len() == 2);

        locations.push(KdiLocation {
            id: format!("CIB_{}", i),
            name: datas.find(|d| d.name == "desc").unwrap().value.clone(),
            latitude: coordinate[1],
            longitude: coordinate[0],
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_location_parcheggio_protetto_biciclette(
    parcheggio_protetto_biciclette: &Kml,
    locations: &mut Vec<KdiLocation>,
) -> Result<(), Box<dyn Error>> {
    for (i, placemark) in parcheggio_protetto_biciclette
        .document
        .folder
        .placemarks
        .iter()
        .enumerate()
    {
        let mut datas = placemark.extended_data.schema_data.simple_datas.iter();
        let coordinate: Vec<_> = placemark
            .point
            .coordinates
            .split(",")
            .map(|c| c.parse::<f64>().unwrap())
            .collect();
        assert!(coordinate.len() == 2);

        locations.push(KdiLocation {
            id: format!("PPB_{}", i),
            name: datas.find(|d| d.name == "park").unwrap().value.clone(),
            latitude: coordinate[1],
            longitude: coordinate[0],
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_location_taxi(
    taxi: &Kml,
    locations: &mut Vec<KdiLocation>,
) -> Result<(), Box<dyn Error>> {
    for (i, placemark) in taxi.document.folder.placemarks.iter().enumerate() {
        let mut datas = placemark.extended_data.schema_data.simple_datas.iter();
        let coordinate: Vec<_> = placemark
            .point
            .coordinates
            .split(",")
            .map(|c| c.parse::<f64>().unwrap())
            .collect();
        assert!(coordinate.len() == 2);

        locations.push(KdiLocation {
            id: format!("TX_{}", i),
            name: datas.find(|d| d.name == "nome").unwrap().value.clone(),
            latitude: coordinate[1],
            longitude: coordinate[0],
        });
    }

    locations.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_calendar_exception(
    gtfs: &Gtfs,
    calendar_exceptions: &mut Vec<KdiCalendarException>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for calendar_date in gtfs.calendar_dates.values() {
        for cd in calendar_date {
            calendar_exceptions.push(KdiCalendarException {
                calendar: to_correct_id(&tt, &cd.service_id),
                date: cd
                    .date
                    .and_time(NaiveTime::from_hms(0, 0, 0))
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string(),
                exception: KdiExceptionEnum::from(cd.exception_type),
            });
        }
    }

    calendar_exceptions.sort_by(|a, b| {
        a.calendar
            .cmp(&b.calendar)
            .then_with(|| a.date.cmp(&b.date))
    });

    Ok(())
}

pub fn align_calendar(
    gtfs: &Gtfs,
    calendars: &mut Vec<KdiCalendar>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for calendar in gtfs.calendar.values() {
        calendars.push(KdiCalendar {
            id: to_correct_id(&tt, &calendar.id),
            start_date: calendar
                .start_date
                .and_time(NaiveTime::from_hms(0, 0, 0))
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            end_date: calendar
                .end_date
                .and_time(NaiveTime::from_hms(0, 0, 0))
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            monday: calendar.monday,
            tuesday: calendar.tuesday,
            wednesday: calendar.wednesday,
            thursday: calendar.thursday,
            friday: calendar.friday,
            saturday: calendar.saturday,
            sunday: calendar.sunday,
        });
    }

    calendars.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_fare_rule(
    archive: &mut ZipArchive<File>,
    fare_rules: &mut Vec<KdiFareRule>,
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
            fare: to_correct_id(&tt, &fare_rule.fare).to_string(),
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
            fare: to_correct_id(&tt, &fare_rule.fare).to_string(),
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
            fare: to_correct_id(&tt, &fare_rule.fare).to_string(),
            origin: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin)),
            destination: format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination)),
        });
    }

    fare_rules.sort_by(|a, b| a.fare.cmp(&b.fare));

    Ok(())
}

pub fn align_fare(
    archive: &mut ZipArchive<File>,
    fares: &mut Vec<KdiFare>,
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
            id: to_correct_id(&tt, &fare.id).to_string(),
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
            id: to_correct_id(&tt, &fare.id).to_string(),
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
            id: to_correct_id(&tt, &fare.id).to_string(),
            ftype: KdiFareEnum::Mobile,
            ..fare
        });
    }

    fares.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

pub fn align_public_transport_stop(
    gtfs: &Gtfs,
    public_transport_stops: &mut Vec<KdiPublicTransportStop>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for stop in gtfs.stops.values() {
        public_transport_stops.push(KdiPublicTransportStop {
            location: to_correct_id(&tt, &stop.id),
            zone: if stop.zone_id.is_some() {
                Some(format!(
                    "ZONE_{}",
                    to_correct_id(&tt, stop.zone_id.as_ref().unwrap())
                ))
            } else {
                None
            },
            // TODO Fixme!
            ptype: Vec::new(),
            weelchair: KdiSupportedEnum::from(stop.wheelchair_boarding),
        });
    }

    public_transport_stops.sort_by(|a, b| a.location.cmp(&b.location));

    Ok(())
}

pub fn align_stop_time(
    gtfs: &Gtfs,
    stop_times: &mut Vec<KdiStopTime>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for trip in gtfs.trips.values() {
        for stop_time in &trip.stop_times {
            stop_times.push(KdiStopTime {
                trip: to_correct_id(&tt, &trip.id),
                stop: to_correct_id(&tt, &stop_time.stop.id),
                arrival: stop_time.arrival_time.map(|time| {
                    NaiveDate::from_ymd(0, 1, 1 + (time / 86_400))
                        .and_time(NaiveTime::from_num_seconds_from_midnight(time % 86_400, 0))
                        .format("%Y-%m-%dT%H:%M:%S")
                        .to_string()
                }),
                departure: stop_time.departure_time.map(|time| {
                    NaiveDate::from_ymd(0, 1, 1 + (time / 86_400))
                        .and_time(NaiveTime::from_num_seconds_from_midnight(time % 86_400, 0))
                        .format("%Y-%m-%dT%H:%M:%S")
                        .to_string()
                }),
                sequence: usize::from(stop_time.stop_sequence),
            })
        }
    }

    stop_times.sort_by(|a, b| {
        a.trip
            .cmp(&b.trip)
            .then_with(|| a.sequence.cmp(&b.sequence))
    });

    Ok(())
}

pub fn align_trip<'a, 'b>(
    gtfs: &'a Gtfs,
    trips: &'b mut Vec<KdiTrip<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for trip in gtfs.trips.values() {
        trips.push(KdiTrip {
            id: to_correct_id(&tt, &trip.id),
            route: to_correct_id(&tt, &trip.route_id),
            calendar: to_correct_id(&tt, &trip.service_id),
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
    for route in gtfs.routes.values() {
        routes.push(KdiRoute {
            id: to_correct_id(&tt, &route.id),
            agency: route.agency_id.as_ref().unwrap(),
            short_name: &route.short_name,
            long_name: &route.long_name,
            transport: KdiTransportEnum::from(route.route_type),
        });
    }

    routes.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}
