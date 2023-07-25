extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // A domain name, hostname, FQDN, URL, HTML link or email address to lookup
    params.insert("host", "neutrinoapi.com");

    // For domains that we have never seen before then perform various live checks and realtime
    // reconnaissance. NOTE: this option may add additional non-deterministic delay to the request, if
    // you require consistently fast API response times or just want to check our domain blocklists then
    // you can disable this option
    params.insert("live", "true");

    let response = client.domain_lookup(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The number of days since the domain was registered. A domain age of under 90 days is generally
        // considered to be potentially risky. A value of 0 indicates no registration date was found for
        // this domain
        println!("age: {:?}", data.get("age"));
        
        // An array of strings indicating which blocklist categories this domain is listed on. Current
        // categories are: phishing, malware, spam, anonymizer, nefarious
        println!("blocklists: {:?}", data.get("blocklists"));
        
        // The primary domain of the DNS provider for this domain
        println!("dns-provider: {:?}", data.get("dns-provider"));
        
        // The primary domain name excluding any subdomains. This is also referred to as the second-level
        // domain (SLD)
        println!("domain: {:?}", data.get("domain"));
        
        // The fully qualified domain name (FQDN)
        println!("fqdn: {:?}", data.get("fqdn"));
        
        // This domain is hosting adult content such as porn, webcams, escorts, etc
        println!("is-adult: {:?}", data.get("is-adult"));
        
        // Is this domain under a government or military TLD
        println!("is-gov: {:?}", data.get("is-gov"));
        
        // Consider this domain malicious as it is currently listed on at least 1 blocklist
        println!("is-malicious: {:?}", data.get("is-malicious"));
        
        // Is this domain under an OpenNIC TLD
        println!("is-opennic: {:?}", data.get("is-opennic"));
        
        // True if this domain is unseen and is currently being processed in the background. This field only
        // matters when the 'live' lookup setting has been explicitly disabled and indicates that not all
        // domain data my be present yet
        println!("is-pending: {:?}", data.get("is-pending"));
        
        // Is the FQDN a subdomain of the primary domain
        println!("is-subdomain: {:?}", data.get("is-subdomain"));
        
        // The primary domain of the email provider for this domain. An empty value indicates the domain has
        // no valid MX records
        println!("mail-provider: {:?}", data.get("mail-provider"));
        
        // The domains estimated global traffic rank with the highest rank being 1. A value of 0 indicates
        // the domain is currently ranked outside of the top 1M of domains
        println!("rank: {:?}", data.get("rank"));
        
        // The ISO date this domain was registered or first seen on the internet. An empty value indicates
        // we could not reliably determine the date
        println!("registered-date: {:?}", data.get("registered-date"));
        
        // The IANA registrar ID (0 if no registrar ID was found)
        println!("registrar-id: {:?}", data.get("registrar-id"));
        
        // The name of the domain registrar owning this domain
        println!("registrar-name: {:?}", data.get("registrar-name"));
        
        // An array of objects containing details on which specific blocklist sensors have detected this
        // domain
        println!("sensors:");
        let sensors = data.get("sensors").unwrap().as_array().unwrap();
        for item in sensors {
        println!();
            // The primary blocklist category this sensor belongs to
            println!("    blocklist: {:?}", item.get("blocklist"));
            // Contains details about the sensor source and what type of malicious activity was detected
            println!("    description: {:?}", item.get("description"));
            // The sensor ID. This is a permanent and unique ID for each sensor
            println!("    id: {:?}", item.get("id"));
        }
        
        // The top-level domain (TLD)
        println!("tld: {:?}", data.get("tld"));
        
        // For a country code top-level domain (ccTLD) this will contain the associated ISO 2-letter country
        // code
        println!("tld-cc: {:?}", data.get("tld-cc"));
        
        // True if a valid domain was found. For a domain to be considered valid it must be registered and
        // have valid DNS NS records
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
