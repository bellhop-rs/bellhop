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
pub struct AssetType {
  #[serde(rename = "id")]
  id: i32,
  #[serde(rename = "plural_name")]
  plural_name: String,
  #[serde(rename = "name")]
  name: String
}

impl AssetType {
  pub fn new(id: i32, plural_name: String, name: String) -> AssetType {
    AssetType {
      id: id,
      plural_name: plural_name,
      name: name
    }
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i32) -> AssetType {
    self.id = id;
    self
  }

  pub fn id(&self) -> &i32 {
    &self.id
  }


  pub fn set_plural_name(&mut self, plural_name: String) {
    self.plural_name = plural_name;
  }

  pub fn with_plural_name(mut self, plural_name: String) -> AssetType {
    self.plural_name = plural_name;
    self
  }

  pub fn plural_name(&self) -> &String {
    &self.plural_name
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AssetType {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



