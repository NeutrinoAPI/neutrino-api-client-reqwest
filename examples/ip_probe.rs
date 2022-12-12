extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(1);

    // IPv4 or IPv6 address
    params.insert("ip", "194.233.98.38");

    let response = client.ip_probe(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The age of the autonomous system (AS) in number of years since registration
        println!("as-age: {:?}", data.get("as-age"));
        
        // The autonomous system (AS) CIDR range
        println!("as-cidr: {:?}", data.get("as-cidr"));
        
        // The autonomous system (AS) ISO 2-letter country code
        println!("as-country-code: {:?}", data.get("as-country-code"));
        
        // The autonomous system (AS) ISO 3-letter country code
        println!("as-country-code3: {:?}", data.get("as-country-code3"));
        
        // The autonomous system (AS) description / company name
        println!("as-description: {:?}", data.get("as-description"));
        
        // Array of all the domains associated with the autonomous system (AS)
        println!("as-domains: {:?}", data.get("as-domains"));
        
        // The autonomous system (AS) number
        println!("asn: {:?}", data.get("asn"));
        
        // Full city name (if detectable)
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
        
        // The IPs host domain
        println!("host-domain: {:?}", data.get("host-domain"));
        
        // The IPs full hostname (PTR)
        println!("hostname: {:?}", data.get("hostname"));
        
        // The IP address
        println!("ip: {:?}", data.get("ip"));
        
        // True if this is a bogon IP address such as a private network, local network or reserved address
        println!("is-bogon: {:?}", data.get("is-bogon"));
        
        // True if this IP belongs to a hosting company. Note that this can still be true even if the
        // provider type is VPN/proxy, this occurs in the case that the IP is detected as both types
        println!("is-hosting: {:?}", data.get("is-hosting"));
        
        // True if this IP belongs to an internet service provider. Note that this can still be true even if
        // the provider type is VPN/proxy, this occurs in the case that the IP is detected as both types
        println!("is-isp: {:?}", data.get("is-isp"));
        
        // True if this IP ia a proxy
        println!("is-proxy: {:?}", data.get("is-proxy"));
        
        // True if this is a IPv4 mapped IPv6 address
        println!("is-v4-mapped: {:?}", data.get("is-v4-mapped"));
        
        // True if this is a IPv6 address. False if IPv4
        println!("is-v6: {:?}", data.get("is-v6"));
        
        // True if this IP ia a VPN
        println!("is-vpn: {:?}", data.get("is-vpn"));
        
        // A description of the provider (usually extracted from the providers website)
        println!("provider-description: {:?}", data.get("provider-description"));
        
        // The domain name of the provider
        println!("provider-domain: {:?}", data.get("provider-domain"));
        
        // The detected provider type, possible values are:
        // • isp - IP belongs to an internet service provider. This includes both mobile, home and
        //   business internet providers
        // • hosting - IP belongs to a hosting company. This includes website hosting, cloud computing
        //   platforms and colocation facilities
        // • vpn - IP belongs to a VPN provider
        // • proxy - IP belongs to a proxy service. This includes HTTP/SOCKS proxies and browser based
        //   proxies
        // • university - IP belongs to a university/college/campus
        // • government - IP belongs to a government department. This includes military facilities
        // • commercial - IP belongs to a commercial entity such as a corporate headquarters or company
        //   office
        // • unknown - could not identify the provider type
        println!("provider-type: {:?}", data.get("provider-type"));
        
        // The website URL for the provider
        println!("provider-website: {:?}", data.get("provider-website"));
        
        // Full region name (if detectable)
        println!("region: {:?}", data.get("region"));
        
        // ISO 3166-2 region code (if detectable)
        println!("region-code: {:?}", data.get("region-code"));
        
        // True if this is a valid IPv4 or IPv6 address
        println!("valid: {:?}", data.get("valid"));
        
        // The domain of the VPN provider (may be empty if the VPN domain is not detectable)
        println!("vpn-domain: {:?}", data.get("vpn-domain"));
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
