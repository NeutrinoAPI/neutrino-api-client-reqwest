extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};
use uuid::Uuid;

fn main() {
    let mut output_file_path = temp_dir();
    output_file_path.push(format!("{}-{}.txt", "html-clean", Uuid::new_v4()));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(2);
    
    // The level of sanitization, possible values are: plain-text: reduce the content to plain text only
    // (no HTML tags at all) simple-text: allow only very basic text formatting tags like b, em, i,
    // strong, u basic-html: allow advanced text formatting and hyper links basic-html-with-images: same
    // as basic html but also allows image tags advanced-html: same as basic html with images but also
    // allows many more common HTML tags like table, ul, dl, pre
    params.insert("output-type", "plain-text");
    
    // The HTML content. This can be either a URL to load from, a file upload or an HTML content string
    params.insert("content", "<div>Some HTML to clean...</div><script>alert()</script>");

    let response = client.html_clean(params, output_file_path);

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
