extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};
use uuid::Uuid;

fn main() {
    let mut output_file_path = temp_dir();
    output_file_path.push(format!("{}-{}.png", "bin-list-download", Uuid::new_v4()));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(4);
    
    // Include ISO 3-letter country codes and ISO 3-letter currency codes in the data. These will be
    // added to columns 10 and 11 respectively
    params.insert("include-iso3", "false");
    
    // Include 8-digit and higher BIN codes. This option includes all 6-digit BINs and all 8-digit and
    // higher BINs (including some 9, 10 and 11 digit BINs where available)
    params.insert("include-8digit", "false");
    
    // Include all BINs and all available fields in the CSV file (overrides any values set for
    // 'include-iso3' or 'include-8digit')
    params.insert("include-all", "false");
    
    // Set this option to 'gzip' to have the output file compressed using gzip
    params.insert("output-encoding", "");

    let response = client.bin_list_download(params, output_file_path);

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
