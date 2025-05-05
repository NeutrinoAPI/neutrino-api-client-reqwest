extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(3);

    // An IP address, domain name, FQDN or URL. If you supply a domain/URL it will be checked against
    // the URI DNSBL lists
    params.insert("host", "neutrinoapi.com");

    // Only check lists with this rating or better
    params.insert("list-rating", "3");

    // Only check these DNSBL zones/hosts. Multiple zones can be supplied as comma-separated values
    params.insert("zones", "");

    let response = client.host_reputation(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The IP address or host name
        println!("host: {:?}", data.get("host"));
        
        // Is this host blacklisted
        println!("is-listed: {:?}", data.get("is-listed"));
        
        // The number of DNSBLs the host is listed on
        println!("list-count: {:?}", data.get("list-count"));
        
        // Array of objects for each DNSBL checked
        println!("lists:");
        let lists = data.get("lists").unwrap().as_array().unwrap();
        for item in lists {
            // True if the host is currently black-listed
            println!("    is-listed: {:?}", item.get("is-listed"));
            // The hostname of the DNSBL
            println!("    list-host: {:?}", item.get("list-host"));
            // The name of the DNSBL
            println!("    list-name: {:?}", item.get("list-name"));
            // The list rating [1-3] with 1 being the best rating and 3 the lowest rating
            println!("    list-rating: {:?}", item.get("list-rating"));
            // The DNSBL server response time in milliseconds
            println!("    response-time: {:?}", item.get("response-time"));
            // The specific return code for this listing (only set if listed)
            println!("    return-code: {:?}", item.get("return-code"));
            // The TXT record returned for this listing (only set if listed)
            println!("    txt-record: {:?}", item.get("txt-record"));
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
