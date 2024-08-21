extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};
use uuid::Uuid;

fn main() {
    let mut output_file_path = temp_dir();
    output_file_path.push(format!("{}-{}.png", "qr-code", Uuid::new_v4()));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(6);
    
    // The barcode format to output. Accepted formats are: qr, c128
    params.insert("code-format", "qr");
    
    // The width of the QR code (in px)
    params.insert("width", "256");
    
    // The QR code foreground color
    params.insert("fg-color", "#000000");
    
    // The QR code background color
    params.insert("bg-color", "#ffffff");
    
    // The content to encode into the QR code (e.g. a URL or a phone number)
    params.insert("content", "https://www.neutrinoapi.com/signup/");
    
    // The height of the QR code (in px)
    params.insert("height", "256");

    let response = client.qr_code(params, output_file_path);

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
