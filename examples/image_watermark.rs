extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};
use uuid::Uuid;

fn main() {
    let mut output_file_path = temp_dir();
    output_file_path.push(format!("{}-{}.png", "image-watermark", Uuid::new_v4()));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(7);
    
    // The output image format, can be either png or jpg
    params.insert("format", "png");
    
    // If set resize the resulting image to this width (in px) while preserving aspect ratio
    params.insert("width", "");
    
    // The URL or Base64 encoded Data URL for the source image (you can also upload an image file
    // directly in which case this field is ignored)
    params.insert("image-url", "https://www.neutrinoapi.com/img/LOGO.png");
    
    // The position of the watermark image, possible values are: center, top-left, top-center,
    // top-right, bottom-left, bottom-center, bottom-right
    params.insert("position", "center");
    
    // The URL or Base64 encoded Data URL for the watermark image (you can also upload an image file
    // directly in which case this field is ignored)
    params.insert("watermark-url", "https://www.neutrinoapi.com/img/icons/security.png");
    
    // The opacity of the watermark (0 to 100)
    params.insert("opacity", "50");
    
    // If set resize the resulting image to this height (in px) while preserving aspect ratio
    params.insert("height", "");

    let response = client.image_watermark(params, output_file_path);

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
