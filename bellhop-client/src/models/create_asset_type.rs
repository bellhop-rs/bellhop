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
pub struct CreateAssetType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "plural_name")]
    pub plural_name: String,
}

impl CreateAssetType {
    pub fn new(name: String, plural_name: String) -> CreateAssetType {
        CreateAssetType {
            name: name,
            plural_name: plural_name,
        }
    }
}