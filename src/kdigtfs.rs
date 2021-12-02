use gtfs_structures::{Availability, Exception, RouteType};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "StopEnum"))]
pub enum KdiStopEnum {
    Generic,
    BikeSharing,
    BikeParking,
    CarSharing,
    Taxi,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "SupportedEnum"))]
pub enum KdiSupportedEnum {
    Unknown,
    Supported,
    NotSupported,
}

impl From<Availability> for KdiSupportedEnum {
    fn from(availability: Availability) -> Self {
        match availability {
            Availability::Available => KdiSupportedEnum::Supported,
            Availability::NotAvailable => KdiSupportedEnum::NotSupported,
            _ => KdiSupportedEnum::Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "TransportEnum"))]
pub enum KdiTransportEnum {
    Tram,
    Subway,
    Rail,
    Bus,
    Ferry,
    CableTram,
    CableCar,
    Funicular,
    Trolleybus,
    Monorail,
}

impl From<RouteType> for KdiTransportEnum {
    fn from(route_type: RouteType) -> Self {
        match route_type {
            RouteType::Tramway => KdiTransportEnum::Tram,
            RouteType::Subway => KdiTransportEnum::Subway,
            RouteType::Rail => KdiTransportEnum::Rail,
            RouteType::Bus => KdiTransportEnum::Bus,
            RouteType::Ferry => KdiTransportEnum::Ferry,
            RouteType::CableCar => KdiTransportEnum::CableTram,
            RouteType::Gondola => KdiTransportEnum::CableCar,
            RouteType::Funicular => KdiTransportEnum::Funicular,
            RouteType::Other(11) => KdiTransportEnum::Trolleybus,
            RouteType::Other(12) => KdiTransportEnum::Monorail,
            _ => panic!("Unknown route type {:?}", route_type),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "ExceptionEnum"))]
pub enum KdiExceptionEnum {
    Added,
    Removed,
}

impl From<Exception> for KdiExceptionEnum {
    fn from(exception: Exception) -> Self {
        match exception {
            Exception::Added => KdiExceptionEnum::Added,
            Exception::Deleted => KdiExceptionEnum::Removed,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Agency"))]
pub struct KdiAgency<'a> {
    pub id: usize,
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub url: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Stop"))]
pub struct KdiStop<'a> {
    pub id: usize,
    pub name: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename(serialize = "type"))]
    pub stype: KdiStopEnum,
    pub weelchair: KdiSupportedEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Route"))]
pub struct KdiRoute<'a> {
    pub id: usize,
    #[serde(rename(serialize = "agencyId"))]
    pub agency_id: usize,
    #[serde(rename(serialize = "shortName"))]
    pub short_name: &'a str,
    #[serde(rename(serialize = "longName"))]
    pub long_name: &'a str,
    pub transport: KdiTransportEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Calendar"))]
pub struct KdiCalendar {
    pub id: usize,
    #[serde(rename(serialize = "startDate"))]
    pub start_date: String,
    #[serde(rename(serialize = "endDate"))]
    pub end_date: String,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "CalendarException"))]
pub struct KdiCalendarException {
    #[serde(rename(serialize = "calendarId"))]
    pub calendar_id: usize,
    pub date: String,
    pub exception: KdiExceptionEnum,
}
