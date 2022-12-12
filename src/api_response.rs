use core::option::Option;
use core::option::Option::{None, Some};
use std::error::Error;
use crate::api_error_code::error_codes;
use serde_json::{Map, Value};

/// API response payload, holds the response data along with any error details
pub struct APIResponse {
    /// The response data for JSON based APIs
    pub data: Option<Map<String, Value>>,
    /// The local file path storing the output for file based APIs
    pub file: Option<String>,
    /// The HTTP status code returned
    pub status_code: u16,
    /// The response content type (MIME type)
    pub content_type: String,
    /// The API error code if any error has occurred
    pub error_code: i32,
    /// The API error message if any error has occurred
    pub error_message: String,
    /// For client-side errors or exceptions get the underlying cause
    pub error_cause: Option<Box<dyn Error>>,
}

impl APIResponse {
    /// Create an API response for JSON data
    pub fn of_data(status_code: u16, content_type: String, data: Map<String, Value>) -> APIResponse {
        APIResponse {
            status_code,
            content_type,
            data: Some(data),
            file: None,
            error_code: 0,
            error_message: "".to_string(),
            error_cause: None,
        }
    }

    /// Create an API response for file data
    pub fn of_output_file_path(status_code: u16, content_type: String, output_file_path: String) -> APIResponse {
        APIResponse {
            status_code,
            content_type,
            data: None,
            file: Some(output_file_path),
            error_code: 0,
            error_message: "".to_string(),
            error_cause: None,
        }
    }

    /// Create an API response for error code
    pub fn of_error_code(status_code: u16, content_type: String, error_code: i32) -> APIResponse {
        let error_message = error_codes::get_error_message(error_code);
        APIResponse {
            status_code,
            content_type,
            data: None,
            file: None,
            error_code,
            error_message,
            error_cause: None,
        }
    }

    /// Create an API response for error cause
    pub fn of_error_cause(error_code: i32, error_cause: Option<Box<dyn Error>>) -> APIResponse {
        let error_message = error_codes::get_error_message(error_code);
        APIResponse {
            status_code: 0,
            content_type: "".to_string(),
            data: None,
            file: None,
            error_code,
            error_message,
            error_cause,
        }
    }

    /// Create an API response for status code
    pub fn of_http_response(
        status_code: u16,
        content_type: String,
        error_code: i32,
        error_message: String,
    ) -> APIResponse {
        APIResponse {
            status_code,
            content_type,
            data: None,
            file: None,
            error_code,
            error_message: error_message,
            error_cause: None,
        }
    }
}

