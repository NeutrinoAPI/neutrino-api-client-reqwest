extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(2);

    // An email address
    params.insert("email", "tech@neutrinoapi.com");

    // Automatically attempt to fix typos in the address
    params.insert("fix-typos", "false");

    let response = client.email_validate(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // The email domain
        println!("domain: {:?}", data.get("domain"));
        
        // True if this address has a domain error (e.g. no valid mail server records)
        println!("domain-error: {:?}", data.get("domain-error"));
        
        // The email address. If you have used the fix-typos option then this will be the fixed address
        println!("email: {:?}", data.get("email"));
        
        // True if this address is a disposable, temporary or darknet related email address
        println!("is-disposable: {:?}", data.get("is-disposable"));
        
        // True if this address is a free-mail address
        println!("is-freemail: {:?}", data.get("is-freemail"));
        
        // True if this address belongs to a person. False if this is a role based address, e.g. admin@,
        // help@, office@, etc.
        println!("is-personal: {:?}", data.get("is-personal"));
        
        // The email service provider domain
        println!("provider: {:?}", data.get("provider"));
        
        // True if this address has a syntax error
        println!("syntax-error: {:?}", data.get("syntax-error"));
        
        // True if typos have been fixed
        println!("typos-fixed: {:?}", data.get("typos-fixed"));
        
        // Is this a valid email
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
