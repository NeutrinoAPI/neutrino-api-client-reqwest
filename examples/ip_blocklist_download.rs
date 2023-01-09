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
    
    let mut params = HashMap::with_capacity(4);
    
    // The data format. Can be either CSV or TXT
    params.insert("format", "csv");
    
    // Include public VPN provider addresses, this option is only available for Tier 3 or higher
    // accounts. Adds any IPs which are solely listed as VPN providers, IPs that are listed on multiple
    // sensors will still be included without enabling this option. WARNING: This adds at least an
    // additional 8 million IP addresses to the download if not using CIDR notation
    params.insert("include-vpn", "false");
    
    // Output IPs using CIDR notation. This option should be preferred but is off by default for
    // backwards compatibility
    params.insert("cidr", "false");
    
    // Output the IPv6 version of the blocklist, the default is to output IPv4 only. Note that this
    // option enables CIDR notation too as this is the only notation currently supported for IPv6
    params.insert("ip6", "false");

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
