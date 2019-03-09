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
pub struct CreateLease {
  #[serde(rename = "end_time")]
  end_time: String
}

impl CreateLease {
  pub fn new(end_time: String) -> CreateLease {
    CreateLease {
      end_time: end_time
    }
  }

  pub fn set_end_time(&mut self, end_time: String) {
    self.end_time = end_time;
  }

  pub fn with_end_time(mut self, end_time: String) -> CreateLease {
    self.end_time = end_time;
    self
  }

  pub fn end_time(&self) -> &String {
    &self.end_time
  }


}



