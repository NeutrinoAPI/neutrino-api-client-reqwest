extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(7);

    // The user-agent string to lookup. For client hints use the 'UA' header or the JSON data directly
    // from 'navigator.userAgentData.brands' or 'navigator.userAgentData.getHighEntropyValues()'
    params.insert("ua", "Mozilla/5.0 (Linux; Android 11; SM-G9980U1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.101 Mobile Safari/537.36");

    // For client hints this corresponds to the 'UA-Full-Version' header or 'uaFullVersion' from
    // NavigatorUAData
    params.insert("ua-version", "");

    // For client hints this corresponds to the 'UA-Platform' header or 'platform' from NavigatorUAData
    params.insert("ua-platform", "");

    // For client hints this corresponds to the 'UA-Platform-Version' header or 'platformVersion' from
    // NavigatorUAData
    params.insert("ua-platform-version", "");

    // For client hints this corresponds to the 'UA-Mobile' header or 'mobile' from NavigatorUAData
    params.insert("ua-mobile", "");

    // For client hints this corresponds to the 'UA-Model' header or 'model' from NavigatorUAData. You
    // can also use this parameter to lookup a device directly by its model name, model code or hardware
    // code, on android you can get the model name from:
    // https://developer.android.com/reference/android/os/Build.html#MODEL
    params.insert("device-model", "");

    // This parameter is only used in combination with 'device-model' when doing direct device lookups
    // without any user-agent data. Set this to the brand or manufacturer name, this is required for
    // accurate device detection with ambiguous model names. On android you can get the device brand
    // from: https://developer.android.com/reference/android/os/Build#MANUFACTURER
    params.insert("device-brand", "");

    let response = client.ua_lookup(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // If the client is a web browser which underlying browser engine does it use
        println!("browser-engine: {:?}", data.get("browser-engine"));
        
        // If the client is a web browser which year was this browser version released
        println!("browser-release: {:?}", data.get("browser-release"));
        
        // The device brand / manufacturer
        println!("device-brand: {:?}", data.get("device-brand"));
        
        // The device display height in CSS 'px'
        println!("device-height-px: {:?}", data.get("device-height-px"));
        
        // The device model
        println!("device-model: {:?}", data.get("device-model"));
        
        // The device model code
        println!("device-model-code: {:?}", data.get("device-model-code"));
        
        // The device display pixel ratio (the ratio of the resolution in physical pixels to the resolution
        // in CSS pixels)
        println!("device-pixel-ratio: {:?}", data.get("device-pixel-ratio"));
        
        // The device display PPI (pixels per inch)
        println!("device-ppi: {:?}", data.get("device-ppi"));
        
        // The average device price on release in USD
        println!("device-price: {:?}", data.get("device-price"));
        
        // The year when this device model was released
        println!("device-release: {:?}", data.get("device-release"));
        
        // The device display resolution in physical pixels (e.g. 720x1280)
        println!("device-resolution: {:?}", data.get("device-resolution"));
        
        // The device display width in CSS 'px'
        println!("device-width-px: {:?}", data.get("device-width-px"));
        
        // Is this a mobile device (e.g. a phone or tablet)
        println!("is-mobile: {:?}", data.get("is-mobile"));
        
        // Is this a WebView / embedded software client
        println!("is-webview: {:?}", data.get("is-webview"));
        
        // The client software name
        println!("name: {:?}", data.get("name"));
        
        // The full operating system name
        println!("os: {:?}", data.get("os"));
        
        // The operating system family. The major OS families are: Android, Windows, macOS, iOS, Linux
        println!("os-family: {:?}", data.get("os-family"));
        
        // The operating system full version
        println!("os-version: {:?}", data.get("os-version"));
        
        // The operating system major version
        println!("os-version-major: {:?}", data.get("os-version-major"));
        
        // The user agent type, possible values are:
        // • desktop
        // • phone
        // • tablet
        // • wearable
        // • tv
        // • console
        // • email
        // • library
        // • robot
        // • unknown
        println!("type: {:?}", data.get("type"));
        
        // The user agent string
        println!("ua: {:?}", data.get("ua"));
        
        // The client software full version
        println!("version: {:?}", data.get("version"));
        
        // The client software major version
        println!("version-major: {:?}", data.get("version-major"));
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
