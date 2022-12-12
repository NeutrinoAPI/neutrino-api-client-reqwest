extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // A phone number
    params.insert("number", "+12106100045");

    // ISO 2-letter country code, assume numbers are based in this country. If not set numbers are
    // assumed to be in international format (with or without the leading + sign)
    params.insert("country-code", "");

    let response = client.hlr_lookup(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The phone number country
        println!("country: {:?}", data.get("country"));
        
        // The number location as an ISO 2-letter country code
        println!("country-code: {:?}", data.get("country-code"));
        
        // The number location as an ISO 3-letter country code
        println!("country-code3: {:?}", data.get("country-code3"));
        
        // ISO 4217 currency code associated with the country
        println!("currency-code: {:?}", data.get("currency-code"));
        
        // The currently used network/carrier name
        println!("current-network: {:?}", data.get("current-network"));
        
        // The HLR lookup status, possible values are:
        // • ok - the HLR lookup was successful and the device is connected
        // • absent - the number was once registered but the device has been switched off or out of
        //   network range for some time
        // • unknown - the number is not known by the mobile network
        // • invalid - the number is not a valid mobile MSISDN number
        // • fixed-line - the number is a registered fixed-line not mobile
        // • voip - the number has been detected as a VOIP line
        // • failed - the HLR lookup has failed, we could not determine the real status of this number
        println!("hlr-status: {:?}", data.get("hlr-status"));
        
        // Was the HLR lookup successful. If true then this is a working and registered cell-phone or mobile
        // device (SMS and phone calls will be delivered)
        println!("hlr-valid: {:?}", data.get("hlr-valid"));
        
        // The mobile IMSI number (International Mobile Subscriber Identity)
        println!("imsi: {:?}", data.get("imsi"));
        
        // The international calling code
        println!("international-calling-code: {:?}", data.get("international-calling-code"));
        
        // The number represented in full international format
        println!("international-number: {:?}", data.get("international-number"));
        
        // True if this is a mobile number (only true with 100% certainty, if the number type is unknown
        // this value will be false)
        println!("is-mobile: {:?}", data.get("is-mobile"));
        
        // Has this number been ported to another network
        println!("is-ported: {:?}", data.get("is-ported"));
        
        // Is this number currently roaming from its origin country
        println!("is-roaming: {:?}", data.get("is-roaming"));
        
        // The number represented in local dialing format
        println!("local-number: {:?}", data.get("local-number"));
        
        // The number location. Could be a city, region or country depending on the type of number
        println!("location: {:?}", data.get("location"));
        
        // The mobile MCC number (Mobile Country Code)
        println!("mcc: {:?}", data.get("mcc"));
        
        // The mobile MNC number (Mobile Network Code)
        println!("mnc: {:?}", data.get("mnc"));
        
        // The mobile MSC number (Mobile Switching Center)
        println!("msc: {:?}", data.get("msc"));
        
        // The mobile MSIN number (Mobile Subscription Identification Number)
        println!("msin: {:?}", data.get("msin"));
        
        // The number type, possible values are:
        // • mobile
        // • fixed-line
        // • premium-rate
        // • toll-free
        // • voip
        // • unknown
        println!("number-type: {:?}", data.get("number-type"));
        
        // True if this a valid phone number
        println!("number-valid: {:?}", data.get("number-valid"));
        
        // The origin network/carrier name
        println!("origin-network: {:?}", data.get("origin-network"));
        
        // The ported to network/carrier name (only set if the number has been ported)
        println!("ported-network: {:?}", data.get("ported-network"));
        
        // If the number is currently roaming, the ISO 2-letter country code of the roaming in country
        println!("roaming-country-code: {:?}", data.get("roaming-country-code"));
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
