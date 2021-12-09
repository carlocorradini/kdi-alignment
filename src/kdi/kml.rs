use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename(deserialize = "kml"))]
pub struct Kml {
    #[serde(rename(deserialize = "Document"))]
    pub document: Document,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Document {
    #[serde(rename(deserialize = "Folder"))]
    pub folder: Folder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    pub name: String,
    #[serde(rename = "Placemark")]
    pub placemarks: Vec<Placemark>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Placemark {
    #[serde(rename = "Point")]
    pub point: Point,
    #[serde(rename = "ExtendedData")]
    pub extended_data: ExtendedData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Point {
    pub coordinates: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExtendedData {
    #[serde(rename = "SchemaData")]
    pub schema_data: SchemaData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SchemaData {
    #[serde(rename = "SimpleData")]
    pub simple_datas: Vec<SimpleData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SimpleData {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}
