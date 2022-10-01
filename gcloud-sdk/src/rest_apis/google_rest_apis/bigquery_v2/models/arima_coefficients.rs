use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ArimaCoefficients : Arima coefficients.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArimaCoefficients {
    /// Auto-regressive coefficients, an array of double.
    #[serde(
        rename = "autoRegressiveCoefficients",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_regressive_coefficients: Option<Vec<f64>>,
    /// Intercept coefficient, just a double not an array.
    #[serde(
        rename = "interceptCoefficient",
        skip_serializing_if = "Option::is_none"
    )]
    pub intercept_coefficient: Option<f64>,
    /// Moving-average coefficients, an array of double.
    #[serde(
        rename = "movingAverageCoefficients",
        skip_serializing_if = "Option::is_none"
    )]
    pub moving_average_coefficients: Option<Vec<f64>>,
}

impl ArimaCoefficients {
    /// Arima coefficients.
    pub fn new() -> ArimaCoefficients {
        ArimaCoefficients {
            auto_regressive_coefficients: None,
            intercept_coefficient: None,
            moving_average_coefficients: None,
        }
    }
}
