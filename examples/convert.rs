extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(3);

    // The value to convert from (e.g. 10.95)
    params.insert("from-value", "100");

    // The type of the value to convert from (e.g. USD)
    params.insert("from-type", "USD");

    // The type to convert to (e.g. EUR)
    params.insert("to-type", "EUR");

    let response = client.convert(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The full name of the type being converted from
        println!("from-name: {:?}", data.get("from-name"));
        
        // The standard UTF-8 symbol used to represent the type being converted from
        println!("from-symbol: {:?}", data.get("from-symbol"));
        
        // The type of the value being converted from
        println!("from-type: {:?}", data.get("from-type"));
        
        // The value being converted from
        println!("from-value: {:?}", data.get("from-value"));
        
        // The result of the conversion in string format
        println!("result: {:?}", data.get("result"));
        
        // The result of the conversion as a floating-point number
        println!("result-float: {:?}", data.get("result-float"));
        
        // The full name of the type being converted to
        println!("to-name: {:?}", data.get("to-name"));
        
        // The standard UTF-8 symbol used to represent the type being converted to
        println!("to-symbol: {:?}", data.get("to-symbol"));
        
        // The type being converted to
        println!("to-type: {:?}", data.get("to-type"));
        
        // True if the conversion was successful and produced a valid result
        println!("valid: {:?}", data.get("valid"));
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
