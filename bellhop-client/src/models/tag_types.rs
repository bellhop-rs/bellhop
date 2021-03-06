/*
 * Bellhop
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagTypes {
    #[serde(rename = "items")]
    pub items: Vec<::models::TagType>,
    #[serde(rename = "pages")]
    pub pages: ::models::Pages,
}

impl TagTypes {
    pub fn new(items: Vec<::models::TagType>, pages: ::models::Pages) -> TagTypes {
        TagTypes {
            items: items,
            pages: pages,
        }
    }
}
