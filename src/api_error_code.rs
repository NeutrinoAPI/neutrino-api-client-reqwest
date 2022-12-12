/// Neutrino API error codes
#[allow(dead_code)]
pub mod error_codes {
    pub const INVALID_PARAMETER: i32 = 1;
    pub const MAX_CALL_LIMIT: i32 = 2;
    pub const BAD_URL: i32 = 3;
    pub const ABUSE_DETECTED: i32 = 4;
    pub const NOT_RESPONDING: i32 = 5;
    pub const CONCURRENT: i32 = 6;
    pub const NOT_VERIFIED: i32 = 7;
    pub const TELEPHONY_LIMIT: i32 = 8;
    pub const INVALID_JSON: i32 = 9;
    pub const ACCESS_DENIED: i32 = 10;
    pub const MAX_PHONE_CALLS: i32 = 11;
    pub const BAD_AUDIO: i32 = 12;
    pub const HLR_LIMIT_REACHED: i32 = 13;
    pub const TELEPHONY_BLOCKED: i32 = 14;
    pub const TELEPHONY_RATE_EXCEEDED: i32 = 15;
    pub const FREE_LIMIT: i32 = 16;
    pub const RENDERING_FAILED: i32 = 17;
    pub const DEPRECATED_API: i32 = 18;
    pub const CREDIT_LIMIT_REACHED: i32 = 19;
    pub const NOT_MULTI_ENABLED: i32 = 21;
    pub const NO_BATCH_MODE: i32 = 22;
    pub const BATCH_LIMIT_EXCEEDED: i32 = 23;
    pub const BATCH_INVALID: i32 = 24;
    pub const USER_DEFINED_DAILY_LIMIT: i32 = 31;
    pub const ACCESS_FORBIDDEN: i32 = 43;
    pub const REQUEST_TOO_LARGE: i32 = 44;
    pub const NO_ENDPOINT: i32 = 45;
    pub const INTERNAL_SERVER_ERROR: i32 = 51;
    pub const SERVER_OFFLINE: i32 = 52;
    pub const CONNECT_TIMEOUT: i32 = 61;
    pub const READ_TIMEOUT: i32 = 62;
    pub const TIMEOUT: i32 = 63;
    pub const DNS_LOOKUP_FAILED: i32 = 64;
    pub const TLS_PROTOCOL_ERROR: i32 = 65;
    pub const URL_PARSING_ERROR: i32 = 66;
    pub const NETWORK_IO_ERROR: i32 = 67;
    pub const FILE_IO_ERROR: i32 = 68;
    pub const INVALID_JSON_RESPONSE: i32 = 69;
    pub const NO_DATA: i32 = 70;
    pub const API_GATEWAY_ERROR: i32 = 71;
    
    /// Get description of error code
    pub fn get_error_message(error_code: i32) -> String {
        match error_code {
            INVALID_PARAMETER => "MISSING OR INVALID PARAMETER".to_string(),
            MAX_CALL_LIMIT => "DAILY API LIMIT EXCEEDED".to_string(),
            BAD_URL => "INVALID URL".to_string(),
            ABUSE_DETECTED => "ACCOUNT OR IP BANNED".to_string(),
            NOT_RESPONDING => "NOT RESPONDING. RETRY IN 5 SECONDS".to_string(),
            CONCURRENT => "TOO MANY CONNECTIONS".to_string(),
            NOT_VERIFIED => "ACCOUNT NOT VERIFIED".to_string(),
            TELEPHONY_LIMIT => "TELEPHONY NOT ENABLED ON YOUR ACCOUNT. PLEASE CONTACT SUPPORT FOR HELP".to_string(),
            INVALID_JSON => "INVALID JSON. JSON CONTENT TYPE SET BUT NON-PARSABLE JSON SUPPLIED".to_string(),
            ACCESS_DENIED => "ACCESS DENIED. PLEASE CONTACT SUPPORT FOR ACCESS TO THIS API".to_string(),
            MAX_PHONE_CALLS => "MAXIMUM SIMULTANEOUS PHONE CALLS".to_string(),
            BAD_AUDIO => "COULD NOT LOAD AUDIO FROM URL".to_string(),
            HLR_LIMIT_REACHED => "HLR LIMIT REACHED. CARD DECLINED".to_string(),
            TELEPHONY_BLOCKED => "CALLS AND SMS TO THIS NUMBER ARE LIMITED".to_string(),
            TELEPHONY_RATE_EXCEEDED => "CALL IN PROGRESS".to_string(),
            FREE_LIMIT => "FREE PLAN LIMIT EXCEEDED".to_string(),
            RENDERING_FAILED => "RENDERING FAILED. COULD NOT GENERATE OUTPUT FILE".to_string(),
            DEPRECATED_API => "THIS API IS DEPRECATED. PLEASE USE THE LATEST VERSION".to_string(),
            CREDIT_LIMIT_REACHED => "MAXIMUM ACCOUNT CREDIT LIMIT REACHED. PAYMENT METHOD DECLINED".to_string(),
            NOT_MULTI_ENABLED => "BATCH PROCESSING NOT ENABLED FOR THIS ENDPOINT".to_string(),
            NO_BATCH_MODE => "BATCH PROCESSING NOT AVAILABLE ON YOUR PLAN".to_string(),
            BATCH_LIMIT_EXCEEDED => "BATCH PROCESSING REQUEST LIMIT EXCEEDED".to_string(),
            BATCH_INVALID => "INVALID BATCH REQUEST. DOES NOT CONFORM TO SPEC".to_string(),
            USER_DEFINED_DAILY_LIMIT => "DAILY API LIMIT EXCEEDED. SET BY ACCOUNT HOLDER".to_string(),
            ACCESS_FORBIDDEN => "ACCESS DENIED. USER ID OR API KEY INVALID".to_string(),
            REQUEST_TOO_LARGE => "REQUEST TOO LARGE. MAXIMUM SIZE IS 5MB FOR DATA AND 25MB FOR UPLOADS".to_string(),
            NO_ENDPOINT => "ENDPOINT DOES NOT EXIST".to_string(),
            INTERNAL_SERVER_ERROR => "FATAL EXCEPTION. REQUEST COULD NOT BE COMPLETED".to_string(),
            SERVER_OFFLINE => "SERVER OFFLINE. MAINTENANCE IN PROGRESS".to_string(),
            CONNECT_TIMEOUT => "TIMEOUT OCCURRED CONNECTING TO SERVER".to_string(),
            READ_TIMEOUT => "TIMEOUT OCCURRED READING API RESPONSE".to_string(),
            TIMEOUT => "TIMEOUT OCCURRED DURING API REQUEST".to_string(),
            DNS_LOOKUP_FAILED => "ERROR RECEIVED FROM YOUR DNS RESOLVER".to_string(),
            TLS_PROTOCOL_ERROR => "ERROR DURING TLS PROTOCOL HANDSHAKE".to_string(),
            URL_PARSING_ERROR => "ERROR PARSING REQUEST URL".to_string(),
            NETWORK_IO_ERROR => "IO ERROR DURING API REQUEST".to_string(),
            FILE_IO_ERROR => "IO ERROR WRITING TO OUTPUT FILE".to_string(),
            INVALID_JSON_RESPONSE => "INVALID JSON DATA RECEIVED".to_string(),
            NO_DATA => "NO PAYLOAD DATA RECEIVED".to_string(),
            API_GATEWAY_ERROR => "API GATEWAY ERROR".to_string(),
            _ => format!("API Error: {}", error_code),
        }
    }
}
