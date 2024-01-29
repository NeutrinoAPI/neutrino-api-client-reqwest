extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};
use uuid::Uuid;

fn main() {
    let mut output_file_path = temp_dir();
    output_file_path.push(format!("{}-{}.csv", "ip-blocklist-download", Uuid::new_v4()));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(6);
    
    // The data format. Can be either CSV or TXT
    params.insert("format", "csv");
    
    // Output IPs using CIDR notation. This option should be preferred but is off by default for
    // backwards compatibility
    params.insert("cidr", "false");
    
    // Output the IPv6 version of the blocklist, the default is to output IPv4 only. Note that this
    // option enables CIDR notation too as this is the only notation currently supported for IPv6
    params.insert("ip6", "false");
    
    // The category of IP addresses to include in the download file, possible values are:
    // • all - all IPs available on your current plan (excludes VPN providers for any plans lower than
    //   Tier 3)
    // • bot - all IPs hosting a malicious bot or part of a botnet. This is a broad category which
    //   includes brute-force crackers
    // • exploit-bot - all IPs hosting an exploit finding bot or running exploit scanning software
    // • hijacked - all IPs that are part of a hijacked netblock or a netblock controlled by a
    //   criminal organization
    // • malware - all IPs involved in distributing or running malware
    // • proxy - all IPs detected as an anonymous web proxy or anonymous HTTP proxy
    // • spam-bot - all IPs hosting a spam bot, comment spamming or any other spamming type software
    // • spider - all IPs running a hostile web spider / web crawler
    // • spyware - all IPs involved in distributing or running spyware
    // • tor - all IPs that are Tor nodes or running a Tor related service
    // • vpn - all IPs belonging to public VPN providers (only available for Tier 3 or higher
    //   accounts)
    params.insert("category", "all");
    
    // Set this option to 'gzip' to have the output file compressed using gzip
    params.insert("output-encoding", "");
    
    // Do not download the file but just return the current files MurmurHash3 checksum. You can use this
    // feature to check if the file has changed since a previous check
    params.insert("checksum", "false");

    let response = client.ip_blocklist_download(params, output_file_path);

    if response.file.is_some() {
        let output_file = response.file.unwrap();
        // API request successful, print out the response data
        println!("API Response OK, output saved to: {}", output_file);
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
