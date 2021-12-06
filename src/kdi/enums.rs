use gtfs_structures::{Availability, BikesAllowedType, DirectionType, Exception, RouteType};
use serde::{Serialize, Deserialize};
use serde_repr::Deserialize_repr;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
#[serde(rename(serialize = "LocationTypeEnum"))]
pub enum KdiLocationTypeEnum {
    Zone,
    TrainStop,
    BusStop,
    CableCarStop,
    BikeSharingStop,
    BikeParkingStop,
    CarSharingStop,
    TaxiStop,
}

#[derive(Debug, Serialize, Deserialize_repr, EnumString, EnumVariantNames)]
#[repr(u8)]
#[serde(rename(serialize = "PaymentEnum"))]
pub enum KdiPaymentEnum {
    OnBoard = 0,
    BeforeBoarding = 1,
}

#[derive(Debug, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename(serialize = "CurrencyEnum"))]
pub enum KdiCurrencyEnum {
    EUR,
}

#[derive(Debug, Serialize, EnumString, EnumVariantNames)]
#[serde(rename(serialize = "FareEnum"))]
pub enum KdiFareEnum {
    Cash,
    Cartascalare,
    Mobile,
}

impl Default for KdiFareEnum {
    fn default() -> Self {
        Self::Cash
    }
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

#[derive(Debug, Serialize, EnumString, EnumVariantNames, PartialEq, Eq)]
#[serde(rename(serialize = "TransportEnum"))]
pub enum KdiTransportEnum {
    Rail,
    Bus,
    CableCar,
}

impl From<RouteType> for KdiTransportEnum {
    fn from(route_type: RouteType) -> Self {
        match route_type {
            RouteType::Rail => KdiTransportEnum::Rail,
            RouteType::Bus => KdiTransportEnum::Bus,
            RouteType::CableCar => KdiTransportEnum::CableCar,
            _ => panic!("Unknown route type {:?}", route_type),
        }
    }
}
