use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ArimaResult : (Auto-)arima fitting result. Wrap everything in ArimaResult for easier refactoring if we want to use model-specific iteration results.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArimaResult {
    /// This message is repeated because there are multiple arima models fitted in auto-arima. For non-auto-arima model, its size is one.
    #[serde(rename = "arimaModelInfo", skip_serializing_if = "Option::is_none")]
    pub arima_model_info: Option<Vec<crate::google_rest_apis::bigquery_v2::models::ArimaModelInfo>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename = "seasonalPeriods", skip_serializing_if = "Option::is_none")]
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
}

impl ArimaResult {
    /// (Auto-)arima fitting result. Wrap everything in ArimaResult for easier refactoring if we want to use model-specific iteration results.
    pub fn new() -> ArimaResult {
        ArimaResult {
            arima_model_info: None,
            seasonal_periods: None,
        }
    }
}

/// Seasonal periods. Repeated because multiple periods are supported for one time series.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeasonalPeriods {
    #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
    SeasonalPeriodTypeUnspecified,
    #[serde(rename = "NO_SEASONALITY")]
    NoSeasonality,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "YEARLY")]
    Yearly,
}

impl Default for SeasonalPeriods {
    fn default() -> SeasonalPeriods {
        Self::SeasonalPeriodTypeUnspecified
    }
}
