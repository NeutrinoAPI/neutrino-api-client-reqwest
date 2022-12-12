extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(5);

    // The URL to probe
    params.insert("url", "https://www.neutrinoapi.com/");

    // If this URL responds with html, text, json or xml then return the response. This option is useful
    // if you want to perform further processing on the URL content (e.g. with the HTML Extract or HTML
    // Clean APIs)
    params.insert("fetch-content", "false");

    // Ignore any TLS/SSL certificate errors and load the URL anyway
    params.insert("ignore-certificate-errors", "false");

    // Timeout in seconds. Give up if still trying to load the URL after this number of seconds
    params.insert("timeout", "60");

    // If the request fails for any reason try again this many times
    params.insert("retry", "0");

    let response = client.url_info(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The actual content this URL responded with. Only set if the 'fetch-content' option was used
        println!("content: {:?}", data.get("content"));
        
        // The encoding format the URL uses
        println!("content-encoding: {:?}", data.get("content-encoding"));
        
        // The size of the URL content in bytes
        println!("content-size: {:?}", data.get("content-size"));
        
        // The content-type this URL serves
        println!("content-type: {:?}", data.get("content-type"));
        
        // True if this URL responded with an HTTP OK (200) status
        println!("http-ok: {:?}", data.get("http-ok"));
        
        // True if this URL responded with an HTTP redirect
        println!("http-redirect: {:?}", data.get("http-redirect"));
        
        // The HTTP status code this URL responded with. An HTTP status of 0 indicates a network level issue
        println!("http-status: {:?}", data.get("http-status"));
        
        // The HTTP status message assoicated with the status code
        println!("http-status-message: {:?}", data.get("http-status-message"));
        
        // True if an error occurred while loading the URL. This includes network errors, TLS errors and
        // timeouts
        println!("is-error: {:?}", data.get("is-error"));
        
        // True if a timeout occurred while loading the URL. You can set the timeout with the request
        // parameter 'timeout'
        println!("is-timeout: {:?}", data.get("is-timeout"));
        
        // The ISO 2-letter language code of the page. Extracted from either the HTML document or via HTTP
        // headers
        println!("language-code: {:?}", data.get("language-code"));
        
        // The time taken to load the URL content in seconds
        println!("load-time: {:?}", data.get("load-time"));
        
        // A key-value map of the URL query paramaters
        println!("query: {:?}", data.get("query"));
        
        // Is this URL actually serving real content
        println!("real: {:?}", data.get("real"));
        
        // The servers IP geo-location: full city name (if detectable)
        println!("server-city: {:?}", data.get("server-city"));
        
        // The servers IP geo-location: full country name
        println!("server-country: {:?}", data.get("server-country"));
        
        // The servers IP geo-location: ISO 2-letter country code
        println!("server-country-code: {:?}", data.get("server-country-code"));
        
        // The servers hostname (PTR record)
        println!("server-hostname: {:?}", data.get("server-hostname"));
        
        // The IP address of the server hosting this URL
        println!("server-ip: {:?}", data.get("server-ip"));
        
        // The name of the server software hosting this URL
        println!("server-name: {:?}", data.get("server-name"));
        
        // The servers IP geo-location: full region name (if detectable)
        println!("server-region: {:?}", data.get("server-region"));
        
        // The document title
        println!("title: {:?}", data.get("title"));
        
        // The fully qualified URL. This may be different to the URL requested if http-redirect is true
        println!("url: {:?}", data.get("url"));
        
        // The URL path
        println!("url-path: {:?}", data.get("url-path"));
        
        // The URL port
        println!("url-port: {:?}", data.get("url-port"));
        
        // The URL protocol, usually http or https
        println!("url-protocol: {:?}", data.get("url-protocol"));
        
        // Is this a valid well-formed URL
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
