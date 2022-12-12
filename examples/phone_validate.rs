extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(3);

    // A phone number. This can be in international format (E.164) or local format. If passing local
    // format you must also set either the 'country-code' OR 'ip' options as well
    params.insert("number", "+6495552000");

    // ISO 2-letter country code, assume numbers are based in this country. If not set numbers are
    // assumed to be in international format (with or without the leading + sign)
    params.insert("country-code", "");

    // Pass in a users IP address and we will assume numbers are based in the country of the IP address
    params.insert("ip", "");

    let response = client.phone_validate(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The phone number country
        println!("country: {:?}", data.get("country"));
        
        // The phone number country as an ISO 2-letter country code
        println!("country-code: {:?}", data.get("country-code"));
        
        // The phone number country as an ISO 3-letter country code
        println!("country-code3: {:?}", data.get("country-code3"));
        
        // ISO 4217 currency code associated with the country
        println!("currency-code: {:?}", data.get("currency-code"));
        
        // The international calling code
        println!("international-calling-code: {:?}", data.get("international-calling-code"));
        
        // The number represented in full international format (E.164)
        println!("international-number: {:?}", data.get("international-number"));
        
        // True if this is a mobile number. If the number type is unknown this value will be false
        println!("is-mobile: {:?}", data.get("is-mobile"));
        
        // The number represented in local dialing format
        println!("local-number: {:?}", data.get("local-number"));
        
        // The phone number location. Could be the city, region or country depending on the type of number
        println!("location: {:?}", data.get("location"));
        
        // The network/carrier who owns the prefix (this only works for some countries, use HLR lookup for
        // global network detection)
        println!("prefix-network: {:?}", data.get("prefix-network"));
        
        // The number type based on the number prefix. Possible values are:
        // • mobile
        // • fixed-line
        // • premium-rate
        // • toll-free
        // • voip
        // • unknown (use HLR lookup)
        println!("type: {:?}", data.get("type"));
        
        // Is this a valid phone number
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
