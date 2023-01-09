extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(3);

    // The character to use to censor out the bad words found
    params.insert("censor-character", "");

    // Which catalog of bad words to use, we currently maintain two bad word catalogs:
    // • strict - the largest database of bad words which includes profanity, obscenity, sexual, rude,
    //   cuss, dirty, swear and objectionable words and phrases. This catalog is suitable for
    //   environments of all ages including educational or children's content
    // • obscene - like the strict catalog but does not include any mild profanities, idiomatic
    //   phrases or words which are considered formal terminology. This catalog is suitable for adult
    //   environments where certain types of bad words are considered OK
    params.insert("catalog", "strict");

    // The content to scan. This can be either a URL to load from, a file upload (multipart/form-data)
    // or an HTML content string
    params.insert("content", "https://en.wikipedia.org/wiki/Profanity");

    let response = client.bad_word_filter(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // An array of the bad words found
        println!("bad-words-list: {:?}", data.get("bad-words-list"));
        
        // Total number of bad words detected
        println!("bad-words-total: {:?}", data.get("bad-words-total"));
        
        // The censored content (only set if censor-character has been set)
        println!("censored-content: {:?}", data.get("censored-content"));
        
        // Does the text contain bad words
        println!("is-bad: {:?}", data.get("is-bad"));
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
