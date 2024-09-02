use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExplainQueryStage {
    /// Number of parallel input segments completed.
    #[serde(
        rename = "completedParallelInputs",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_parallel_inputs: Option<String>,
    /// Milliseconds the average shard spent on CPU-bound tasks.
    #[serde(rename = "computeMsAvg", skip_serializing_if = "Option::is_none")]
    pub compute_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent on CPU-bound tasks.
    #[serde(rename = "computeMsMax", skip_serializing_if = "Option::is_none")]
    pub compute_ms_max: Option<String>,
    /// Relative amount of time the average shard spent on CPU-bound tasks.
    #[serde(rename = "computeRatioAvg", skip_serializing_if = "Option::is_none")]
    pub compute_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on CPU-bound tasks.
    #[serde(rename = "computeRatioMax", skip_serializing_if = "Option::is_none")]
    pub compute_ratio_max: Option<f64>,
    /// Stage end time represented as milliseconds since epoch.
    #[serde(rename = "endMs", skip_serializing_if = "Option::is_none")]
    pub end_ms: Option<String>,
    /// Unique ID for stage within plan.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// IDs for stages that are inputs to this stage.
    #[serde(rename = "inputStages", skip_serializing_if = "Option::is_none")]
    pub input_stages: Option<Vec<String>>,
    /// Human-readable name for stage.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of parallel input segments to be processed.
    #[serde(rename = "parallelInputs", skip_serializing_if = "Option::is_none")]
    pub parallel_inputs: Option<String>,
    /// Milliseconds the average shard spent reading input.
    #[serde(rename = "readMsAvg", skip_serializing_if = "Option::is_none")]
    pub read_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent reading input.
    #[serde(rename = "readMsMax", skip_serializing_if = "Option::is_none")]
    pub read_ms_max: Option<String>,
    /// Relative amount of time the average shard spent reading input.
    #[serde(rename = "readRatioAvg", skip_serializing_if = "Option::is_none")]
    pub read_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent reading input.
    #[serde(rename = "readRatioMax", skip_serializing_if = "Option::is_none")]
    pub read_ratio_max: Option<f64>,
    /// Number of records read into the stage.
    #[serde(rename = "recordsRead", skip_serializing_if = "Option::is_none")]
    pub records_read: Option<String>,
    /// Number of records written by the stage.
    #[serde(rename = "recordsWritten", skip_serializing_if = "Option::is_none")]
    pub records_written: Option<String>,
    /// Total number of bytes written to shuffle.
    #[serde(rename = "shuffleOutputBytes", skip_serializing_if = "Option::is_none")]
    pub shuffle_output_bytes: Option<String>,
    /// Total number of bytes written to shuffle and spilled to disk.
    #[serde(
        rename = "shuffleOutputBytesSpilled",
        skip_serializing_if = "Option::is_none"
    )]
    pub shuffle_output_bytes_spilled: Option<String>,
    /// Slot-milliseconds used by the stage.
    #[serde(rename = "slotMs", skip_serializing_if = "Option::is_none")]
    pub slot_ms: Option<String>,
    /// Stage start time represented as milliseconds since epoch.
    #[serde(rename = "startMs", skip_serializing_if = "Option::is_none")]
    pub start_ms: Option<String>,
    /// Current status for the stage.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// List of operations within the stage in dependency order (approximately chronological).
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::google_rest_apis::bigquery_v2::models::ExplainQueryStep>>,
    /// Milliseconds the average shard spent waiting to be scheduled.
    #[serde(rename = "waitMsAvg", skip_serializing_if = "Option::is_none")]
    pub wait_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent waiting to be scheduled.
    #[serde(rename = "waitMsMax", skip_serializing_if = "Option::is_none")]
    pub wait_ms_max: Option<String>,
    /// Relative amount of time the average shard spent waiting to be scheduled.
    #[serde(rename = "waitRatioAvg", skip_serializing_if = "Option::is_none")]
    pub wait_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent waiting to be scheduled.
    #[serde(rename = "waitRatioMax", skip_serializing_if = "Option::is_none")]
    pub wait_ratio_max: Option<f64>,
    /// Milliseconds the average shard spent on writing output.
    #[serde(rename = "writeMsAvg", skip_serializing_if = "Option::is_none")]
    pub write_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent on writing output.
    #[serde(rename = "writeMsMax", skip_serializing_if = "Option::is_none")]
    pub write_ms_max: Option<String>,
    /// Relative amount of time the average shard spent on writing output.
    #[serde(rename = "writeRatioAvg", skip_serializing_if = "Option::is_none")]
    pub write_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on writing output.
    #[serde(rename = "writeRatioMax", skip_serializing_if = "Option::is_none")]
    pub write_ratio_max: Option<f64>,
}

impl ExplainQueryStage {
    pub fn new() -> ExplainQueryStage {
        ExplainQueryStage {
            completed_parallel_inputs: None,
            compute_ms_avg: None,
            compute_ms_max: None,
            compute_ratio_avg: None,
            compute_ratio_max: None,
            end_ms: None,
            id: None,
            input_stages: None,
            name: None,
            parallel_inputs: None,
            read_ms_avg: None,
            read_ms_max: None,
            read_ratio_avg: None,
            read_ratio_max: None,
            records_read: None,
            records_written: None,
            shuffle_output_bytes: None,
            shuffle_output_bytes_spilled: None,
            slot_ms: None,
            start_ms: None,
            status: None,
            steps: None,
            wait_ms_avg: None,
            wait_ms_max: None,
            wait_ratio_avg: None,
            wait_ratio_max: None,
            write_ms_avg: None,
            write_ms_max: None,
            write_ratio_avg: None,
            write_ratio_max: None,
        }
    }
}
