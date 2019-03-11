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
pub struct Pages {
    #[serde(rename = "next")]
    pub next: Option<String>,
    #[serde(rename = "prev")]
    pub prev: Option<String>,
}

impl Pages {
    pub fn new(next: Option<String>, prev: Option<String>) -> Pages {
        Pages {
            next: next,
            prev: prev,
        }
    }
}