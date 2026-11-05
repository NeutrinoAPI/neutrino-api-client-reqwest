extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::collections::HashMap;

fn main() {
    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );

    let mut params = HashMap::with_capacity(10);

    // The user-agent string to lookup. For client hints this corresponds to the 'sec-ch-ua' header or
    // you can also pass the JSON data directly from the
    // 'navigator.userAgentData.getHighEntropyValues()' JavaScript API
    params.insert("ua", "Mozilla/5.0 (Linux; Android 11; SM-G9980U1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.101 Mobile Safari/537.36");

    // For client hints this corresponds to the 'sec-ch-ua-full-version' header
    params.insert("ua-version", "");

    // For client hints this corresponds to the 'sec-ch-ua-platform' header
    params.insert("ua-platform", "");

    // For client hints this corresponds to the 'sec-ch-ua-platform-version' header
    params.insert("ua-platform-version", "");

    // For client hints this corresponds to the 'sec-ch-ua-mobile' header
    params.insert("ua-mobile", "");

    // For client hints this corresponds to the 'sec-ch-ua-model' header. You can also use this
    // parameter to lookup a device directly by its model name, model code or hardware code, on android
    // you can get the model name from:
    // https://developer.android.com/reference/android/os/Build.html#MODEL
    params.insert("device-model", "");

    // This parameter is only used in combination with 'device-model' for direct device lookups without
    // any user-agent data. Set this to the brand or manufacturer name, this is required for accurate
    // device detection with ambiguous model names. On android you can get the device brand from:
    // https://developer.android.com/reference/android/os/Build#MANUFACTURER
    params.insert("device-brand", "");

    // For client hints this corresponds to the 'sec-ch-ua-arch' header
    params.insert("ua-arch", "");

    // For client hints this corresponds to the 'sec-ch-ua-bitness' header
    params.insert("ua-bitness", "");

    // For client hints this corresponds to the 'sec-ch-ua-form-factors' header
    params.insert("ua-form-factors", "");

    let response = client.ua_lookup(params);

    if response.data.is_some() {
        let data = response.data.unwrap();
        
        // API request successful, print out the response data
        println!("API Response OK: ");
        
        // For web browsers which underlying browser engine does it use
        println!("browser-engine: {:?}", data.get("browser-engine"));
        
        // The web browser name. For Apps/WebViews this will be the name of the underlying browser used,
        // otherwise this is identical to the 'name' field
        println!("browser-name: {:?}", data.get("browser-name"));
        
        // For web browsers which year was this browser version released
        println!("browser-release: {:?}", data.get("browser-release"));
        
        // For web browsers the ISO date this browser version was released
        println!("browser-release-date: {:?}", data.get("browser-release-date"));
        
        // The browsers full version. For Apps/WebViews this will be the full version of the underlying app
        // browser, otherwise this is identical to the 'version' field
        println!("browser-version: {:?}", data.get("browser-version"));
        
        // The browsers major version. For Apps/WebViews this will be the major version of the underlying
        // app browser, otherwise this is identical to the 'version-major' field
        println!("browser-version-major: {:?}", data.get("browser-version-major"));
        
        // The device brand / manufacturer
        println!("device-brand: {:?}", data.get("device-brand"));
        
        // The device display height in millimeters
        println!("device-height-mm: {:?}", data.get("device-height-mm"));
        
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
        
        // The device display width in millimeters
        println!("device-width-mm: {:?}", data.get("device-width-mm"));
        
        // The device display width in CSS 'px'
        println!("device-width-px: {:?}", data.get("device-width-px"));
        
        // True if this user agent is from a mobile or desktop app. You can distinguish the app name and
        // version from the 'name' and 'version-*' fields and the underlying browser/WebView from the
        // 'browser-*' fields
        println!("is-app: {:?}", data.get("is-app"));
        
        // For web browsers is the version up-to-date with the current stable release (+/- one version)
        println!("is-current: {:?}", data.get("is-current"));
        
        // Is this a mobile device (e.g. a phone or tablet)
        println!("is-mobile: {:?}", data.get("is-mobile"));
        
        // For chrome/chromium based browsers is this user agent string frozen and requires client hints for
        // more accurate details. For more information see user-agent reduction and to test a browser for
        // client hints go to the client hints analyzer test page
        println!("is-ua-frozen: {:?}", data.get("is-ua-frozen"));
        
        // Is this a WebView or embedded web browser. This includes mobile and desktop apps using WebViews
        println!("is-webview: {:?}", data.get("is-webview"));
        
        // The client software name / user agent name / web browser name
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
        
        // The user agent software full version
        println!("version: {:?}", data.get("version"));
        
        // The user agent software major version
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
