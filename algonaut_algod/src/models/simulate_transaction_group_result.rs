/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// SimulateTransactionGroupResult : Simulation result for an atomic transaction group

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransactionGroupResult {
    /// Total budget added during execution of app calls in the transaction group.
    #[serde(rename = "app-budget-added", skip_serializing_if = "Option::is_none")]
    pub app_budget_added: Option<u64>,
    /// Total budget consumed during execution of app calls in the transaction group.
    #[serde(
        rename = "app-budget-consumed",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_budget_consumed: Option<u64>,
    /// If present, indicates which transaction in this group caused the failure. This array represents the path to the failing transaction. Indexes are zero based, the first element indicates the top-level transaction, and successive elements indicate deeper inner transactions.
    #[serde(rename = "failed-at", skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<Vec<u64>>,
    /// If present, indicates that the transaction group failed and specifies why that happened
    #[serde(rename = "failure-message", skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// Simulation result for individual transactions
    #[serde(rename = "txn-results")]
    pub txn_results: Vec<crate::models::SimulateTransactionResult>,
}

impl SimulateTransactionGroupResult {
    /// Simulation result for an atomic transaction group
    pub fn new(
        txn_results: Vec<crate::models::SimulateTransactionResult>,
    ) -> SimulateTransactionGroupResult {
        SimulateTransactionGroupResult {
            app_budget_added: None,
            app_budget_consumed: None,
            failed_at: None,
            failure_message: None,
            txn_results,
        }
    }
}