/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// Application : Application index and its parameters

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Application {
    /// \\[appidx\\] application index.
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(rename = "params")]
    pub params: Box<crate::models::ApplicationParams>,
}

impl Application {
    /// Application index and its parameters
    pub fn new(id: u64, params: crate::models::ApplicationParams) -> Application {
        Application {
            id,
            params: Box::new(params),
        }
    }
}