extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(5);

    // The location latitude in decimal degrees format
    params.insert("latitude", "-41.2775847");

    // The location longitude in decimal degrees format
    params.insert("longitude", "174.7775229");

    // The language to display results in, available languages are:
    // • ar, de, en, es, fr, it, ja, nl, pt, ru, zh
    params.insert("language-code", "en");

    // The zoom level to respond with:
    // • address - the most precise address available
    // • street - the street level
    // • city - the city level
    // • state - the state level
    // • country - the country level
    params.insert("zoom", "address");

    // Reverse lookup by a geohash instead of using latitude and longitude
    params.insert("geohash", "");

    let response = client.geocode_reverse(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The full address string using comma-separated values, use 'postal-address' for the address string
        // formatted using country/region specific rules
        println!("address: {:?}", data.get("address"));
        
        // The components which make up the address such as road, city, state, etc. These are country/region
        // specific values, use 'address-structured' for a strict address structure which is the same for
        // all locations globally
        println!("address-components: {:?}", data.get("address-components"));
        
        // Structure of address-structured
        println!("address-structured: {:?}", data.get("address-structured"));
        
        // The detected building type for the address, possible values are:
        // • none - no building type was detected for this location
        // • house - a residential house/property
        // • apartment - a residential apartment complex
        // • office - a commercial/business office
        // • accommodation - a hotel/motel or other type of accommodation
        // • education - a school or university
        // • food - a restaurant or place where food can be purchased
        // • shop - a shop selling any type of goods
        // • attraction - an attraction such as theme parks or tourist activities
        // • medical - a hospital or other medical facility
        // • transport - a transportation building such as a bus or train depot
        // • commercial - a commercial/industrial building
        // • government - a government or military facility
        // • public - a public building such as a townhall or community centre
        println!("building-type: {:?}", data.get("building-type"));
        
        // The city of the location
        println!("city: {:?}", data.get("city"));
        
        // ISO 2-letter continent code
        println!("continent-code: {:?}", data.get("continent-code"));
        
        // The country of the location
        println!("country: {:?}", data.get("country"));
        
        // The ISO 2-letter country code of the location
        println!("country-code: {:?}", data.get("country-code"));
        
        // The ISO 3-letter country code of the location
        println!("country-code3: {:?}", data.get("country-code3"));
        
        // ISO 4217 currency code associated with the country
        println!("currency-code: {:?}", data.get("currency-code"));
        
        // True if these coordinates map to a real location
        println!("found: {:?}", data.get("found"));
        
        // The location geohash
        println!("geohash: {:?}", data.get("geohash"));
        
        // The ISO 2-letter language code for the official language spoken in the country
        println!("language-code: {:?}", data.get("language-code"));
        
        // The location latitude
        println!("latitude: {:?}", data.get("latitude"));
        
        // Map containing any external metadata linked to the location. Currently supports the following
        // values which are only set if known: website-url, phone-number, org-name, wikipedia-title,
        // wikidata-entity
        println!("location-profile: {:?}", data.get("location-profile"));
        
        // Array of strings containing any location tags associated with the address. Tags are additional
        // pieces of metadata about a specific location, there are thousands of different tags. Some
        // examples of tags: shop, office, cafe, bank, pub
        println!("location-tags: {:?}", data.get("location-tags"));
        
        // The detected location type ordered roughly from most to least precise, possible values are:
        // • address - indicates a precise street address
        // • street - accurate to the street level but may not point to the exact location of the
        //   house/building number
        // • city - accurate to the city level, this includes villages, towns, suburbs, etc
        // • postal-code - indicates a postal code area (no house or street information present)
        // • railway - location is part of a rail network such as a station or railway track
        // • natural - indicates a natural feature, for example a mountain peak or a waterway
        // • island - location is an island or archipelago
        // • administrative - indicates an administrative boundary such as a country, state or province
        println!("location-type: {:?}", data.get("location-type"));
        
        // The location longitude
        println!("longitude: {:?}", data.get("longitude"));
        
        // The formatted address using local standards suitable for printing on an envelope
        println!("postal-address: {:?}", data.get("postal-address"));
        
        // The postal code for the location
        println!("postal-code: {:?}", data.get("postal-code"));
        
        // The ISO 3166-2 region code for the location
        println!("region-code: {:?}", data.get("region-code"));
        
        // The state of the location
        println!("state: {:?}", data.get("state"));
        
        // Structure of timezone
        println!("timezone: {:?}", data.get("timezone"));
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
