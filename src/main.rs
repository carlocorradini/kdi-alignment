mod kdi;

use env_logger::{Builder, Target};
use gtfs_structures::Gtfs;
use kdi::enums::KdiFareEnum;
use log::{debug, info, LevelFilter};
use serde_json::json;
use serde_xml_rs::{self};
use std::error::Error;
use std::fs::{self, File};
use strum::VariantNames;
use zip::ZipArchive;

use crate::kdi::align::{self, TT};
use crate::kdi::enums::{
    KdiCurrencyEnum, KdiDirectionEnum, KdiExceptionEnum, KdiParkingStopEnum, KdiPaymentEnum,
    KdiSupportedEnum, KdiTransportEnum,
};
use crate::kdi::kml::Kml;
use crate::kdi::structs::{
    KdiAgency, KdiCalendar, KdiCalendarException, KdiFare, KdiFareRule, KdiLocation,
    KdiPublicTransportStop, KdiRoute, KdiStopTime, KdiTrip, KdiParkingStop,
};

const ALIGNEMENT_DIR: &str = "./alignment";
const EXTRAURBAN_FILE: &str = "./data/extraurban.zip";
const URBAN_FILE: &str = "./data/urban.zip";
const EXTRAURBAN_FARE_FILE: &str = "./data/extraurban_fare.zip";
const URBAN_FARE_FILE: &str = "./data/urban_fare.zip";
const CAR_SHARING_FILE: &str = "./data/car_sharing.kml";
const CENTRO_IN_BICI_FILE: &str = "./data/centro_in_bici.kml";
const PARCHEGGIO_PROTETTO_BICICLETTE: &str = "./data/parcheggio_protetto_biciclette.kml";
const TAXI_FILE: &str = "./data/taxi.kml";

fn main() -> Result<(), Box<dyn Error>> {
    // --- LOGGER
    // - Initialize logger
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        .filter_module("serde_xml_rs::de", LevelFilter::Off)
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
    // - Read `KML` files
    info!("Reading `{}`", CAR_SHARING_FILE);
    let car_sharing: Kml = serde_xml_rs::from_str(&fs::read_to_string(CAR_SHARING_FILE)?)?;
    info!("Reading `{}`", CENTRO_IN_BICI_FILE);
    let centro_in_bici: Kml = serde_xml_rs::from_str(&fs::read_to_string(CENTRO_IN_BICI_FILE)?)?;
    info!("Reading `{}`", PARCHEGGIO_PROTETTO_BICICLETTE);
    let parcheggio_protetto_biciclette: Kml =
        serde_xml_rs::from_str(&fs::read_to_string(PARCHEGGIO_PROTETTO_BICICLETTE)?)?;
    info!("Reading `{}`", TAXI_FILE);
    let taxi: Kml = serde_xml_rs::from_str(&fs::read_to_string(TAXI_FILE)?)?;

    // --- COMMON
    // - Location
    info!("Aligning `Common::Location`");
    let mut locations: Vec<KdiLocation> = Vec::new();
    // Zone
    debug!("Aligning extraurban `Common::Location::Zone`");
    align::align_location_zone(&mut extraurban_fare, &mut locations, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Location::Zone`");
    align::align_location_zone(&mut urban_fare, &mut locations, TT::Urban)?;
    // PublicTransportStop
    debug!("Aligning extraurban `Common::Location::PublicTransportStop`");
    align::align_location_public_transport_stop(&gtfs_extraurban, &mut locations, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Location::PublicTransportStop`");
    align::align_location_public_transport_stop(&gtfs_urban, &mut locations, TT::Urban)?;
    // CarSharing
    debug!("Aligning `Common::Location::CarSharing`");
    align::align_location_car_sharing(&car_sharing, &mut locations)?;
    // CentroInBici
    debug!("Aligning `Common::Location::CentroInBici`");
    align::align_location_centro_in_bici(&centro_in_bici, &mut locations)?;
    // ParcheggioProtettoBiciclette
    debug!("Aligning `Common::Location::ParcheggioProtettoBiciclette`");
    align::align_location_parcheggio_protetto_biciclette(
        &parcheggio_protetto_biciclette,
        &mut locations,
    )?;
    // Taxi
    debug!("Aligning `Common::Location::Taxi`");
    align::align_location_taxi(&taxi, &mut locations)?;
    info!("Writing `locations.json` file");
    fs::write(
        format!("{}/locations.json", ALIGNEMENT_DIR),
        serde_json::to_string(&locations)?,
    )?;
    // - CalendarException
    let mut calendar_exceptions: Vec<KdiCalendarException> = Vec::new();
    info!("Aligning `Common::CalendarException`");
    debug!("Aligning extraurban `Common::CalendarException`");
    align::align_calendar_exception(&gtfs_extraurban, &mut calendar_exceptions, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::CalendarException`");
    align::align_calendar_exception(&gtfs_urban, &mut calendar_exceptions, TT::Urban)?;
    info!("Writing `calendar_exceptions.json` file");
    fs::write(
        format!("{}/calendar_exceptions.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendar_exceptions)?,
    )?;
    // - Calendar
    let mut calendars: Vec<KdiCalendar> = Vec::new();
    info!("Aligning `Common::Calendar`");
    debug!("Aligning extraurban `Common::Calendar`");
    align::align_calendar(&gtfs_extraurban, &mut calendars, TT::ExtraUrban)?;
    debug!("Aligning urban `Common::Calendar`");
    align::align_calendar(&gtfs_urban, &mut calendars, TT::Urban)?;
    info!("Writing `calendars.json` file");
    fs::write(
        format!("{}/calendars.json", ALIGNEMENT_DIR),
        serde_json::to_string(&calendars)?,
    )?;
    // - Agency
    let mut agencies: Vec<KdiAgency> = Vec::new();
    info!("Aligning `Common::Agency`");
    assert!(gtfs_extraurban.agencies.len() == 1);
    assert!(gtfs_urban.agencies.len() == 1);
    {
        let gtfs_agency = gtfs_extraurban.agencies.first().unwrap();
        agencies.push(KdiAgency {
            id: gtfs_agency.id.as_ref().unwrap(),
            name: &gtfs_agency.name,
            email: "info@trentinotrasporti.it",
            phone: gtfs_agency.phone.as_ref().unwrap(),
            url: &gtfs_agency.url,
        });
    }
    info!("Writing `agencies.json` file");
    fs::write(
        format!("{}/agencies.json", ALIGNEMENT_DIR),
        serde_json::to_string(&agencies)?,
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
    // - ParkingStop
    info!("Aligning `Core::ParkingStop`");
    let mut parking_stops: Vec<KdiParkingStop> = Vec::new();
    // CarSharing
    debug!("Aligning `Core::ParkingStop::CarSharing`");
    align::align_parking_stop_car_sharing(&car_sharing, &mut parking_stops)?;
    // CentroInBici
    debug!("Aligning `Core::ParkingStop::CentroInBici`");
    align::align_parking_stop_centro_in_bici(&centro_in_bici, &mut parking_stops)?;
    // ParcheggioProtettoBiciclette
    debug!("Aligning `Core::ParkingStop::ParcheggioProtettoBiciclette`");
    align::align_parking_stop_parcheggio_protetto_biciclette(
        &parcheggio_protetto_biciclette,
        &mut parking_stops,
    )?;
    // Taxi
    debug!("Aligning `Core::ParkingStop::Taxi`");
    align::align_parking_stop_taxi(&taxi, &mut parking_stops)?;
    info!("Writing `parking_stops.json` file");
    fs::write(
        format!("{}/parking_stops.json", ALIGNEMENT_DIR),
        serde_json::to_string(&parking_stops)?,
    )?;
    // - Fare
    let mut fares: Vec<KdiFare> = Vec::new();
    info!("Aligning `Core::Fare`");
    debug!("Aligning extraurban `Core::Fare`");
    align::align_fare(&mut extraurban_fare, &mut fares, TT::ExtraUrban)?;
    debug!("Aligning urban `Core::Fare`");
    align::align_fare(&mut urban_fare, &mut fares, TT::Urban)?;
    info!("Writing `fares.json` file");
    fs::write(
        format!("{}/fares.json", ALIGNEMENT_DIR),
        serde_json::to_string(&fares)?,
    )?;
    // - BikeSharingStop
    // TODO
    // - PublicTransportStop
    let mut public_transport_stops: Vec<KdiPublicTransportStop> = Vec::new();
    info!("Aligning `Core::PublicTransportStop`");
    debug!("Aligning extraurban `Core::PublicTransportStop`");
    align::align_public_transport_stop(
        &gtfs_extraurban,
        &mut public_transport_stops,
        TT::ExtraUrban,
    )?;
    debug!("Aligning urban `Core::PublicTransportStop`");
    align::align_public_transport_stop(&gtfs_urban, &mut public_transport_stops, TT::Urban)?;
    info!("Writing `public_transport_stops.json` file");
    fs::write(
        format!("{}/public_transport_stops.json", ALIGNEMENT_DIR),
        serde_json::to_string(&public_transport_stops)?,
    )?;
    // - StopTime
    let mut stop_times: Vec<KdiStopTime> = Vec::new();
    info!("Aligning `Core::StopTime`");
    debug!("Aligning extraurban `Core::StopTime`");
    align::align_stop_time(&gtfs_extraurban, &mut stop_times, TT::ExtraUrban)?;
    debug!("Aligning urban `Core::StopTime`");
    align::align_stop_time(&gtfs_urban, &mut stop_times, TT::Urban)?;
    info!("Writing `stop_times.json` file");
    fs::write(
        format!("{}/stop_times.json", ALIGNEMENT_DIR),
        serde_json::to_string(&stop_times)?,
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

    // --- CONTEXTUAL
    info!("Aligning `Contextual::*`");
    // - PaymentEnum
    info!("Writing `payment_enum.json` file");
    fs::write(
        format!("{}/payment_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiPaymentEnum::VARIANTS }))?,
    )?;
    // - ParkingStopEnum
    info!("Writing `parking_stop_enum.json` file");
    fs::write(
        format!("{}/parking_stop_enum.json", ALIGNEMENT_DIR),
        serde_json::to_string(&json!({ "value": KdiParkingStopEnum::VARIANTS }))?,
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
