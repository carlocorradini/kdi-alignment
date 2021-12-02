use gtfs_structures::{Availability, BikesAllowedType, DirectionType, Exception, RouteType};
use serde::Serialize;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
#[serde(rename(serialize = "StopEnum"))]
pub enum KdiStopEnum {
    Generic,
    BikeSharing,
    BikeParking,
    CarSharing,
    Taxi,
}

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
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

impl From<BikesAllowedType> for KdiSupportedEnum {
    fn from(bikes_allowed_type: BikesAllowedType) -> Self {
        match bikes_allowed_type {
            BikesAllowedType::AtLeastOneBike => KdiSupportedEnum::Supported,
            BikesAllowedType::NoBikesAllowed => KdiSupportedEnum::NotSupported,
            _ => KdiSupportedEnum::Unknown,
        }
    }
}

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
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

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
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

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
#[serde(rename(serialize = "DirectionEnum"))]
pub enum KdiDirectionEnum {
    Outbound,
    Inbound,
}

impl From<DirectionType> for KdiDirectionEnum {
    fn from(direction_type: DirectionType) -> Self {
        match direction_type {
            DirectionType::Outbound => KdiDirectionEnum::Outbound,
            DirectionType::Inbound => KdiDirectionEnum::Inbound,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Agency"))]
pub struct KdiAgency<'a> {
    pub id: String,
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub url: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Stop"))]
pub struct KdiStop<'a> {
    pub id: String,
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
    pub id: String,
    #[serde(rename(serialize = "agencyId"))]
    pub agency_id: String,
    #[serde(rename(serialize = "shortName"))]
    pub short_name: &'a str,
    #[serde(rename(serialize = "longName"))]
    pub long_name: &'a str,
    pub transport: KdiTransportEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Calendar"))]
pub struct KdiCalendar {
    pub id: String,
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
    pub calendar_id: String,
    pub date: String,
    pub exception: KdiExceptionEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Trip"))]
pub struct KdiTrip<'a> {
    pub id: String,
    #[serde(rename(serialize = "routeId"))]
    pub route_id: String,
    #[serde(rename(serialize = "calendarId"))]
    pub calendar_id: String,
    pub name: &'a str,
    pub direction: KdiDirectionEnum,
    pub weelchair: KdiSupportedEnum,
    pub bike: KdiSupportedEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "StopTime"))]
pub struct KdiStopTime {
    #[serde(rename(serialize = "tripId"))]
    pub trip_id: String,
    #[serde(rename(serialize = "stopId"))]
    pub stop_id: String,
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub sequence: usize,
}
