# NeutrinoAPI Rust Reqwest SDK

Rust client using the Reqwest HTTP client and Serde JSON library

The official API client and SDK built by [NeutrinoAPI](https://www.neutrinoapi.com/)

| Feature          |         |
|------------------|---------|
| Platform Version | >= 1.41 |
| HTTP Library     | Reqwest |
| JSON Library     | Serde   |
| HTTP/2           | No      |
| HTTP/3           | No      |
| CodeGen Version  | 4.6.12  |

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

## For Support 
[Contact us](https://www.neutrinoapi.com/contact-us/)
