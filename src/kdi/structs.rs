use serde::{Deserialize, Serialize, Serializer};

use super::enums::{
    KdiCurrencyEnum, KdiDirectionEnum, KdiExceptionEnum, KdiFareEnum, KdiParkingStopEnum,
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

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "CalendarException"))]
pub struct KdiCalendarException {
    #[serde(rename(serialize = "calendarId"))]
    pub calendar: String,
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

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Agency"))]
pub struct KdiAgency<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub url: &'a str,
}

// Core
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "FareRule"))]
pub struct KdiFareRule {
    #[serde(rename(deserialize = "FARE_ID"))]
    pub fare: String,
    #[serde(rename(deserialize = "ORIGIN_ID"), default = "kdi_fare_rule_default")]
    pub origin: String,
    #[serde(
        rename(deserialize = "DESTINATION_ID"),
        default = "kdi_fare_rule_default"
    )]
    pub destination: String,
}

fn kdi_fare_rule_default() -> String {
    "0001".to_string()
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
    #[serde(rename(serialize = "type"), skip_deserializing)]
    pub ftype: KdiFareEnum,
    #[serde(rename(deserialize = "PAYMENT_METHOD"))]
    pub payment: KdiPaymentEnum,
    #[serde(rename(deserialize = "TRANSFER_DURATION"))]
    pub duration: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "ParkingStop"))]
pub struct KdiParkingStop {
    pub location: String,
    #[serde(rename(serialize = "type"))]
    pub ptype: KdiParkingStopEnum,
    pub address: String,
    #[serde(rename(serialize = "totalSlots"))]
    pub total_slots: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "BikeSharingStop"))]
pub struct KdiBikeSharingStop {
    pub location: String,
    #[serde(rename(serialize = "type"))]
    pub ptype: KdiParkingStopEnum,
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
    pub location: String,
    pub zone: Option<String>,
    #[serde(serialize_with = "ptype_serialization")]
    pub ptype: Vec<KdiTransportEnum>,
    pub weelchair: KdiSupportedEnum,
}

fn ptype_serialization<S>(t: &Vec<KdiTransportEnum>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{:?}", t.iter().map(|transport| format!("{:?}", transport)).collect::<Vec<String>>()))
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "StopTime"))]
pub struct KdiStopTime {
    pub trip: String,
    pub stop: String,
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub sequence: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Trip"))]
pub struct KdiTrip<'a> {
    pub id: String,
    pub route: String,
    pub calendar: String,
    pub name: &'a str,
    pub direction: KdiDirectionEnum,
    pub weelchair: KdiSupportedEnum,
    pub bike: KdiSupportedEnum,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "Route"))]
pub struct KdiRoute<'a> {
    pub id: String,
    pub agency: &'a str,
    #[serde(rename(serialize = "shortName"))]
    pub short_name: &'a str,
    #[serde(rename(serialize = "longName"))]
    pub long_name: &'a str,
    pub transport: KdiTransportEnum,
}
