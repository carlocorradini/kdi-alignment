use serde::{Deserialize, Serialize};

use super::enums::{
    KdiCurrencyEnum, KdiDirectionEnum, KdiExceptionEnum, KdiFareEnum, KdiLocationTypeEnum,
    KdiPaymentEnum, KdiSupportedEnum, KdiTransportEnum,
};

// Common
#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Location"))]
pub struct KdiLocation {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "Fare"))]
pub struct KdiFare {
    #[serde(rename(deserialize = "FARE_ID"))]
    pub id: String,
    #[serde(rename(deserialize = "PRICE"))]
    pub price: f64,
    #[serde(rename(deserialize = "CURRENCY_TYPE"))]
    pub currency: KdiCurrencyEnum,
    #[serde(skip_deserializing)]
    #[serde(rename(serialize = "type"))]
    pub ftype: KdiFareEnum,
    #[serde(rename(deserialize = "PAYMENT_METHOD"))]
    pub payment: KdiPaymentEnum,
    #[serde(rename(deserialize = "TRANSFER_DURATION"))]
    pub duration: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Trip"))]
pub struct KdiTrip<'a> {
    pub id: String,
    #[serde(rename(serialize = "routeId"))]
    pub route_id: String,
    #[serde(rename(serialize = "calendarId"))]
    pub calendar_id: String,
    pub name: &'a String,
    pub direction: KdiDirectionEnum,
    pub weelchair: KdiSupportedEnum,
    pub bike: KdiSupportedEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Agency"))]
pub struct KdiAgency<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub email: &'a String,
    pub phone: &'a String,
    pub url: &'a String,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Route"))]
pub struct KdiRoute<'a> {
    pub id: String,
    #[serde(rename(serialize = "agencyId"))]
    pub agency_id: &'a String,
    #[serde(rename(serialize = "shortName"))]
    pub short_name: &'a String,
    #[serde(rename(serialize = "longName"))]
    pub long_name: &'a String,
    pub transport: KdiTransportEnum,
}

// Core
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "FareRule"))]
pub struct KdiFareRule {
    #[serde(rename(serialize = "fareId", deserialize = "FARE_ID"))]
    pub fare_id: String,
    #[serde(
        rename(serialize = "origin", deserialize = "ORIGIN_ID"),
        default = "kdi_fare_rule_default"
    )]
    pub origin: String,
    #[serde(
        rename(serialize = "destination", deserialize = "DESTINATION_ID"),
        default = "kdi_fare_rule_default"
    )]
    pub destination: String,
}

fn kdi_fare_rule_default() -> String {
    "0001".to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "LocationType"))]
pub struct KdiLocationType {
    #[serde(rename(serialize = "locationId"))]
    pub location_id: String,
    #[serde(rename(serialize = "type"))]
    pub ltype: KdiLocationTypeEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "ParkingStop"))]
pub struct KdiParkingStop {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub address: String,
    #[serde(rename(serialize = "totalSlots"))]
    pub total_slots: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "BikeSharingStop"))]
pub struct KdiBikeSharingStop {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub address: String,
    #[serde(rename(serialize = "totalSlots"))]
    pub total_slots: usize,
    #[serde(rename(serialize = "freeSlots"))]
    pub free_slots: usize,
    pub bikes: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "PublicTransportStop"))]
pub struct KdiPublicTransportStop {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub zone: Option<String>,
    pub weelchair: KdiSupportedEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "StopTime"))]
pub struct KdiStopTime {
    #[serde(rename(serialize = "tripId"))]
    pub trip_id: String,
    #[serde(rename(serialize = "publicTransportStopId"))]
    pub public_transport_stop_id: String,
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub sequence: usize,
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
