mod kdi;

use env_logger::{Builder, Target};
use gtfs_structures::Gtfs;
use kdi::enums::KdiFareEnum;
use log::{debug, info, LevelFilter};
use serde_json::json;
use std::error::Error;
use std::fs::{self, File};
use strum::VariantNames;
use zip::ZipArchive;

use crate::kdi::align::{self, TT};
use crate::kdi::enums::{
    KdiCurrencyEnum, KdiDirectionEnum, KdiExceptionEnum, KdiLocationTypeEnum, KdiPaymentEnum,
    KdiSupportedEnum, KdiTransportEnum,
};
use crate::kdi::structs::{KdiAgency, KdiFare, KdiFareRule, KdiLocation, KdiRoute, KdiTrip};

const ALIGNEMENT_DIR: & str = "./alignment";
const EXTRAURBAN_FILE: & str = "./data/extraurban.zip";
const URBAN_FILE: & str = "./data/urban.zip";
const EXTRAURBAN_FARE_FILE: & str = "./data/extraurban_fare.zip";
const URBAN_FARE_FILE: & str = "./data/urban_fare.zip";

fn main() -> Result<(), Box<dyn Error>> {
    // --- LOGGER
    // - Initialize logger
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        .init();

    // --- DIRECTORY TREE
    // - Remove and recreate alignment directory
    info!("Removing `{}` directory", ALIGNEMENT_DIR);
    fs::remove_dir_all(ALIGNEMENT_DIR).ok();
    info!("Creating `{}` directory", ALIGNEMENT_DIR);
    fs::create_dir(ALIGNEMENT_DIR)?;

    // --- DATA FILES
    // - Read `GTFS` files
    info!("Reading `{}`", EXTRAURBAN_FILE);
    let gtfs_extraurban = Gtfs::new(EXTRAURBAN_FILE)?;
    info!("Reading `{}`", URBAN_FILE);
    let gtfs_urban = Gtfs::new(URBAN_FILE)?;
    // - Read `FARE` files
    info!("Reading `{}`", EXTRAURBAN_FARE_FILE);
    let mut extraurban_fare = ZipArchive::new(File::open(&EXTRAURBAN_FARE_FILE)?)?;
    info!("Reading `{}`", URBAN_FARE_FILE);
    let mut urban_fare = ZipArchive::new(File::open(&URBAN_FARE_FILE)?)?;

    // --- COMMON
    // - Location
    let mut locations: Vec<KdiLocation> = Vec::new();
    // Zone
    info!("Aligning `Common::Location::Zone`");
    debug!("Aligning extraurban `Common::Location::Zone`");
    align::align_location_zone(&mut extraurban_fare, &mut locations, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Location::Zone`");
    align::align_location_zone(&mut urban_fare, &mut locations, TT::Urban)?;
    info!("Writing `locations.json` file");
    fs::write(
        format!("{}/locations.json", ALIGNEMENT_DIR),
        serde_json::to_string(&locations)?,
    )?;
    // - Fare
    let mut fares: Vec<KdiFare> = Vec::new();
    info!("Aligning `Common::Fare`");
    debug!("Aligning extraurban `Common::Fare`");
    align::align_fare(&mut extraurban_fare, &mut fares, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Fare`");
    align::align_fare(&mut urban_fare, &mut fares, TT::Urban)?;
    info!("Writing `fares.json` file");
    fs::write(
        format!("{}/fares.json", ALIGNEMENT_DIR),
        serde_json::to_string(&fares)?,
    )?;
    // - Trip
    let mut trips: Vec<KdiTrip> = Vec::new();
    info!("Aligning `Common::Trip`");
    debug!("Aligning extraurban `Common::Trip`");
    align::align_trip(&gtfs_extraurban, &mut trips, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Trip`");
    align::align_trip(&gtfs_urban, &mut trips, TT::Urban)?;
    info!("Writing `trips.json` file");
    fs::write(
        format!("{}/trips.json", ALIGNEMENT_DIR),
        serde_json::to_string(&trips)?,
    )?;
    // - Agency
    let mut agencies: Vec<KdiAgency> = Vec::new();
    info!("Aligning `Common::Agency`");
    assert!(gtfs_extraurban.agencies.len() == 1);
    assert!(gtfs_urban.agencies.len() == 1);
    let agency_email = "info@trentinotrasporti.it".to_string();
    {
        let gtfs_agency = gtfs_extraurban.agencies.first().unwrap();
        agencies.push(KdiAgency {
            id: &gtfs_agency.id.as_ref().unwrap(),
            name: &gtfs_agency.name,
            email: &agency_email,
            phone: &gtfs_agency.phone.as_ref().unwrap(),
            url: &gtfs_agency.url,
        });
    }
    info!("Writing `agencies.json` file");
    fs::write(
        format!("{}/agencies.json", ALIGNEMENT_DIR),
        serde_json::to_string(&agencies)?,
    )?;
    // - Route
    let mut routes: Vec<KdiRoute> = Vec::new();
    info!("Aligning `Common::Route`");
    debug!("Aligning extraurban `Common::Route`");
    align::align_route(&gtfs_extraurban, &mut routes, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Route`");
    align::align_route(&gtfs_urban, &mut routes, TT::Urban)?;
    info!("Writing `routes.json` file");
    fs::write(
        format!("{}/routes.json", ALIGNEMENT_DIR),
        serde_json::to_string(&routes)?,
    )?;

    // --- CORE
    // - FareRule
    info!("Aligning `Core:FareRule`");
    let mut fare_rules: Vec<KdiFareRule> = Vec::new();
    debug!("Aligning extraurban `Core:FareRule`");
    align::align_fare_rule(&mut extraurban_fare, &mut fare_rules, TT::ExtraUrban)?;
    debug!("Aligning urban `Core:FareRule`");
    align::align_fare_rule(&mut urban_fare, &mut fare_rules, TT::Urban)?;
    info!("Writing `fare_rules.json` file");
    fs::write(
        format!("{}/fare_rules.json", ALIGNEMENT_DIR),
        serde_json::to_string(&fare_rules)?,
    )?;

    // --- CONTEXTUAL
    info!("Aligning `Contextual::*`");
    // - LocationTypeEnum
    info!("Writing `location_type_enum.json` file");
    fs::write(
        format!("{}/location_type_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiLocationTypeEnum::VARIANTS }))?,
    )?;
    // - PaymentEnum
    info!("Writing `payment_enum.json` file");
    fs::write(
        format!("{}/payment_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiPaymentEnum::VARIANTS }))?,
    )?;
    // - CurrencyEnum
    info!("Writing `currency_enum.json` file");
    fs::write(
        format!("{}/currency_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiCurrencyEnum::VARIANTS }))?,
    )?;
    // - FareEnum
    info!("Writing `fare_enum.json` file");
    fs::write(
        format!("{}/fare_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiFareEnum::VARIANTS }))?,
    )?;
    // - SupportedEnum
    info!("Writing `supported_enum.json` file");
    fs::write(
        format!("{}/supported_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiSupportedEnum::VARIANTS }))?,
    )?;
    // - DirectionEnum
    info!("Writing `direction_enum.json` file");
    fs::write(
        format!("{}/direction_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiDirectionEnum::VARIANTS }))?,
    )?;
    // - ExceptionEnum
    info!("Writing `exception_enum.json` file");
    fs::write(
        format!("{}/exception_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiExceptionEnum::VARIANTS }))?,
    )?;
    // - TransportEnum
    info!("Writing `transport_enum.json` file");
    fs::write(
        format!("{}/transport_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiTransportEnum::VARIANTS }))?,
    )?;

    /*

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

    let mut a: HashMap<&String, &KdiTransportEnum> = HashMap::new();

    for stop in &stops {
        println!("Evaluating Stop {}", stop.id);
        let stop_times_filtered: Vec<&KdiStopTime> = stop_times
            .iter()
            .filter(|st| st.stop_id == stop.id)
            .collect();
        let trips_filtered: Vec<&KdiTrip> = trips
            .iter()
            .filter(|t| stop_times_filtered.iter().any(|&st| st.trip_id == t.id))
            .collect();
        let routes_filtered: Vec<&KdiRoute> = routes
            .iter()
            .filter(|r| trips_filtered.iter().any(|&t| t.route_id == r.id))
            .collect();

        for route in &routes_filtered {
            if !a.contains_key(&stop.id) {
                a.insert(&stop.id, &route.transport);
            } else if a.get(&stop.id).unwrap().eq(&&route.transport) {
                // OK
            } else {
                panic!(
                    "Found Stop {} having transport {:?} and {:?}",
                    stop.id,
                    a.get(&stop.id).unwrap(),
                    route.transport
                );
            }
        }
    }
    */

    Ok(())
}

/*





fn align_stops<'a, 'b>(
    gtfs: &'a Gtfs,
    stops: &'b mut Vec<KdiStop<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, stop) in &gtfs.stops {
        stops.push(KdiStop {
            id: to_correct_id(&tt, &stop.id),
            zone_id: if stop.zone_id.is_some() {
                Some(format!(
                    "ZONE_{}",
                    to_correct_id(&tt, &stop.zone_id.as_ref().unwrap())
                ))
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

*/
