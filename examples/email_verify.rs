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

    let response = client.email_verify(params);

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
        
        // True if this email domain has a catch-all policy (it will accept mail for any username)
        println!("is-catch-all: {:?}", data.get("is-catch-all"));
        
        // True if the mail server responded with a temporary failure (either a 4xx response code or
        // unresponsive server). You can retry this address later, we recommend waiting at least 15 minutes
        // before retrying
        println!("is-deferred: {:?}", data.get("is-deferred"));
        
        // True if this address is a disposable, temporary or darknet related email address
        println!("is-disposable: {:?}", data.get("is-disposable"));
        
        // True if this address is a free-mail address
        println!("is-freemail: {:?}", data.get("is-freemail"));
        
        // True if this address is for a person. False if this is a role based address, e.g. admin@, help@,
        // office@, etc.
        println!("is-personal: {:?}", data.get("is-personal"));
        
        // The email service provider domain
        println!("provider: {:?}", data.get("provider"));
        
        // The raw SMTP response message received during verification
        println!("smtp-response: {:?}", data.get("smtp-response"));
        
        // The SMTP verification status for the address:
        // • ok - SMTP verification was successful, this is a real address that can receive mail
        // • invalid - this is not a valid email address (has either a domain or syntax error)
        // • absent - this address is not registered with the email service provider
        // • unresponsive - the mail server(s) for this address timed-out or refused to open an SMTP
        //   connection
        // • unknown - sorry, we could not reliably determine the real status of this address (this
        //   address may or may not exist)
        println!("smtp-status: {:?}", data.get("smtp-status"));
        
        // True if this address has a syntax error
        println!("syntax-error: {:?}", data.get("syntax-error"));
        
        // True if typos have been fixed
        println!("typos-fixed: {:?}", data.get("typos-fixed"));
        
        // Is this a valid email address (syntax and domain is valid)
        println!("valid: {:?}", data.get("valid"));
        
        // True if this address has passed SMTP verification. Check the smtp-status and smtp-response fields
        // for specific verification details
        println!("verified: {:?}", data.get("verified"));
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
