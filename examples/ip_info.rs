extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // IPv4 or IPv6 address
    params.insert("ip", "1.1.1.1");

    // Do a reverse DNS (PTR) lookup. This option can add extra delay to the request so only use it if
    // you need it
    params.insert("reverse-lookup", "false");

    let response = client.ip_info(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // Name of the city (if detectable)
        println!("city: {:?}", data.get("city"));
        
        // ISO 2-letter continent code
        println!("continent-code: {:?}", data.get("continent-code"));
        
        // Full country name
        println!("country: {:?}", data.get("country"));
        
        // ISO 2-letter country code
        println!("country-code: {:?}", data.get("country-code"));
        
        // ISO 3-letter country code
        println!("country-code3: {:?}", data.get("country-code3"));
        
        // ISO 4217 currency code associated with the country
        println!("currency-code: {:?}", data.get("currency-code"));
        
        // The IPs host domain (only set if reverse-lookup has been used)
        println!("host-domain: {:?}", data.get("host-domain"));
        
        // The IPs full hostname (only set if reverse-lookup has been used)
        println!("hostname: {:?}", data.get("hostname"));
        
        // An IPv4 or IPv6 address. Accepts standard IP notation and also CIDR notation.
        println!("ip: {:?}", data.get("ip"));
        
        // True if this is a bogon IP address such as a private network, local network or reserved address
        println!("is-bogon: {:?}", data.get("is-bogon"));
        
        // True if this is a IPv4 mapped IPv6 address
        println!("is-v4-mapped: {:?}", data.get("is-v4-mapped"));
        
        // True if this is a IPv6 address. False if IPv4
        println!("is-v6: {:?}", data.get("is-v6"));
        
        // Location latitude
        println!("latitude: {:?}", data.get("latitude"));
        
        // Location longitude
        println!("longitude: {:?}", data.get("longitude"));
        
        // Name of the region (if detectable)
        println!("region: {:?}", data.get("region"));
        
        // ISO 3166-2 region code (if detectable)
        println!("region-code: {:?}", data.get("region-code"));
        
        // Map containing timezone details
        println!("timezone: {:?}", data.get("timezone"));
        
        // True if this is a valid IPv4 or IPv6 address
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
