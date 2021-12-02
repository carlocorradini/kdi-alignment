mod kdigtfs;

use chrono::NaiveTime;
use env_logger::{Builder, Target};
use gtfs_structures::{Gtfs};
use log::{debug, info, LevelFilter};
use std::fs;
use std::{error::Error};

use crate::kdigtfs::{
    KdiAgency, KdiCalendar, KdiCalendarException, KdiExceptionEnum, KdiRoute, KdiStop, KdiStopEnum,
    KdiTransportEnum, KdiWeelchairEnum,
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
    align_stops(&gtfs_extraurban, &mut stops, None)?;
    debug!("Aligning urban `stops.txt`");
    let last_stop_id: usize = stops.last().unwrap().id;
    align_stops(&gtfs_urban, &mut stops, Some(last_stop_id))?;
    info!("Writing `stop.json` file");
    fs::write(
        format!("{}/stop.json", ALIGNEMENT_DIR),
        serde_json::to_string(&stops)?,
    )?;

    // routes.txt
    info!("Aligning `routes.txt`");
    let mut routes: Vec<KdiRoute> = Vec::new();

    debug!("Aligning extraurban `routes.txt`");
    align_routes(&gtfs_extraurban, &mut routes, None)?;
    debug!("Aligning urban `routes.txt`");
    let last_route_id: usize = routes.last().unwrap().id;
    align_routes(&gtfs_urban, &mut routes, Some(last_route_id))?;
    info!("Writing `route.json` file");
    fs::write(
        format!("{}/route.json", ALIGNEMENT_DIR),
        serde_json::to_string(&routes)?,
    )?;

    // calendar.txt
    info!("Aligning `calendar.txt`");
    let mut calendars: Vec<KdiCalendar> = Vec::new();

    debug!("Aligning extraurban `calendar.txt`");
    align_calendar(&gtfs_extraurban, &mut calendars, None)?;
    debug!("Aligning urban `calendar.txt`");
    let last_calendar_id: usize = calendars.last().unwrap().id;
    align_calendar(&gtfs_urban, &mut calendars, Some(last_calendar_id))?;
    info!("Writing `calendar.json` file");
    fs::write(
        format!("{}/calendar.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars)?,
    )?;

    // calendar_dates.txt
    info!("Aligning `calendar_dates.txt`");
    let mut calendars_exception: Vec<KdiCalendarException> = Vec::new();

    debug!("Aligning extraurban `calendar_dates.txt`");
    align_calendar_dates(&gtfs_extraurban, &mut calendars_exception, None)?;
    debug!("Aligning urban `calendar_dates.txt`");
    align_calendar_dates(
        &gtfs_urban,
        &mut calendars_exception,
        Some(last_calendar_id),
    )?;
    info!("Writing `calendar_exception.json` file");
    fs::write(
        format!("{}/calendar_exception.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars_exception)?,
    )?;

    Ok(())
}

fn align_stops<'a, 'b>(
    gtfs: &'a Gtfs,
    stops: &'b mut Vec<KdiStop<'a>>,
    last_stop_id: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    for (_, stop) in &gtfs.stops {
        stops.push(KdiStop {
            id: stop.id.parse::<usize>()? + last_stop_id.unwrap_or(0),
            name: &stop.name,
            latitude: stop.latitude.unwrap(),
            longitude: stop.longitude.unwrap(),
            stype: KdiStopEnum::Generic,
            weelchair: KdiWeelchairEnum::from(stop.wheelchair_boarding),
        });
    }

    stops.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn align_routes<'a, 'b>(
    gtfs: &'a Gtfs,
    routes: &'b mut Vec<KdiRoute<'a>>,
    last_route_id: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    for (_, route) in &gtfs.routes {
        routes.push(KdiRoute {
            id: route.id.parse::<usize>()? + last_route_id.unwrap_or(0),
            agency_id: 1,
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
    last_calendar_id: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    for (_, calendar) in &gtfs.calendar {
        calendars.push(KdiCalendar {
            id: calendar.id.parse::<usize>()? + last_calendar_id.unwrap_or(0),
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
    last_calendar_id: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    for (_, calendar_date) in &gtfs.calendar_dates {
        for cd in calendar_date {
            calendars_exception.push(KdiCalendarException {
                calendar_id: cd.service_id.parse::<usize>()? + last_calendar_id.unwrap_or(0),
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
