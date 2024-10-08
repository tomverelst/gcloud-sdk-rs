use serde::{Deserialize, Serialize}; /*
                                      * Workflow Executions API
                                      *
                                      * Execute workflows created with Workflows API.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// StepEntryMetadata : StepEntryMetadata contains metadata information about this step.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StepEntryMetadata {
    /// Progress number represents the current state of the current progress. eg: A step entry represents the 4th iteration in a progress of PROGRESS_TYPE_FOR.
    #[serde(rename = "progressNumber", skip_serializing_if = "Option::is_none")]
    pub progress_number: Option<String>,
    /// Progress type of this step entry.
    #[serde(rename = "progressType", skip_serializing_if = "Option::is_none")]
    pub progress_type: Option<ProgressType>,
    /// Child thread id that this step entry belongs to.
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl StepEntryMetadata {
    /// StepEntryMetadata contains metadata information about this step.
    pub fn new() -> StepEntryMetadata {
        StepEntryMetadata {
            progress_number: None,
            progress_type: None,
            thread_id: None,
        }
    }
}

/// Progress type of this step entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "PROGRESS_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "PROGRESS_TYPE_FOR")]
    For,
    #[serde(rename = "PROGRESS_TYPE_SWITCH")]
    Switch,
    #[serde(rename = "PROGRESS_TYPE_RETRY")]
    Retry,
    #[serde(rename = "PROGRESS_TYPE_PARALLEL_FOR")]
    ParallelFor,
    #[serde(rename = "PROGRESS_TYPE_PARALLEL_BRANCH")]
    ParallelBranch,
}

impl Default for ProgressType {
    fn default() -> ProgressType {
        Self::Unspecified
    }
}
