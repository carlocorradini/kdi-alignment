mod kdigtfs;

use chrono::{NaiveTime};
use env_logger::{Builder, Target};
use gtfs_structures::Gtfs;
use log::{debug, info, LevelFilter};
use std::error::Error;
use std::fs;

use crate::kdigtfs::{
    KdiAgency, KdiCalendar, KdiRoute, KdiStop, KdiStopEnum, KdiTransportEnum, KdiWeelchairEnum,
};

const ALIGNEMENT_DIR: &'static str = "./alignment";
const EXTRAURBAN_FILE: &'static str = "./data/extraurban.zip";
const URBAN_FILE: &'static str = "./data/urban.zip";

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

    // agency.txt
    info!("Aligning `agency.txt`");
    let agency: KdiAgency;

    assert!(gtfs_extraurban.agencies.len() == 1);
    assert!(gtfs_urban.agencies.len() == 1);
    {
        let gtfs_agency = gtfs_extraurban.agencies.first().unwrap();
        agency = KdiAgency {
            id: 1,
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
    for (_, stop) in &gtfs_extraurban.stops {
        stops.push(KdiStop {
            id: stop.id.parse::<usize>()?,
            name: &stop.name,
            latitude: stop.latitude.unwrap(),
            longitude: stop.longitude.unwrap(),
            stype: KdiStopEnum::Generic,
            weelchair: KdiWeelchairEnum::from(stop.wheelchair_boarding),
        });
    }
    stops.sort_by(|a, b| a.id.cmp(&b.id));

    debug!("Aligning urban `stops.txt`");
    let last_stop_id: usize = stops.last().unwrap().id;
    for (_, stop) in &gtfs_urban.stops {
        stops.push(KdiStop {
            id: last_stop_id + stop.id.parse::<usize>()?,
            name: &stop.name,
            latitude: stop.latitude.unwrap(),
            longitude: stop.longitude.unwrap(),
            stype: KdiStopEnum::Generic,
            weelchair: KdiWeelchairEnum::from(stop.wheelchair_boarding),
        });
    }
    stops.sort_by(|a, b| a.id.cmp(&b.id));

    info!("Writing `stops.json` file");
    fs::write(
        format!("{}/stops.json", ALIGNEMENT_DIR),
        serde_json::to_string(&stops)?,
    )?;

    // routes.txt
    info!("Aligning `routes.txt`");
    let mut routes: Vec<KdiRoute> = Vec::new();

    debug!("Aligning extraurban `routes.txt`");
    for (_, route) in &gtfs_extraurban.routes {
        routes.push(KdiRoute {
            id: route.id.parse::<usize>()?,
            agency_id: 1,
            short_name: &route.short_name,
            long_name: &route.long_name,
            transport: KdiTransportEnum::from(route.route_type),
        })
    }
    routes.sort_by(|a, b| a.id.cmp(&b.id));

    debug!("Aligning urban `routes.txt`");
    let last_route_id: usize = routes.last().unwrap().id;
    for (_, route) in &gtfs_urban.routes {
        routes.push(KdiRoute {
            id: last_route_id + route.id.parse::<usize>()?,
            agency_id: 1,
            short_name: &route.short_name,
            long_name: &route.long_name,
            transport: KdiTransportEnum::from(route.route_type),
        })
    }
    routes.sort_by(|a, b| a.id.cmp(&b.id));

    info!("Writing `routes.json` file");
    fs::write(
        format!("{}/routes.json", ALIGNEMENT_DIR),
        serde_json::to_string(&routes)?,
    )?;

    // calendar.txt
    info!("Aligning `calendar.txt`");
    let mut calendars: Vec<KdiCalendar> = Vec::new();

    debug!("Aligning extraurban `calendar.txt`");
    for (_, calendar) in &gtfs_extraurban.calendar {
        calendars.push(KdiCalendar {
            id: calendar.id.parse::<usize>()?,
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
        })
    }
    calendars.sort_by(|a, b| a.id.cmp(&b.id));

    debug!("Aligning urban `calendar.txt`");
    let last_calendar_id: usize = calendars.last().unwrap().id;
    for (_, calendar) in &gtfs_urban.calendar {
        calendars.push(KdiCalendar {
            id: last_calendar_id + calendar.id.parse::<usize>()?,
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
        })
    }
    calendars.sort_by(|a, b| a.id.cmp(&b.id));

    info!("Writing `calendar.json` file");
    fs::write(
        format!("{}/calendar.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars)?,
    )?;

    Ok(())
}
