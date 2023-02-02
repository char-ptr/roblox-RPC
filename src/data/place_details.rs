use serde::Deserialize;
use serde::Serialize;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub place_id: i64,
    pub name: String,
    pub description: String,
    pub source_name: String,
    pub source_description: String,
    pub url: String,
    pub builder: String,
    pub builder_id: i64,
    pub has_verified_badge: bool,
    pub is_playable: bool,
    pub reason_prohibited: String,
    pub universe_id: i64,
    pub universe_root_place_id: i64,
    pub price: i64,
    pub image_token: String,
}
