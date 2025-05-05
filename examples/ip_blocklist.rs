extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // An IPv4 or IPv6 address. Accepts standard IP notation (with or without port number), CIDR
    // notation and IPv6 compressed notation. If multiple IPs are passed using comma-separated values
    // the first non-bogon address on the list will be checked
    params.insert("ip", "104.244.72.115");

    // Include public VPN provider IP addresses. NOTE: For more advanced VPN detection including the
    // ability to identify private and stealth VPNs use the IP Probe API
    params.insert("vpn-lookup", "false");

    let response = client.ip_blocklist(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // An array of strings indicating which blocklist categories this IP is listed on. Current possible
        // values are:
        // • tor - IP is a Tor node or running a Tor related service
        // • proxy - IP has been detected as an anonymous web proxy or HTTP proxy
        // • vpn - IP belongs to a public VPN provider
        // • bot - IP is hosting a malicious bot or is part of a botnet. This is a broad category which
        //   includes brute-force crackers
        // • spam-bot - IP address is hosting a spam bot, comment spamming or any other spamming type
        //   software
        // • exploit-bot - IP is hosting an exploit finding bot or is running exploit scanning software
        // • hijacked - IP is part of a hijacked netblock or a netblock controlled by a criminal
        //   organization
        // • malware - IP is currently involved in distributing or is running malware
        // • spyware - IP is currently involved in distributing or is running spyware
        // • spider - IP is running a hostile web spider / web crawler
        // • dshield - IP has been flagged as a significant attack source by DShield (dshield.org)
        println!("blocklists: {:?}", data.get("blocklists"));
        
        // The CIDR address for this listing (only set if the IP is listed)
        println!("cidr: {:?}", data.get("cidr"));
        
        // The IP address
        println!("ip: {:?}", data.get("ip"));
        
        // IP is hosting a malicious bot or is part of a botnet. This is a broad category which includes
        // brute-force crackers
        println!("is-bot: {:?}", data.get("is-bot"));
        
        // IP has been flagged as a significant attack source by DShield (dshield.org)
        println!("is-dshield: {:?}", data.get("is-dshield"));
        
        // IP is hosting an exploit finding bot or is running exploit scanning software
        println!("is-exploit-bot: {:?}", data.get("is-exploit-bot"));
        
        // IP is part of a hijacked netblock or a netblock controlled by a criminal organization
        println!("is-hijacked: {:?}", data.get("is-hijacked"));
        
        // Is this IP on a blocklist
        println!("is-listed: {:?}", data.get("is-listed"));
        
        // IP is involved in distributing or is running malware
        println!("is-malware: {:?}", data.get("is-malware"));
        
        // IP has been detected as an anonymous web proxy or anonymous HTTP proxy
        println!("is-proxy: {:?}", data.get("is-proxy"));
        
        // IP address is hosting a spam bot, comment spamming or any other spamming type software
        println!("is-spam-bot: {:?}", data.get("is-spam-bot"));
        
        // IP is running a hostile web spider / web crawler
        println!("is-spider: {:?}", data.get("is-spider"));
        
        // IP is involved in distributing or is running spyware
        println!("is-spyware: {:?}", data.get("is-spyware"));
        
        // IP is a Tor node or running a Tor related service
        println!("is-tor: {:?}", data.get("is-tor"));
        
        // IP belongs to a public VPN provider (only set if the 'vpn-lookup' option is enabled)
        println!("is-vpn: {:?}", data.get("is-vpn"));
        
        // The unix time when this IP was last seen on any blocklist. IPs are automatically removed after 7
        // days therefor this value will never be older than 7 days
        println!("last-seen: {:?}", data.get("last-seen"));
        
        // The number of blocklists the IP is listed on
        println!("list-count: {:?}", data.get("list-count"));
        
        // An array of objects containing details on which specific sensors detected the IP
        println!("sensors:");
        let sensors = data.get("sensors").unwrap().as_array().unwrap();
        for item in sensors {
            // The primary blocklist category this sensor belongs to
            println!("    blocklist: {:?}", item.get("blocklist"));
            // Contains details about the sensor source and what type of malicious activity was detected
            println!("    description: {:?}", item.get("description"));
            // The sensor ID. This is a permanent and unique ID for each sensor
            println!("    id: {:?}", item.get("id"));
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
