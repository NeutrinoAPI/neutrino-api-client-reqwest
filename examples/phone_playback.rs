extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(4);

    // The phone number to call. Must be in valid international format
    params.insert("number", "+12106100045");

    // Limit the total number of calls allowed to the supplied phone number, if the limit is reached
    // within the TTL then error code 14 will be returned
    params.insert("limit", "3");

    // A URL to a valid audio file. Accepted audio formats are:
    // • MP3
    // • WAV
    // • OGG You can use the following MP3 URL for testing:
    //   https://www.neutrinoapi.com/test-files/test1.mp3
    params.insert("audio-url", "https://www.neutrinoapi.com/test-files/test1.mp3");

    // Set the TTL in number of days that the 'limit' option will remember a phone number (the default
    // is 1 day and the maximum is 365 days)
    params.insert("limit-ttl", "1");

    let response = client.phone_playback(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // True if the call is being made now
        println!("calling: {:?}", data.get("calling"));
        
        // True if this a valid phone number
        println!("number-valid: {:?}", data.get("number-valid"));
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
