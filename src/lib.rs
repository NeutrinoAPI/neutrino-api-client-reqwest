//! NeutrinoAPIClient
//!
//! Rust client using Reqwest crate

mod api_error_code;
mod api_response;

use api_error_code::error_codes;
use api_response::APIResponse;
use core::result::Result::{Err, Ok};
use core::time::Duration;
use reqwest::blocking::RequestBuilder;
use reqwest::header::HeaderValue;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;

/// Make a request to the Neutrino API
pub struct NeutrinoAPIClient {
    user_id: &'static str,
    api_key: &'static str,
    base_url: &'static str,
}

impl NeutrinoAPIClient {
    /// Neutrino API endpoint
    #[allow(dead_code)]
    pub const MULTICLOUD_ENDPOINT: &'static str = "https://neutrinoapi.net/";
    /// Neutrino API endpoint
    #[allow(dead_code)]
    pub const AWS_ENDPOINT: &'static str = "https://aws.neutrinoapi.net/";
    /// Neutrino API endpoint
    #[allow(dead_code)]
    pub const GCP_ENDPOINT: &'static str = "https://gcp.neutrinoapi.net/";
    /// Neutrino API endpoint
    #[allow(dead_code)]
    pub const MS_AZURE_ENDPOINT: &'static str = "https://msa.neutrinoapi.net/";

    /// Constructs a new client using the default Neutrino API endpoint
    pub fn new(
        user_id: &'static str,
        api_key: &'static str
    ) -> NeutrinoAPIClient {
        NeutrinoAPIClient {
            user_id,
            api_key,
            base_url: NeutrinoAPIClient::MULTICLOUD_ENDPOINT
        }
    }

    /// Constructs a new client using a user defined Neutrino API endpoint
    pub fn new_with_base_url(
        user_id: &'static str,
        api_key: &'static str,
        base_url: &'static str
    ) -> NeutrinoAPIClient {
        NeutrinoAPIClient {
            user_id,
            api_key,
            base_url
        }
    }

    /// Detect bad words, swear words and profanity in a given text
    ///
    /// ## The parameters this API accepts are:
    /// * censor-character - The character to use to censor out the bad words found
    /// * catalog - Which catalog of bad words to use
    /// * content - The content to scan
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/bad-word-filter
    ///
    pub fn bad_word_filter(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "bad-word-filter", params, default_path, 30);
    }

    /// Download our entire BIN database for direct use on your own systems
    ///
    /// ## The parameters this API accepts are:
    /// * include-iso3 - Include ISO 3-letter country codes and ISO 3-letter currency codes in the data
    /// * include-8digit - Include 8-digit and higher BIN codes
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/bin-list-download
    ///
    pub fn bin_list_download(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "bin-list-download", params, output_file_path, 30);
    }

    /// Perform a BIN (Bank Identification Number) or IIN (Issuer Identification Number) lookup
    ///
    /// ## The parameters this API accepts are:
    /// * bin-number - The BIN or IIN number
    /// * customer-ip - Pass in the customers IP address and we will return some extra information about them
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/bin-lookup
    ///
    pub fn bin_lookup(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "bin-lookup", params, default_path, 10);
    }

    /// Browser bot can extract content, interact with keyboard and mouse events, and execute JavaScript on a website
    ///
    /// ## The parameters this API accepts are:
    /// * delay - Delay in seconds to wait before capturing any page data
    /// * ignore-certificate-errors - Ignore any TLS/SSL certificate errors and load the page anyway
    /// * selector - Extract content from the page DOM using this selector
    /// * url - The URL to load
    /// * timeout - Timeout in seconds
    /// * exec - Execute JavaScript on the website
    /// * user-agent - Override the browsers default user-agent string with this one
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/browser-bot
    ///
    pub fn browser_bot(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "browser-bot", params, default_path, 300);
    }

    /// A currency and unit conversion tool
    ///
    /// ## The parameters this API accepts are:
    /// * from-value - The value to convert from (e.g. 10.95)
    /// * from-type - The type of the value to convert from (e.g. USD)
    /// * to-type - The type to convert to (e.g. EUR)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/convert
    ///
    pub fn convert(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "convert", params, default_path, 10);
    }

    /// Parse, validate and clean an email address
    ///
    /// ## The parameters this API accepts are:
    /// * email - An email address
    /// * fix-typos - Automatically attempt to fix typos in the address
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/email-validate
    ///
    pub fn email_validate(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "email-validate", params, default_path, 30);
    }

    /// SMTP based email address verification
    ///
    /// ## The parameters this API accepts are:
    /// * email - An email address
    /// * fix-typos - Automatically attempt to fix typos in the address
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/email-verify
    ///
    pub fn email_verify(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "email-verify", params, default_path, 120);
    }

    /// Geocode an address, partial address or just the name of a place
    ///
    /// ## The parameters this API accepts are:
    /// * address - The full address
    /// * house-number - The house/building number to locate
    /// * street - The street/road name to locate
    /// * city - The city/town name to locate
    /// * county - The county/region name to locate
    /// * state - The state name to locate
    /// * postal-code - The postal code to locate
    /// * country-code - Limit result to this country (the default is no country bias)
    /// * language-code - The language to display results in
    /// * fuzzy-search - If no matches are found for the given address
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/geocode-address
    ///
    pub fn geocode_address(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "geocode-address", params, default_path, 30);
    }

    /// Convert a geographic coordinate (latitude and longitude) into a real world address
    ///
    /// ## The parameters this API accepts are:
    /// * latitude - The location latitude in decimal degrees format
    /// * longitude - The location longitude in decimal degrees format
    /// * language-code - The language to display results in
    /// * zoom - The zoom level to respond with: address - the most precise address available street - the street level city - the city level state - the state level country - the country level 
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/geocode-reverse
    ///
    pub fn geocode_reverse(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "geocode-reverse", params, default_path, 30);
    }

    /// Connect to the global mobile cellular network and retrieve the status of a mobile device
    ///
    /// ## The parameters this API accepts are:
    /// * number - A phone number
    /// * country-code - ISO 2-letter country code
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/hlr-lookup
    ///
    pub fn hlr_lookup(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "hlr-lookup", params, default_path, 30);
    }

    /// Check the reputation of an IP address, domain name or URL against a comprehensive list of blacklists and blocklists
    ///
    /// ## The parameters this API accepts are:
    /// * host - An IP address
    /// * list-rating - Only check lists with this rating or better
    /// * zones - Only check these DNSBL zones/hosts
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/host-reputation
    ///
    pub fn host_reputation(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "host-reputation", params, default_path, 120);
    }

    /// Clean and sanitize untrusted HTML
    ///
    /// ## The parameters this API accepts are:
    /// * output-type - The level of sanitization
    /// * content - The HTML content
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/html-clean
    ///
    pub fn html_clean(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "html-clean", params, output_file_path, 30);
    }

    /// Render HTML content to PDF, JPG or PNG
    ///
    /// ## The parameters this API accepts are:
    /// * margin - The document margin (in mm)
    /// * css - Inject custom CSS into the HTML
    /// * image-width - If rendering to an image format (PNG or JPG) use this image width (in pixels)
    /// * footer - The footer HTML to insert into each page
    /// * format - Which format to output
    /// * zoom - Set the zoom factor when rendering the page (2.0 for double size
    /// * title - The document title
    /// * content - The HTML content
    /// * page-width - Set the PDF page width explicitly (in mm)
    /// * timeout - Timeout in seconds
    /// * margin-right - The document right margin (in mm)
    /// * grayscale - Render the final document in grayscale
    /// * margin-left - The document left margin (in mm)
    /// * page-size - Set the document page size
    /// * delay - Number of seconds to wait before rendering the page (can be useful for pages with animations etc)
    /// * ignore-certificate-errors - Ignore any TLS/SSL certificate errors
    /// * page-height - Set the PDF page height explicitly (in mm)
    /// * image-height - If rendering to an image format (PNG or JPG) use this image height (in pixels)
    /// * header - The header HTML to insert into each page
    /// * margin-top - The document top margin (in mm)
    /// * margin-bottom - The document bottom margin (in mm)
    /// * landscape - Set the document to landscape orientation
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/html-render
    ///
    pub fn html_render(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "html-render", params, output_file_path, 300);
    }

    /// Resize an image and output as either JPEG or PNG
    ///
    /// ## The parameters this API accepts are:
    /// * width - The width to resize to (in px) while preserving aspect ratio
    /// * format - The output image format
    /// * image-url - The URL or Base64 encoded Data URL for the source image (you can also upload an image file directly in which case this field is ignored)
    /// * height - The height to resize to (in px) while preserving aspect ratio
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/image-resize
    ///
    pub fn image_resize(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "image-resize", params, output_file_path, 20);
    }

    /// Watermark one image with another image
    ///
    /// ## The parameters this API accepts are:
    /// * format - The output image format
    /// * width - If set resize the resulting image to this width (in px) while preserving aspect ratio
    /// * image-url - The URL or Base64 encoded Data URL for the source image (you can also upload an image file directly in which case this field is ignored)
    /// * position - The position of the watermark image
    /// * watermark-url - The URL or Base64 encoded Data URL for the watermark image (you can also upload an image file directly in which case this field is ignored)
    /// * opacity - The opacity of the watermark (0 to 100)
    /// * height - If set resize the resulting image to this height (in px) while preserving aspect ratio
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/image-watermark
    ///
    pub fn image_watermark(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "image-watermark", params, output_file_path, 20);
    }

    /// The IP Blocklist API will detect potentially malicious or dangerous IP addresses
    ///
    /// ## The parameters this API accepts are:
    /// * ip - An IPv4 or IPv6 address
    /// * vpn-lookup - Include public VPN provider IP addresses
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/ip-blocklist
    ///
    pub fn ip_blocklist(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "ip-blocklist", params, default_path, 10);
    }

    /// This API is a direct feed to our IP blocklist data
    ///
    /// ## The parameters this API accepts are:
    /// * format - The data format
    /// * include-vpn - Include public VPN provider IP addresses
    /// * cidr - Output IPs using CIDR notation
    /// * ip6 - Output the IPv6 version of the blocklist
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/ip-blocklist-download
    ///
    pub fn ip_blocklist_download(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "ip-blocklist-download", params, output_file_path, 30);
    }

    /// Get location information about an IP address and do reverse DNS (PTR) lookups
    ///
    /// ## The parameters this API accepts are:
    /// * ip - IPv4 or IPv6 address
    /// * reverse-lookup - Do a reverse DNS (PTR) lookup
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/ip-info
    ///
    pub fn ip_info(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "ip-info", params, default_path, 10);
    }

    /// Analyze and extract provider information for an IP address
    ///
    /// ## The parameters this API accepts are:
    /// * ip - IPv4 or IPv6 address
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/ip-probe
    ///
    pub fn ip_probe(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "ip-probe", params, default_path, 120);
    }

    /// Make an automated call to any valid phone number and playback an audio message
    ///
    /// ## The parameters this API accepts are:
    /// * number - The phone number to call
    /// * limit - Limit the total number of calls allowed to the supplied phone number
    /// * audio-url - A URL to a valid audio file
    /// * limit-ttl - Set the TTL in number of days that the 'limit' option will remember a phone number (the default is 1 day and the maximum is 365 days)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/phone-playback
    ///
    pub fn phone_playback(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "phone-playback", params, default_path, 30);
    }

    /// Parse, validate and get location information about a phone number
    ///
    /// ## The parameters this API accepts are:
    /// * number - A phone number
    /// * country-code - ISO 2-letter country code
    /// * ip - Pass in a users IP address and we will assume numbers are based in the country of the IP address
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/phone-validate
    ///
    pub fn phone_validate(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "phone-validate", params, default_path, 10);
    }

    /// Make an automated call to any valid phone number and playback a unique security code
    ///
    /// ## The parameters this API accepts are:
    /// * number - The phone number to send the verification code to
    /// * country-code - ISO 2-letter country code
    /// * security-code - Pass in your own security code
    /// * language-code - The language to playback the verification code in
    /// * code-length - The number of digits to use in the security code (between 4 and 12)
    /// * limit - Limit the total number of calls allowed to the supplied phone number
    /// * playback-delay - The delay in milliseconds between the playback of each security code
    /// * limit-ttl - Set the TTL in number of days that the 'limit' option will remember a phone number (the default is 1 day and the maximum is 365 days)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/phone-verify
    ///
    pub fn phone_verify(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "phone-verify", params, default_path, 30);
    }

    /// Generate a QR code as a PNG image
    ///
    /// ## The parameters this API accepts are:
    /// * width - The width of the QR code (in px)
    /// * fg-color - The QR code foreground color
    /// * bg-color - The QR code background color
    /// * content - The content to encode into the QR code (e.g. a URL or a phone number)
    /// * height - The height of the QR code (in px)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/qr-code
    ///
    pub fn qr_code(
        &self,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
    ) -> APIResponse {
        return self.exec_request("POST", "qr-code", params, output_file_path, 20);
    }

    /// Send a free-form message to any mobile device via SMS
    ///
    /// ## The parameters this API accepts are:
    /// * number - The phone number to send a message to
    /// * country-code - ISO 2-letter country code
    /// * limit - Limit the total number of SMS allowed to the supplied phone number
    /// * message - The SMS message to send
    /// * limit-ttl - Set the TTL in number of days that the 'limit' option will remember a phone number (the default is 1 day and the maximum is 365 days)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/sms-message
    ///
    pub fn sms_message(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "sms-message", params, default_path, 30);
    }

    /// Send a unique security code to any mobile device via SMS
    ///
    /// ## The parameters this API accepts are:
    /// * number - The phone number to send a verification code to
    /// * country-code - ISO 2-letter country code
    /// * security-code - Pass in your own security code
    /// * language-code - The language to send the verification code in
    /// * code-length - The number of digits to use in the security code (must be between 4 and 12)
    /// * limit - Limit the total number of SMS allowed to the supplied phone number
    /// * limit-ttl - Set the TTL in number of days that the 'limit' option will remember a phone number (the default is 1 day and the maximum is 365 days)
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/sms-verify
    ///
    pub fn sms_verify(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("POST", "sms-verify", params, default_path, 30);
    }

    /// Parse, validate and get detailed user-agent information from a user agent string or from client hints
    ///
    /// ## The parameters this API accepts are:
    /// * ua - The user-agent string to lookup
    /// * ua-version - For client hints this corresponds to the 'UA-Full-Version' header or 'uaFullVersion' from NavigatorUAData
    /// * ua-platform - For client hints this corresponds to the 'UA-Platform' header or 'platform' from NavigatorUAData
    /// * ua-platform-version - For client hints this corresponds to the 'UA-Platform-Version' header or 'platformVersion' from NavigatorUAData
    /// * ua-mobile - For client hints this corresponds to the 'UA-Mobile' header or 'mobile' from NavigatorUAData
    /// * device-model - For client hints this corresponds to the 'UA-Model' header or 'model' from NavigatorUAData
    /// * device-brand - This parameter is only used in combination with 'device-model' when doing direct device lookups without any user-agent data
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/ua-lookup
    ///
    pub fn ua_lookup(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "ua-lookup", params, default_path, 10);
    }

    /// Parse, analyze and retrieve content from the supplied URL
    ///
    /// ## The parameters this API accepts are:
    /// * url - The URL to probe
    /// * fetch-content - If this URL responds with html
    /// * ignore-certificate-errors - Ignore any TLS/SSL certificate errors and load the URL anyway
    /// * timeout - Timeout in seconds
    /// * retry - If the request fails for any reason try again this many times
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/url-info
    ///
    pub fn url_info(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "url-info", params, default_path, 30);
    }

    /// Check if a security code sent via SMS Verify or Phone Verify is valid
    ///
    /// ## The parameters this API accepts are:
    /// * security-code - The security code to verify
    /// * limit-by - If set then enable additional brute-force protection by limiting the number of attempts by the supplied value
    ///
    /// ## Link
    /// * https://www.neutrinoapi.com/api/verify-security-code
    ///
    pub fn verify_security_code(&self, params: HashMap<&str, &str>) -> APIResponse {
        let default_path = PathBuf::default();
        return self.exec_request("GET", "verify-security-code", params, default_path, 30);
    }

    /// Make a request to the Neutrino API
    pub fn exec_request(
        &self,
        http_method: &'static str,
        endpoint: &'static str,
        params: HashMap<&str, &str>,
        output_file_path: PathBuf,
        read_timeout: u64,
    ) -> APIResponse {
        let read_timeout = Duration::from_secs(read_timeout);
        let default_content_type = if output_file_path.to_str().unwrap_or_default().len() > 0 {
            HeaderValue::from_static("application/octet-stream")
        } else {
            HeaderValue::from_static("text/plain")
        };
        let builder: RequestBuilder;
        if http_method.contains("GET") {
            builder = reqwest::blocking::Client::new()
                .get(format!("{}{}", self.base_url, endpoint))
                .query(&params);
        } else {
            builder = reqwest::blocking::Client::new()
                .post(format!("{}{}", self.base_url, endpoint))
                .form(&params);
        }
        let builder = builder
            .header("API-Key", self.api_key)
            .header("User-ID", self.user_id)
            .timeout(read_timeout);
        let response = builder.send();

        match response {
            Ok(response) if !response.status().is_success() => {
                let status_code = response.status().as_u16();
                let content_type = response
                    .headers()
                    .get("content-type")
                    .unwrap_or(&default_content_type)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                if content_type.contains("application/json") {
                    match response.json::<Value>() {
                        Ok(json) => {
                            let error_code = json
                                .get("api-error")
                                .unwrap_or(&json!(0))
                                .as_i64()
                                .unwrap_or(0) as i32;
                            let error_message = json
                                .get("api-error-msg")
                                .unwrap_or(&json!(""))
                                .as_str()
                                .unwrap_or_default()
                                .to_string();
                            APIResponse::of_http_response(status_code, content_type, error_code, error_message)
                        }
                        Err(err) => APIResponse::of_error_cause(error_codes::INVALID_JSON, Some(Box::new(err))),
                    }
                } else {
                    match response.text() {
                        Ok(text) => APIResponse::of_http_response(status_code, content_type, error_codes::NETWORK_IO_ERROR, text),
                        Err(err) => APIResponse::of_error_cause(error_codes::NETWORK_IO_ERROR, Some(Box::new(err))),
                    }
                }
            }
            Ok(mut response) => {
                let status_code = response.status().as_u16();
                let content_type = response
                    .headers()
                    .get("content-type")
                    .unwrap_or(&default_content_type)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                if content_type.contains("application/json") {
                    // 200 JSON
                    match response.json::<Map<String, Value>>() {
                        Ok(json) => APIResponse::of_data(status_code, content_type, json),
                        Err(err) => APIResponse::of_error_cause(error_codes::INVALID_JSON, Some(Box::new(err))),
                    }
                } else {
                    // 200 File
                    let output_file = std::fs::File::create(&output_file_path).ok();
                    if output_file.is_some() {
                        let mut output_file = output_file.unwrap();
                        match response.copy_to(&mut output_file) {
                            Ok(_) => APIResponse::of_output_file_path(status_code, content_type, output_file_path.to_str().unwrap_or_default().to_string()),
                            Err(err) => APIResponse::of_error_cause(error_codes::FILE_IO_ERROR, Some(Box::new(err))),
                        }
                    } else {
                        match response.text() {
                            Ok(text) => APIResponse::of_http_response(status_code, content_type, error_codes::API_GATEWAY_ERROR, text),
                            Err(err) => APIResponse::of_error_cause(error_codes::API_GATEWAY_ERROR, Some(Box::new(err))),
                        }
                    }
                }
            }
            Err(err) => {
                if err.is_builder() {
                    return APIResponse::of_error_cause(error_codes::BAD_URL, Some(Box::new(err)));
                }
                if err.is_timeout() {
                    return APIResponse::of_error_cause(error_codes::READ_TIMEOUT, Some(Box::new(err)));
                }
                if err.is_request() {
                    return APIResponse::of_error_cause(error_codes::TLS_PROTOCOL_ERROR, Some(Box::new(err)));
                }
                if err.is_connect() {
                    return APIResponse::of_error_cause(error_codes::CONNECT_TIMEOUT, Some(Box::new(err)));
                }
                APIResponse::of_error_cause(error_codes::NETWORK_IO_ERROR, Some(Box::new(err)))
            }
        }
    }
}
