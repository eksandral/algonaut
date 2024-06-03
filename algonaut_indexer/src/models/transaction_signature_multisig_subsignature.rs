/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use algonaut_encoding::Bytes;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionSignatureMultisigSubsignature {
    /// \\[pk\\]
    #[serde(rename = "public-key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Bytes>,
    /// \\[s\\]
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,
}

impl TransactionSignatureMultisigSubsignature {
    pub fn new() -> TransactionSignatureMultisigSubsignature {
        TransactionSignatureMultisigSubsignature {
            public_key: None,
            signature: None,
        }
    }
}