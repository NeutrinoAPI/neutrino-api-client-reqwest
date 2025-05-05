# NeutrinoAPI Rust Reqwest SDK

Neutrino API Rust client using the Reqwest HTTP and Serde JSON libraries

The official API client and SDK built by [NeutrinoAPI](https://www.neutrinoapi.com/)

| Feature          |         |
|------------------|---------|
| Platform Version | >= 1.41 |
| HTTP Library     | Reqwest |
| JSON Library     | Serde   |
| HTTP/2           | Yes     |
| HTTP/3           | No      |
| CodeGen Version  | 4.7.1   |

## Getting started

First you will need a user ID and API key pair: [SignUp](https://www.neutrinoapi.com/signup/)

## To Initialize 
```rust
use neutrino_api_client_reqwest::NeutrinoAPIClient;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
}
```

## Running Examples

```sh
$ cargo run --example="ip_info"
```
You can find examples of all APIs in _examples/_

Set the __'your-user-id'__ and __'your-api-key'__ values in the example to retrieve real API responses

## For Support 
[Contact us](https://www.neutrinoapi.com/contact-us/)
