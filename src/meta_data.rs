use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub save_pool: i64,
    pub custom_name: CustomName,
    pub save_time: String,
    pub version: String,
    pub saved_state_type_name: String,
    pub day: i64,
    pub seconds_played: f64,
    pub current_lvl: i64,
    pub potions_brewed: i64,
    pub legendary_substances_brewed: i64,
    pub clients_served: i64,
    pub popularity: i64,
    pub karma: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomName {
    pub custom_name: String,
    pub parameters: Vec<Value>,
    pub is_name_from_localization: bool,
}
