use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ArimaOrder : Arima order, can be used for both non-seasonal and seasonal parts.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArimaOrder {
    /// Order of the differencing part.
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    /// Order of the autoregressive part.
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    /// Order of the moving-average part.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ArimaOrder {
    /// Arima order, can be used for both non-seasonal and seasonal parts.
    pub fn new() -> ArimaOrder {
        ArimaOrder {
            d: None,
            p: None,
            q: None,
        }
    }
}
