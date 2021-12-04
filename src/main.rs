mod kdigtfs;

use chrono::{NaiveDate, NaiveTime};
use csv::{ReaderBuilder, Trim};
use env_logger::{Builder, Target};
use gtfs_structures::Gtfs;
use kdigtfs::{KdiDirectionEnum, KdiFareEnum, KdiStopTime, KdiTrip};
use log::{debug, info, LevelFilter};
use serde_json::json;
use std::error::Error;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::Read;
use strum::VariantNames;
use zip::ZipArchive;

use crate::kdigtfs::{
    KdiAgency, KdiCalendar, KdiCalendarException, KdiCurrencyEnum, KdiExceptionEnum, KdiFare,
    KdiFareRule, KdiPaymentEnum, KdiRoute, KdiStop, KdiStopEnum, KdiSupportedEnum,
    KdiTransportEnum, KdiZone,
};

const ALIGNEMENT_DIR: &'static str = "./alignment";
const EXTRAURBAN_FILE: &'static str = "./data/extraurban.zip";
const URBAN_FILE: &'static str = "./data/urban.zip";
const EXTRAURBAN_FARE_FILE: &'static str = "./data/extraurban_fare.zip";
const URBAN_FARE_FILE: &'static str = "./data/urban_fare.zip";

#[derive(PartialEq)]
enum TT {
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

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        .init();

    // Remove and recreate alignment directory
    info!("Removing `{}` directory", ALIGNEMENT_DIR);
    fs::remove_dir_all(ALIGNEMENT_DIR).ok();
    info!("Creating `{}` directory", ALIGNEMENT_DIR);
    fs::create_dir(ALIGNEMENT_DIR)?;

    // Read GTFS files
    info!("Reading `{}`", EXTRAURBAN_FILE);
    let gtfs_extraurban = Gtfs::new(EXTRAURBAN_FILE)?;

    info!("Reading `{}`", URBAN_FILE);
    let gtfs_urban = Gtfs::new(URBAN_FILE)?;

    // Read FARE files
    info!("Reading `{}`", EXTRAURBAN_FARE_FILE);
    let mut extraurban_fare = ZipArchive::new(File::open(&EXTRAURBAN_FARE_FILE)?)?;

    info!("Reading `{}`", URBAN_FARE_FILE);
    let mut urban_fare = ZipArchive::new(File::open(&URBAN_FARE_FILE)?)?;

    // agency.txt
    info!("Aligning `agency.txt`");
    let agency: KdiAgency;

    assert!(gtfs_extraurban.agencies.len() == 1);
    assert!(gtfs_urban.agencies.len() == 1);
    {
        let gtfs_agency = gtfs_extraurban.agencies.first().unwrap();
        agency = KdiAgency {
            id: gtfs_agency.id.as_ref().unwrap().to_string(),
            name: &gtfs_agency.name,
            email: "info@trentinotrasporti.it",
            phone: &gtfs_agency.phone.as_ref().unwrap(),
            url: &gtfs_agency.url,
        };
    }
    info!("Writing `agency.json` file");
    fs::write(
        format!("{}/agency.json", ALIGNEMENT_DIR),
        serde_json::to_string(&agency)?,
    )?;

    // stops.txt
    info!("Aligning `stops.txt`");
    let mut stops: Vec<KdiStop> = Vec::new();

    debug!("Aligning extraurban `stops.txt`");
    align_stops(&gtfs_extraurban, &mut stops, TT::ExtraUrban)?;
    debug!("Aligning urban `stops.txt`");
    align_stops(&gtfs_urban, &mut stops, TT::Urban)?;
    info!("Writing `stop.json` file");
    fs::write(
        format!("{}/stop.json", ALIGNEMENT_DIR),
        serde_json::to_string(&stops)?,
    )?;

    // stop_times.txt
    info!("Aligning `stop_times.txt`");
    let mut stop_times: Vec<KdiStopTime> = Vec::new();

    debug!("Aligning extraurban `stop_times.txt`");
    align_stop_times(&gtfs_extraurban, &mut stop_times, TT::ExtraUrban)?;
    debug!("Aligning urban `stop_times.txt`");
    align_stop_times(&gtfs_urban, &mut stop_times, TT::Urban)?;
    info!("Writing `stop_time.json` file");
    fs::write(
        format!("{}/stop_time.json", ALIGNEMENT_DIR),
        serde_json::to_string(&stop_times)?,
    )?;

    // routes.txt
    info!("Aligning `routes.txt`");
    let mut routes: Vec<KdiRoute> = Vec::new();

    debug!("Aligning extraurban `routes.txt`");
    align_routes(&gtfs_extraurban, &mut routes, TT::ExtraUrban)?;
    debug!("Aligning urban `routes.txt`");
    align_routes(&gtfs_urban, &mut routes, TT::Urban)?;
    info!("Writing `route.json` file");
    fs::write(
        format!("{}/route.json", ALIGNEMENT_DIR),
        serde_json::to_string(&routes)?,
    )?;

    // calendar.txt
    info!("Aligning `calendar.txt`");
    let mut calendars: Vec<KdiCalendar> = Vec::new();

    debug!("Aligning extraurban `calendar.txt`");
    align_calendar(&gtfs_extraurban, &mut calendars, TT::ExtraUrban)?;
    debug!("Aligning urban `calendar.txt`");
    align_calendar(&gtfs_urban, &mut calendars, TT::Urban)?;
    info!("Writing `calendar.json` file");
    fs::write(
        format!("{}/calendar.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars)?,
    )?;

    // calendar_dates.txt
    info!("Aligning `calendar_dates.txt`");
    let mut calendars_exception: Vec<KdiCalendarException> = Vec::new();

    debug!("Aligning extraurban `calendar_dates.txt`");
    align_calendar_dates(&gtfs_extraurban, &mut calendars_exception, TT::ExtraUrban)?;
    debug!("Aligning urban `calendar_dates.txt`");
    align_calendar_dates(&gtfs_urban, &mut calendars_exception, TT::Urban)?;
    info!("Writing `calendar_exception.json` file");
    fs::write(
        format!("{}/calendar_exception.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars_exception)?,
    )?;

    // trips.txt
    info!("Aligning `trips.txt`");
    let mut trips: Vec<KdiTrip> = Vec::new();

    debug!("Aligning extraurban `trips.txt`");
    align_trips(&gtfs_extraurban, &mut trips, TT::ExtraUrban)?;
    debug!("Aligning urban `trips.txt`");
    align_trips(&gtfs_urban, &mut trips, TT::Urban)?;
    info!("Writing `trip.json` file");
    fs::write(
        format!("{}/trip.json", ALIGNEMENT_DIR),
        serde_json::to_string(&trips)?,
    )?;

    // zones.txt
    info!("Aligning `zones.txt`");
    let mut zones: Vec<KdiZone> = Vec::new();

    debug!("Aligning extraurban `zones.txt`");
    align_zones(&mut extraurban_fare, &mut zones, TT::ExtraUrban)?;
    debug!("Aligning urban `zones.txt`");
    align_zones(&mut urban_fare, &mut zones, TT::Urban)?;
    info!("Writing `zone.json` file");
    fs::write(
        format!("{}/zone.json", ALIGNEMENT_DIR),
        serde_json::to_string(&zones)?,
    )?;

    // fare_attributes.txt
    info!("Aligning `fare_attributes.txt`");
    let mut fares: Vec<KdiFare> = Vec::new();

    debug!("Aligning extraurban `fare_attributes.txt`");
    align_fare_attributes(&mut extraurban_fare, &mut fares, TT::ExtraUrban)?;
    debug!("Aligning urban `fare_attributes.txt`");
    align_fare_attributes(&mut urban_fare, &mut fares, TT::Urban)?;
    info!("Writing `fare.json` file");
    fs::write(
        format!("{}/fare.json", ALIGNEMENT_DIR),
        serde_json::to_string(&fares)?,
    )?;

    // fare_rules.txt
    info!("Aligning `fare_rules.txt`");
    let mut fare_rules: Vec<KdiFareRule> = Vec::new();

    debug!("Aligning extraurban `fare_rules.txt`");
    align_fare_rules(&mut extraurban_fare, &mut fare_rules, TT::ExtraUrban)?;
    debug!("Aligning urban `fare_rules.txt`");
    align_fare_rules(&mut urban_fare, &mut fare_rules, TT::Urban)?;
    info!("Writing `fare_rule.json` file");
    fs::write(
        format!("{}/fare_rule.json", ALIGNEMENT_DIR),
        serde_json::to_string(&fare_rules)?,
    )?;

    // ENUMS
    info!("Writing `stop_enum.json` file");
    fs::write(
        format!("{}/stop_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiStopEnum::VARIANTS }))?,
    )?;

    info!("Writing `supported_enum.json` file");
    fs::write(
        format!("{}/supported_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiSupportedEnum::VARIANTS }))?,
    )?;

    info!("Writing `direction_enum.json` file");
    fs::write(
        format!("{}/direction_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiDirectionEnum::VARIANTS }))?,
    )?;

    info!("Writing `exception_enum.json` file");
    fs::write(
        format!("{}/exception_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiExceptionEnum::VARIANTS }))?,
    )?;

    info!("Writing `transport_enum.json` file");
    fs::write(
        format!("{}/transport_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiTransportEnum::VARIANTS }))?,
    )?;

    info!("Writing `fare_enum.json` file");
    fs::write(
        format!("{}/fare_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiFareEnum::VARIANTS }))?,
    )?;

    info!("Writing `currency_enum.json` file");
    fs::write(
        format!("{}/currency_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiCurrencyEnum::VARIANTS }))?,
    )?;

    info!("Writing `payment_enum.json` file");
    fs::write(
        format!("{}/payment_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiPaymentEnum::VARIANTS }))?,
    )?;

    Ok(())
}

fn to_correct_id(tt: &TT, id: &String) -> String {
    format!("{}_{}", tt, id)
}

fn align_stops<'a, 'b>(
    gtfs: &'a Gtfs,
    stops: &'b mut Vec<KdiStop<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, stop) in &gtfs.stops {
        stops.push(KdiStop {
            id: to_correct_id(&tt, &stop.id),
            zone_id: if stop.zone_id.is_some() {
                Some(format!("ZONE_{}", to_correct_id(&tt, &stop.zone_id.as_ref().unwrap())))
            } else {
                None
            },
            name: &stop.name,
            latitude: stop.latitude.unwrap(),
            longitude: stop.longitude.unwrap(),
            stype: KdiStopEnum::Generic,
            weelchair: KdiSupportedEnum::from(stop.wheelchair_boarding),
        });
    }

    stops.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_routes<'a, 'b>(
    gtfs: &'a Gtfs,
    routes: &'b mut Vec<KdiRoute<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, route) in &gtfs.routes {
        routes.push(KdiRoute {
            id: to_correct_id(&tt, &route.id),
            agency_id: route.agency_id.as_ref().unwrap().to_string(),
            short_name: &route.short_name,
            long_name: &route.long_name,
            transport: KdiTransportEnum::from(route.route_type),
        });
    }

    routes.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_calendar<'a, 'b>(
    gtfs: &'a Gtfs,
    calendars: &'b mut Vec<KdiCalendar>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, calendar) in &gtfs.calendar {
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

fn align_calendar_dates<'a, 'b>(
    gtfs: &'a Gtfs,
    calendars_exception: &'b mut Vec<KdiCalendarException>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, calendar_date) in &gtfs.calendar_dates {
        for cd in calendar_date {
            calendars_exception.push(KdiCalendarException {
                calendar_id: to_correct_id(&tt, &cd.service_id),
                date: cd
                    .date
                    .and_time(NaiveTime::from_hms(0, 0, 0))
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string(),
                exception: KdiExceptionEnum::from(cd.exception_type),
            });
        }
    }

    calendars_exception.sort_by(|a, b| {
        a.calendar_id
            .cmp(&b.calendar_id)
            .then_with(|| a.date.cmp(&b.date))
    });

    Ok(())
}

fn align_trips<'a, 'b>(
    gtfs: &'a Gtfs,
    trips: &'b mut Vec<KdiTrip<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, trip) in &gtfs.trips {
        trips.push(KdiTrip {
            id: to_correct_id(&tt, &trip.id),
            route_id: to_correct_id(&tt, &trip.route_id),
            calendar_id: to_correct_id(&tt, &trip.service_id),
            name: &trip.trip_headsign.as_ref().unwrap(),
            direction: KdiDirectionEnum::from(trip.direction_id.unwrap()),
            weelchair: KdiSupportedEnum::from(trip.wheelchair_accessible),
            bike: KdiSupportedEnum::from(trip.bikes_allowed),
        })
    }

    trips.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_stop_times<'a, 'b>(
    gtfs: &'a Gtfs,
    stop_times: &'b mut Vec<KdiStopTime>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, trip) in &gtfs.trips {
        for stop_time in &trip.stop_times {
            stop_times.push(KdiStopTime {
                trip_id: to_correct_id(&tt, &trip.id),
                stop_id: to_correct_id(&tt, &stop_time.stop.id),
                arrival: if let Some(time) = stop_time.arrival_time {
                    Some(
                        NaiveDate::from_ymd(0, 1, 1 + (time / 86_400))
                            .and_time(NaiveTime::from_num_seconds_from_midnight(time % 86_400, 0))
                            .format("%Y-%m-%dT%H:%M:%S")
                            .to_string(),
                    )
                } else {
                    None
                },
                departure: if let Some(time) = stop_time.departure_time {
                    Some(
                        NaiveDate::from_ymd(0, 1, 1 + (time / 86_400))
                            .and_time(NaiveTime::from_num_seconds_from_midnight(time % 86_400, 0))
                            .format("%Y-%m-%dT%H:%M:%S")
                            .to_string(),
                    )
                } else {
                    None
                },
                sequence: stop_time.stop_sequence as usize,
            })
        }
    }

    stop_times.sort_by(|a, b| {
        a.trip_id
            .cmp(&b.trip_id)
            .then_with(|| a.sequence.cmp(&b.sequence))
    });

    Ok(())
}

fn align_zones<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    zones: &'b mut Vec<KdiZone>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
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
        let mut zone: KdiZone = result?;
        zone.id = format!("ZONE_{}", to_correct_id(&tt, &zone.id));
        zones.push(zone);
    }

    zones.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_fare_attributes<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    fares: &'b mut Vec<KdiFare>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    let mut fare_attributes_string: String = String::new();
    let mut fare_attributes_cartascalare_string: String = String::new();
    let mut fare_attributes_mobile_string: String = String::new();

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano.txt"
        })?
        .read_to_string(&mut fare_attributes_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano_cartascalare.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano_cartascalare.txt"
        })?
        .read_to_string(&mut fare_attributes_cartascalare_string)?;

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_attributes_extraurbano_mobile.txt"
        } else {
            "tariffegtfsurbano/fare_attributes_urbano_mobile.txt"
        })?
        .read_to_string(&mut fare_attributes_mobile_string)?;

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_attributes_string.as_bytes())
        .deserialize()
    {
        let mut fare: KdiFare = result?;
        fare.id = to_correct_id(&tt, &fare.id);
        fare.ftype = KdiFareEnum::Cash;
        fares.push(fare);
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_attributes_cartascalare_string.as_bytes())
        .deserialize()
    {
        let mut fare: KdiFare = result?;
        fare.id = to_correct_id(&tt, &fare.id);
        fare.ftype = KdiFareEnum::Cartascalare;
        fares.push(fare);
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_attributes_mobile_string.as_bytes())
        .deserialize()
    {
        let mut fare: KdiFare = result?;
        fare.id = to_correct_id(&tt, &fare.id);
        fare.ftype = KdiFareEnum::Mobile;
        fares.push(fare);
    }

    fares.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_fare_rules<'a, 'b>(
    archive: &'a mut ZipArchive<File>,
    fare_rules: &'b mut Vec<KdiFareRule>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    let mut fare_rules_string: String = String::new();
    let mut fare_rules_cartascalare_string: String = String::new();
    let mut fare_rules_mobile_string: String = String::new();

    archive
        .by_name(if matches!(tt, TT::ExtraUrban) {
            "tariffegtfsextraurbano/fare_rules_extraurbano.txt"
        } else {
            "tariffegtfsurbano/fare_rules_urbano.txt"
        })?
        .read_to_string(&mut fare_rules_string)?;

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
        .from_reader(fare_rules_string.as_bytes())
        .deserialize()
    {
        let mut fare_rule: KdiFareRule = result?;
        fare_rule.fare_id = to_correct_id(&tt, &fare_rule.fare_id);
        if fare_rule.origin_id.is_some() {
            fare_rule.origin_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin_id.unwrap())));
        }
        if fare_rule.destination_id.is_some() {
            fare_rule.destination_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination_id.unwrap())));
        }
        fare_rules.push(fare_rule);
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_rules_cartascalare_string.as_bytes())
        .deserialize()
    {
        let mut fare_rule: KdiFareRule = result?;
        fare_rule.fare_id = to_correct_id(&tt, &fare_rule.fare_id);
        if fare_rule.origin_id.is_some() {
            fare_rule.origin_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin_id.unwrap())));
        }
        if fare_rule.destination_id.is_some() {
            fare_rule.destination_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination_id.unwrap())));
        }
        fare_rules.push(fare_rule);
    }

    for result in ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(fare_rules_mobile_string.as_bytes())
        .deserialize()
    {
        let mut fare_rule: KdiFareRule = result?;
        fare_rule.fare_id = to_correct_id(&tt, &fare_rule.fare_id);
        if fare_rule.origin_id.is_some() {
            fare_rule.origin_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.origin_id.unwrap())));
        }
        if fare_rule.destination_id.is_some() {
            fare_rule.destination_id = Some(format!("ZONE_{}", to_correct_id(&tt, &fare_rule.destination_id.unwrap())));
        }
        fare_rules.push(fare_rule);
    }

    fare_rules.sort_by(|a, b| a.fare_id.cmp(&b.fare_id));

    Ok(())
}
