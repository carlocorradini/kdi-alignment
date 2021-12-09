use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BikeSharing {
    pub id: String,
    pub name: String,
    pub address: String,
    pub bikes: usize,
    pub slots: usize,
    #[serde(rename(deserialize = "totalSlots"))]
    pub total_slots: usize,
    pub position: Vec<f64>,
}
