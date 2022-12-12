extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // The BIN or IIN number. This is the first 6, 8 or 10 digits of a card number, use 8 (or more)
    // digits for the highest level of accuracy
    params.insert("bin-number", "47192100");

    // Pass in the customers IP address and we will return some extra information about them
    params.insert("customer-ip", "");

    let response = client.bin_lookup(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The BIN or IIN number
        println!("bin-number: {:?}", data.get("bin-number"));
        
        // The card brand (e.g. Visa or Mastercard)
        println!("card-brand: {:?}", data.get("card-brand"));
        
        // The card category. There are many different card categories the most common card categories are:
        // CLASSIC, BUSINESS, CORPORATE, PLATINUM, PREPAID
        println!("card-category: {:?}", data.get("card-category"));
        
        // The card type, will always be one of: DEBIT, CREDIT, CHARGE CARD
        println!("card-type: {:?}", data.get("card-type"));
        
        // The full country name of the issuer
        println!("country: {:?}", data.get("country"));
        
        // The ISO 2-letter country code of the issuer
        println!("country-code: {:?}", data.get("country-code"));
        
        // The ISO 3-letter country code of the issuer
        println!("country-code3: {:?}", data.get("country-code3"));
        
        // ISO 4217 currency code associated with the country of the issuer
        println!("currency-code: {:?}", data.get("currency-code"));
        
        // True if the customers IP is listed on one of our blocklists, see the IP Blocklist API
        println!("ip-blocklisted: {:?}", data.get("ip-blocklisted"));
        
        // An array of strings indicating which blocklists this IP is listed on
        println!("ip-blocklists: {:?}", data.get("ip-blocklists"));
        
        // The city of the customers IP (if detectable)
        println!("ip-city: {:?}", data.get("ip-city"));
        
        // The country of the customers IP
        println!("ip-country: {:?}", data.get("ip-country"));
        
        // The ISO 2-letter country code of the customers IP
        println!("ip-country-code: {:?}", data.get("ip-country-code"));
        
        // The ISO 3-letter country code of the customers IP
        println!("ip-country-code3: {:?}", data.get("ip-country-code3"));
        
        // True if the customers IP country matches the BIN country
        println!("ip-matches-bin: {:?}", data.get("ip-matches-bin"));
        
        // The region of the customers IP (if detectable)
        println!("ip-region: {:?}", data.get("ip-region"));
        
        // Is this a commercial/business use card
        println!("is-commercial: {:?}", data.get("is-commercial"));
        
        // Is this a prepaid or prepaid reloadable card
        println!("is-prepaid: {:?}", data.get("is-prepaid"));
        
        // The card issuer
        println!("issuer: {:?}", data.get("issuer"));
        
        // The card issuers phone number
        println!("issuer-phone: {:?}", data.get("issuer-phone"));
        
        // The card issuers website
        println!("issuer-website: {:?}", data.get("issuer-website"));
        
        // Is this a valid BIN or IIN number
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
