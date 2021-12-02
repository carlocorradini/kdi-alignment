mod kdigtfs;

use chrono::NaiveTime;
use env_logger::{Builder, Target};
use gtfs_structures::Gtfs;
use kdigtfs::{KdiDirectionEnum, KdiTrip};
use log::{debug, info, LevelFilter};
use serde_json::json;
use std::fmt::Display;
use std::fs;
use std::{error::Error};
use strum::VariantNames;

use crate::kdigtfs::{
    KdiAgency, KdiCalendar, KdiCalendarException, KdiExceptionEnum, KdiRoute, KdiStop, KdiStopEnum,
    KdiSupportedEnum, KdiTransportEnum,
};

const ALIGNEMENT_DIR: &'static str = "./alignment";
const EXTRAURBAN_FILE: &'static str = "./data/extraurban.zip";
const URBAN_FILE: &'static str = "./data/urban.zip";

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

    // StopEnum
    // WeelchairEnum
    // BikeEnum

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

    Ok(())
}

fn to_correct_id(tt: &TT, id: &String) -> String {
    format!("{}{}", tt, id)
}

fn align_stops<'a, 'b>(
    gtfs: &'a Gtfs,
    stops: &'b mut Vec<KdiStop<'a>>,
    tt: TT,
) -> Result<(), Box<dyn Error>> {
    for (_, stop) in &gtfs.stops {
        stops.push(KdiStop {
            id: to_correct_id(&tt, &stop.id),
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
