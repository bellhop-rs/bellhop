/*
 * Bellhop
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lease {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "user_id")]
    pub user_id: i32,
    #[serde(rename = "last_notified")]
    pub last_notified: Option<String>,
    #[serde(rename = "end_time")]
    pub end_time: String,
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
}

impl Lease {
    pub fn new(id: i32, user_id: i32, last_notified: Option<String>, end_time: String) -> Lease {
        Lease {
            id: id,
            user_id: user_id,
            last_notified: last_notified,
            end_time: end_time,
            start_time: None,
        }
    }
}
