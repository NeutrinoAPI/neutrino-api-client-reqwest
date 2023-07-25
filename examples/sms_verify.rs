extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(8);

    // The phone number to send a verification code to
    params.insert("number", "+12106100045");

    // ISO 2-letter country code, assume numbers are based in this country. If not set numbers are
    // assumed to be in international format (with or without the leading + sign)
    params.insert("country-code", "");

    // Pass in your own security code. This is useful if you have implemented TOTP or similar 2FA
    // methods. If not set then we will generate a secure random code
    params.insert("security-code", "");

    // The language to send the verification code in, available languages are:
    // • de - German
    // • en - English
    // • es - Spanish
    // • fr - French
    // • it - Italian
    // • pt - Portuguese
    // • ru - Russian
    params.insert("language-code", "en");

    // The number of digits to use in the security code (must be between 4 and 12)
    params.insert("code-length", "5");

    // Limit the total number of SMS allowed to the supplied phone number, if the limit is reached
    // within the TTL then error code 14 will be returned
    params.insert("limit", "10");

    // Set a custom brand or product name in the verification message
    params.insert("brand-name", "");

    // Set the TTL in number of days that the 'limit' option will remember a phone number (the default
    // is 1 day and the maximum is 365 days)
    params.insert("limit-ttl", "1");

    let response = client.sms_verify(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // True if this a valid phone number
        println!("number-valid: {:?}", data.get("number-valid"));
        
        // The security code generated, you can save this code to perform your own verification or you can
        // use the Verify Security Code API
        println!("security-code: {:?}", data.get("security-code"));
        
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
