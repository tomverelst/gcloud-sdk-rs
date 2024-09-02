/*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatistics2 {
    #[serde(rename = "biEngineStatistics", skip_serializing_if = "Option::is_none")]
    pub bi_engine_statistics: Option<Box<models::BiEngineStatistics>>,
    /// [Output only] Billing tier for the job.
    #[serde(rename = "billingTier", skip_serializing_if = "Option::is_none")]
    pub billing_tier: Option<i32>,
    /// [Output only] Whether the query result was fetched from the query cache.
    #[serde(rename = "cacheHit", skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,
    /// [Output only] [Preview] The number of row access policies affected by a DDL statement. Present only for DROP ALL ROW ACCESS POLICIES queries.
    #[serde(
        rename = "ddlAffectedRowAccessPolicyCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddl_affected_row_access_policy_count: Option<String>,
    #[serde(
        rename = "ddlDestinationTable",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddl_destination_table: Option<Box<models::TableReference>>,
    /// The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): \"CREATE\": The query created the DDL target. \"SKIP\": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. \"REPLACE\": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. \"DROP\": The query deleted the DDL target.
    #[serde(
        rename = "ddlOperationPerformed",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddl_operation_performed: Option<String>,
    #[serde(rename = "ddlTargetDataset", skip_serializing_if = "Option::is_none")]
    pub ddl_target_dataset: Option<Box<models::DatasetReference>>,
    #[serde(rename = "ddlTargetRoutine", skip_serializing_if = "Option::is_none")]
    pub ddl_target_routine: Option<Box<models::RoutineReference>>,
    #[serde(
        rename = "ddlTargetRowAccessPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddl_target_row_access_policy: Option<Box<models::RowAccessPolicyReference>>,
    #[serde(rename = "ddlTargetTable", skip_serializing_if = "Option::is_none")]
    pub ddl_target_table: Option<Box<models::TableReference>>,
    #[serde(rename = "dmlStats", skip_serializing_if = "Option::is_none")]
    pub dml_stats: Option<Box<models::DmlStatistics>>,
    /// [Output only] The original estimate of bytes processed for the job.
    #[serde(
        rename = "estimatedBytesProcessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_bytes_processed: Option<String>,
    #[serde(rename = "mlStatistics", skip_serializing_if = "Option::is_none")]
    pub ml_statistics: Option<Box<models::MlStatistics>>,
    #[serde(rename = "modelTraining", skip_serializing_if = "Option::is_none")]
    pub model_training: Option<Box<models::BigQueryModelTraining>>,
    /// [Output only, Beta] Deprecated; do not use.
    #[serde(
        rename = "modelTrainingCurrentIteration",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_training_current_iteration: Option<i32>,
    /// [Output only, Beta] Deprecated; do not use.
    #[serde(
        rename = "modelTrainingExpectedTotalIteration",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_training_expected_total_iteration: Option<String>,
    /// [Output only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename = "numDmlAffectedRows", skip_serializing_if = "Option::is_none")]
    pub num_dml_affected_rows: Option<String>,
    /// [Output only] Describes execution plan for the query.
    #[serde(rename = "queryPlan", skip_serializing_if = "Option::is_none")]
    pub query_plan: Option<Vec<models::ExplainQueryStage>>,
    /// [Output only] Referenced routines (persistent user-defined functions and stored procedures) for the job.
    #[serde(rename = "referencedRoutines", skip_serializing_if = "Option::is_none")]
    pub referenced_routines: Option<Vec<models::RoutineReference>>,
    /// [Output only] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list.
    #[serde(rename = "referencedTables", skip_serializing_if = "Option::is_none")]
    pub referenced_tables: Option<Vec<models::TableReference>>,
    /// [Output only] Job resource usage breakdown by reservation.
    #[serde(rename = "reservationUsage", skip_serializing_if = "Option::is_none")]
    pub reservation_usage: Option<Vec<models::JobStatistics2ReservationUsageInner>>,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<models::TableSchema>>,
    #[serde(rename = "searchStatistics", skip_serializing_if = "Option::is_none")]
    pub search_statistics: Option<Box<models::SearchStatistics>>,
    #[serde(rename = "sparkStatistics", skip_serializing_if = "Option::is_none")]
    pub spark_statistics: Option<Box<models::SparkStatistics>>,
    /// The type of query statement, if valid. Possible values (new values might be added in the future): \"SELECT\": SELECT query. \"INSERT\": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"UPDATE\": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"DELETE\": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"MERGE\": MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"ALTER_TABLE\": ALTER TABLE query. \"ALTER_VIEW\": ALTER VIEW query. \"ASSERT\": ASSERT condition AS 'description'. \"CREATE_FUNCTION\": CREATE FUNCTION query. \"CREATE_MODEL\": CREATE [OR REPLACE] MODEL ... AS SELECT ... . \"CREATE_PROCEDURE\": CREATE PROCEDURE query. \"CREATE_TABLE\": CREATE [OR REPLACE] TABLE without AS SELECT. \"CREATE_TABLE_AS_SELECT\": CREATE [OR REPLACE] TABLE ... AS SELECT ... . \"CREATE_VIEW\": CREATE [OR REPLACE] VIEW ... AS SELECT ... . \"DROP_FUNCTION\" : DROP FUNCTION query. \"DROP_PROCEDURE\": DROP PROCEDURE query. \"DROP_TABLE\": DROP TABLE query. \"DROP_VIEW\": DROP VIEW query.
    #[serde(rename = "statementType", skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<String>,
    /// [Output only] [Beta] Describes a timeline of job execution.
    #[serde(rename = "timeline", skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<models::QueryTimelineSample>>,
    /// [Output only] Total bytes billed for the job.
    #[serde(rename = "totalBytesBilled", skip_serializing_if = "Option::is_none")]
    pub total_bytes_billed: Option<String>,
    /// [Output only] Total bytes processed for the job.
    #[serde(
        rename = "totalBytesProcessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bytes_processed: Option<String>,
    /// [Output only] For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost.
    #[serde(
        rename = "totalBytesProcessedAccuracy",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bytes_processed_accuracy: Option<String>,
    /// [Output only] Total number of partitions processed from all partitioned tables referenced in the job.
    #[serde(
        rename = "totalPartitionsProcessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_partitions_processed: Option<String>,
    /// [Output only] Slot-milliseconds for the job.
    #[serde(rename = "totalSlotMs", skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,
    /// Standard SQL only: list of undeclared query parameters detected during a dry run validation.
    #[serde(
        rename = "undeclaredQueryParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub undeclared_query_parameters: Option<Vec<models::QueryParameter>>,
}

impl JobStatistics2 {
    pub fn new() -> JobStatistics2 {
        JobStatistics2 {
            bi_engine_statistics: None,
            billing_tier: None,
            cache_hit: None,
            ddl_affected_row_access_policy_count: None,
            ddl_destination_table: None,
            ddl_operation_performed: None,
            ddl_target_dataset: None,
            ddl_target_routine: None,
            ddl_target_row_access_policy: None,
            ddl_target_table: None,
            dml_stats: None,
            estimated_bytes_processed: None,
            ml_statistics: None,
            model_training: None,
            model_training_current_iteration: None,
            model_training_expected_total_iteration: None,
            num_dml_affected_rows: None,
            query_plan: None,
            referenced_routines: None,
            referenced_tables: None,
            reservation_usage: None,
            schema: None,
            search_statistics: None,
            spark_statistics: None,
            statement_type: None,
            timeline: None,
            total_bytes_billed: None,
            total_bytes_processed: None,
            total_bytes_processed_accuracy: None,
            total_partitions_processed: None,
            total_slot_ms: None,
            undeclared_query_parameters: None,
        }
    }
}
