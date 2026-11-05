extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(13);

    // The full address, partial address or name of a place to try and locate. Comma separated address
    // components are preferred.
    params.insert("address", "1 Molesworth Street, Thorndon, Wellington 6011");

    // The house/building number to locate (for structured address searches)
    params.insert("house-number", "");

    // The street/road name to locate (for structured address searches)
    params.insert("street", "");

    // The city/town name to locate (for structured address searches)
    params.insert("city", "");

    // The county/region name to locate (for structured address searches)
    params.insert("county", "");

    // The state name to locate (for structured address searches)
    params.insert("state", "");

    // The postal code to locate (for structured address searches)
    params.insert("postal-code", "");

    // Prefer results from this country code (the default is no country bias)
    params.insert("country-code", "");

    // Prefer results from this list of country codes (space or comma separated values). Preference is
    // in the order supplied, you can use the 'country-code' parameter to also set a dynamic or user
    // supplied country code which takes precedence
    params.insert("country-codes", "");

    // The language to display results in, available languages are:
    // • ar, de, en, es, fr, it, ja, nl, pt, ru, zh
    params.insert("language-code", "en");

    // Enable auto-complete for search-as-you-type and search suggestions. This option can find
    // addresses based on partial search terms, combine with 'country-code' for country specific
    // searches. Auto-complete works best with 'fuzzy-search' disabled for the fastest response times
    // but can be combined if required
    params.insert("auto-complete", "false");

    // If no matches are found for the given address, start performing a recursive fuzzy search until a
    // geolocation is found. This option is useful for processing user supplied input that may be
    // incomplete or have typos and where you do not mind some potential loss in precision
    params.insert("fuzzy-search", "false");

    // Limit search results to this many locations (between 1 and a maximum of 32)
    params.insert("limit", "8");

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
            // The full address string using comma-separated values, use 'postal-address' for the address string
            // formatted using country/region specific rules
            println!("    address: {:?}", item.get("address"));
            // The components which make up the address such as road, city, state, etc. These are country/region
            // specific values, use 'address-structured' for a strict address structure which is the same for
            // all locations globally
            println!("    address-components: {:?}", item.get("address-components"));
            // Structure of address-structured
            println!("    address-structured: {:?}", item.get("address-structured"));
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
            println!("    building-type: {:?}", item.get("building-type"));
            // The city of the location
            println!("    city: {:?}", item.get("city"));
            // ISO 2-letter continent code
            println!("    continent-code: {:?}", item.get("continent-code"));
            // The country of the location
            println!("    country: {:?}", item.get("country"));
            // The ISO 2-letter country code of the location
            println!("    country-code: {:?}", item.get("country-code"));
            // The ISO 3-letter country code of the location
            println!("    country-code3: {:?}", item.get("country-code3"));
            // ISO 4217 currency code associated with the country
            println!("    currency-code: {:?}", item.get("currency-code"));
            // The location geohash
            println!("    geohash: {:?}", item.get("geohash"));
            // The ISO 2-letter language code for the official language spoken in the country
            println!("    language-code: {:?}", item.get("language-code"));
            // The location latitude
            println!("    latitude: {:?}", item.get("latitude"));
            // Map containing any external metadata linked to the location. Currently supports the following
            // values which are only set if known: website-url, phone-number, org-name, wikipedia-title,
            // wikidata-entity
            println!("    location-profile: {:?}", item.get("location-profile"));
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
            // The formatted address using local standards suitable for printing on an envelope
            println!("    postal-address: {:?}", item.get("postal-address"));
            // The postal code for the location
            println!("    postal-code: {:?}", item.get("postal-code"));
            // The ISO 3166-2 region code for the location
            println!("    region-code: {:?}", item.get("region-code"));
            // The state of the location
            println!("    state: {:?}", item.get("state"));
            // Structure of timezone
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
