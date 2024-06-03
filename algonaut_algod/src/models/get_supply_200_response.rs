/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// GetSupply200Response : Supply represents the current supply of MicroAlgos in the system

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetSupply200Response {
    /// Round
    #[serde(rename = "current_round")]
    pub current_round: u64,
    /// OnlineMoney
    #[serde(rename = "online-money")]
    pub online_money: u64,
    /// TotalMoney
    #[serde(rename = "total-money")]
    pub total_money: u64,
}

impl GetSupply200Response {
    /// Supply represents the current supply of MicroAlgos in the system
    pub fn new(current_round: u64, online_money: u64, total_money: u64) -> GetSupply200Response {
        GetSupply200Response {
            current_round,
            online_money,
            total_money,
        }
    }
}