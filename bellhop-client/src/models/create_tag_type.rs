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
pub struct CreateTagType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "detail_only")]
    pub detail_only: bool,
    #[serde(rename = "rightness")]
    pub rightness: i32,
}

impl CreateTagType {
    pub fn new(name: String, detail_only: bool, rightness: i32) -> CreateTagType {
        CreateTagType {
            name: name,
            detail_only: detail_only,
            rightness: rightness,
        }
    }
}