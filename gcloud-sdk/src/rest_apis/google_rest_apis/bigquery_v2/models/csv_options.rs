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
pub struct CsvOptions {
    /// [Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false.
    #[serde(rename = "allowJaggedRows", skip_serializing_if = "Option::is_none")]
    pub allow_jagged_rows: Option<bool>,
    /// [Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(
        rename = "allowQuotedNewlines",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_quoted_newlines: Option<bool>,
    /// [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// [Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',').
    #[serde(rename = "fieldDelimiter", skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    /// [Optional] An custom string that will represent a NULL value in CSV import data.
    #[serde(rename = "null_marker", skip_serializing_if = "Option::is_none")]
    pub null_marker: Option<String>,
    /// [Optional] Preserves the embedded ASCII control characters (the first 32 characters in the ASCII-table, from '\\x00' to '\\x1F') when loading from CSV. Only applicable to CSV, ignored for other formats.
    #[serde(
        rename = "preserveAsciiControlCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub preserve_ascii_control_characters: Option<bool>,
    /// [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// [Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[serde(rename = "skipLeadingRows", skip_serializing_if = "Option::is_none")]
    pub skip_leading_rows: Option<String>,
}

impl CsvOptions {
    pub fn new() -> CsvOptions {
        CsvOptions {
            allow_jagged_rows: None,
            allow_quoted_newlines: None,
            encoding: None,
            field_delimiter: None,
            null_marker: None,
            preserve_ascii_control_characters: None,
            quote: None,
            skip_leading_rows: None,
        }
    }
}
