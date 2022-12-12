extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(10);

    // The full address, partial address or name of a place to try and locate. Comma separated address
    // components are preferred.
    params.insert("address", "1 Molesworth Street, Thorndon, Wellington 6011");

    // The house/building number to locate
    params.insert("house-number", "");

    // The street/road name to locate
    params.insert("street", "");

    // The city/town name to locate
    params.insert("city", "");

    // The county/region name to locate
    params.insert("county", "");

    // The state name to locate
    params.insert("state", "");

    // The postal code to locate
    params.insert("postal-code", "");

    // Limit result to this country (the default is no country bias)
    params.insert("country-code", "");

    // The language to display results in, available languages are:
    // • de, en, es, fr, it, pt, ru, zh
    params.insert("language-code", "en");

    // If no matches are found for the given address, start performing a recursive fuzzy search until a
    // geolocation is found. This option is recommended for processing user input or implementing
    // auto-complete. We use a combination of approximate string matching and data cleansing to find
    // possible location matches
    params.insert("fuzzy-search", "false");

    let response = client.geocode_address(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The number of possible matching locations found
        println!("found: {:?}", data.get("found"));
        
        // Array of matching location objects
        println!("locations:");
        let locations = data.get("locations").unwrap().as_array().unwrap();
        for item in locations {
        println!();
            // The fully formatted address
            println!("    address: {:?}", item.get("address"));
            // The components which make up the address such as road, city, state, etc
            println!("    address-components: {:?}", item.get("address-components"));
            // The city of the location
            println!("    city: {:?}", item.get("city"));
            // The country of the location
            println!("    country: {:?}", item.get("country"));
            // The ISO 2-letter country code of the location
            println!("    country-code: {:?}", item.get("country-code"));
            // The ISO 3-letter country code of the location
            println!("    country-code3: {:?}", item.get("country-code3"));
            // ISO 4217 currency code associated with the country
            println!("    currency-code: {:?}", item.get("currency-code"));
            // The location latitude
            println!("    latitude: {:?}", item.get("latitude"));
            // Array of strings containing any location tags associated with the address. Tags are additional
            // pieces of metadata about a specific location, there are thousands of different tags. Some
            // examples of tags: shop, office, cafe, bank, pub
            println!("    location-tags: {:?}", item.get("location-tags"));
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
            println!("    location-type: {:?}", item.get("location-type"));
            // The location longitude
            println!("    longitude: {:?}", item.get("longitude"));
            // The postal code for the location
            println!("    postal-code: {:?}", item.get("postal-code"));
            // The state of the location
            println!("    state: {:?}", item.get("state"));
            // Map containing timezone details for the location
            println!("    timezone: {:?}", item.get("timezone"));
        }
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
