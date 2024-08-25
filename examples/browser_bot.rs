extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(7);

    // Delay in seconds to wait before capturing any page data, executing selectors or JavaScript
    params.insert("delay", "3");

    // Ignore any TLS/SSL certificate errors and load the page anyway
    params.insert("ignore-certificate-errors", "false");

    // Extract content from the page DOM using this selector. Commonly known as a CSS selector, you can
    // find a good reference here
    params.insert("selector", ".button");

    // The URL to load
    params.insert("url", "https://www.neutrinoapi.com/");

    // Timeout in seconds. Give up if still trying to load the page after this number of seconds
    params.insert("timeout", "30");

    // Execute JavaScript on the website. This parameter accepts JavaScript as either a string
    // containing JavaScript or for sending multiple separate statements a JSON array or POST array can
    // also be used. If a statement returns any value it will be returned in the 'exec-results'
    // response. You can also use the following specially defined user interaction functions:
    // sleep(seconds); Just wait/sleep for the specified number of seconds. click('selector'); Click on
    // the first element matching the given selector. focus('selector'); Focus on the first element
    // matching the given selector. keys('characters'); Send the specified keyboard characters. Use
    // click() or focus() first to send keys to a specific element. enter(); Send the Enter key. tab();
    // Send the Tab key.
    params.insert("exec", "[click('#button-id'), sleep(1), click('.class'), keys('1234'), enter()]");

    // Override the browsers default user-agent string with this one
    params.insert("user-agent", "");

    let response = client.browser_bot(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The complete raw, decompressed and decoded page content. Usually will be either HTML, JSON or XML
        println!("content: {:?}", data.get("content"));
        
        // The size of the returned content in bytes
        println!("content-size: {:?}", data.get("content-size"));
        
        // Array containing all the elements matching the supplied selector
        println!("elements:");
        let elements = data.get("elements").unwrap().as_array().unwrap();
        for item in elements {
        println!();
            // The 'class' attribute of the element
            println!("    class: {:?}", item.get("class"));
            // The 'href' attribute of the element
            println!("    href: {:?}", item.get("href"));
            // The raw HTML of the element
            println!("    html: {:?}", item.get("html"));
            // The 'id' attribute of the element
            println!("    id: {:?}", item.get("id"));
            // The plain-text content of the element with normalized whitespace
            println!("    text: {:?}", item.get("text"));
        }
        
        // Contains the error message if an error has occurred ('is-error' will be true)
        println!("error-message: {:?}", data.get("error-message"));
        
        // If you executed any JavaScript this array holds the results as objects
        println!("exec-results:");
        let exec-results = data.get("exec-results").unwrap().as_array().unwrap();
        for item in exec-results {
        println!();
            // The result of the executed JavaScript statement. Will be empty if the statement returned nothing
            println!("    result: {:?}", item.get("result"));
            // The JavaScript statement that was executed
            println!("    statement: {:?}", item.get("statement"));
        }
        
        // The redirected URL if the URL responded with an HTTP redirect
        println!("http-redirect-url: {:?}", data.get("http-redirect-url"));
        
        // The HTTP status code the URL returned
        println!("http-status-code: {:?}", data.get("http-status-code"));
        
        // The HTTP status message the URL returned
        println!("http-status-message: {:?}", data.get("http-status-message"));
        
        // True if an error has occurred loading the page. Check the 'error-message' field for details
        println!("is-error: {:?}", data.get("is-error"));
        
        // True if the HTTP status is OK (200)
        println!("is-http-ok: {:?}", data.get("is-http-ok"));
        
        // True if the URL responded with an HTTP redirect
        println!("is-http-redirect: {:?}", data.get("is-http-redirect"));
        
        // True if the page is secured using TLS/SSL
        println!("is-secure: {:?}", data.get("is-secure"));
        
        // True if a timeout occurred while loading the page. You can set the timeout with the request
        // parameter 'timeout'
        println!("is-timeout: {:?}", data.get("is-timeout"));
        
        // The ISO 2-letter language code of the page. Extracted from either the HTML document or via HTTP
        // headers
        println!("language-code: {:?}", data.get("language-code"));
        
        // The number of seconds taken to load the page (from initial request until DOM ready)
        println!("load-time: {:?}", data.get("load-time"));
        
        // The document MIME type
        println!("mime-type: {:?}", data.get("mime-type"));
        
        // Map containing all the HTTP response headers the URL responded with
        println!("response-headers: {:?}", data.get("response-headers"));
        
        // Map containing details of the TLS/SSL setup
        println!("security-details: {:?}", data.get("security-details"));
        
        // The HTTP servers hostname (PTR/RDNS record)
        println!("server-hostname: {:?}", data.get("server-hostname"));
        
        // The HTTP servers IP address
        println!("server-ip: {:?}", data.get("server-ip"));
        
        // The document title
        println!("title: {:?}", data.get("title"));
        
        // The requested URL. This may not be the same as the final destination URL, if the URL redirects
        // then it will be set in 'http-redirect-url' and 'is-http-redirect' will also be true
        println!("url: {:?}", data.get("url"));
        
        // Structure of a browser-bot -> url-components response
        println!("url-components: {:?}", data.get("url-components"));
        
        // True if the URL supplied is valid
        println!("url-valid: {:?}", data.get("url-valid"));
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
