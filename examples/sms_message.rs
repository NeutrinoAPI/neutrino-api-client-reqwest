extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(5);

    // The phone number to send a message to
    params.insert("number", "+12106100045");

    // ISO 2-letter country code, assume numbers are based in this country. If not set numbers are
    // assumed to be in international format (with or without the leading + sign)
    params.insert("country-code", "");

    // Limit the total number of SMS allowed to the supplied phone number, if the limit is reached
    // within the TTL then error code 14 will be returned
    params.insert("limit", "10");

    // The SMS message to send. Messages are truncated to a maximum of 150 characters for ASCII content
    // OR 70 characters for UTF content
    params.insert("message", "Hello, this is a test message!");

    // Set the TTL in number of days that the 'limit' option will remember a phone number (the default
    // is 1 day and the maximum is 365 days)
    params.insert("limit-ttl", "1");

    let response = client.sms_message(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // True if this a valid phone number
        println!("number-valid: {:?}", data.get("number-valid"));
        
        // True if the SMS has been sent
        println!("sent: {:?}", data.get("sent"));
    } else {
        // API request failed, you should handle this gracefully!
        eprintln!(
            "API Error: {}, Error Code: {}, HTTP Status Code: {}",
            response.error_message, response.error_code, response.status_code
        );
        if response.error_cause.is_some() {
            eprintln!("Error Caused By: {:?}", response.error_cause.unwrap());
        }
    }
}
