extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // The security code to verify
    params.insert("security-code", "123456");

    // If set then enable additional brute-force protection by limiting the number of attempts by the
    // supplied value. This can be set to any unique identifier you would like to limit by, for example
    // a hash of the users email, phone number or IP address. Requests to this API will be ignored after
    // approximately 10 failed verification attempts
    params.insert("limit-by", "");

    let response = client.verify_security_code(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // True if the code is valid
        println!("verified: {:?}", data.get("verified"));
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
