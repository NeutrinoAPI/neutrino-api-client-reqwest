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
        
        // The domain name of this email address
        println!("domain: {:?}", data.get("domain"));
        
        // True if this address has any domain name or DNS related errors. Check the 'domain-status' field
        // for the detailed error reason
        println!("domain-error: {:?}", data.get("domain-error"));
        
        // The email domain status, possible values are:
        // • ok - the domain is in working order and can receive email
        // • invalid - the domain is not a conformant hostname. May contain invalid syntax or characters
        // • no-service - the domain owner has indicated there is no mail service on the domain (also
        //   known as the 'Null MX')
        // • no-mail - the domain has no valid MX records so cannot receive email
        // • mx-invalid - MX records contain invalid or non-conformant hostname values
        // • mx-bogon - MX records point to bogon IP addresses
        // • resolv-error - MX records do not resolve to any valid IP addresses
        println!("domain-status: {:?}", data.get("domain-status"));
        
        // The complete email address. If you enabled the 'fix-typos' option then this will be the corrected
        // address
        println!("email: {:?}", data.get("email"));
        
        // True if this address is a disposable, temporary or darknet related email address
        println!("is-disposable: {:?}", data.get("is-disposable"));
        
        // True if this address is from a free email provider
        println!("is-freemail: {:?}", data.get("is-freemail"));
        
        // True if this address likely belongs to a person. False if this is a role based address, e.g.
        // admin@, help@, office@, etc.
        println!("is-personal: {:?}", data.get("is-personal"));
        
        // The first resolved IP address of the primary MX server, may be empty if there are domain errors
        // present
        println!("mx-ip: {:?}", data.get("mx-ip"));
        
        // The domain name of the email hosting provider
        println!("provider: {:?}", data.get("provider"));
        
        // True if this address has any syntax errors or is not in RFC compliant formatting
        println!("syntax-error: {:?}", data.get("syntax-error"));
        
        // True if any typos have been fixed. The 'fix-typos' option must be enabled for this to work
        println!("typos-fixed: {:?}", data.get("typos-fixed"));
        
        // Is this a valid email address. To be valid an email must have: correct syntax, a registered and
        // active domain name, correct DNS records and operational MX servers
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
