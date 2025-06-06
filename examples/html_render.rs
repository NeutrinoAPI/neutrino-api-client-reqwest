extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use std::{collections::HashMap, env::temp_dir};

fn main() {
    let mut output_file_path = temp_dir();
    let output_file_timestamp = std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos();
    output_file_path.push(format!("{}-{}.pdf", "html-render", output_file_timestamp));
    let output_file_path = output_file_path.to_path_buf();

    let client = &mut NeutrinoAPIClient::new(
        "<your-user-id>",
        "<your-api-key>",
    );
    
    let mut params = HashMap::with_capacity(25);
    
    // Inject custom CSS into the HTML. e.g. 'body { background-color: red;}'
    params.insert("css", "");
    
    // The footer HTML to insert into each page. The following dynamic tags are supported: {date},
    // {title}, {url}, {pageNumber}, {totalPages}
    params.insert("footer", "");
    
    // The document title
    params.insert("title", "");
    
    // The HTML content. This can be either a URL to load from, a file upload (multipart/form-data) or
    // an HTML content string
    params.insert("content", "<h1>TEST DOCUMENT</h1><p>Hello, this is a test page...</p>");
    
    // Set the PDF page width explicitly (in mm)
    params.insert("page-width", "");
    
    // Timeout in seconds. Give up if still trying to load the HTML content after this number of seconds
    params.insert("timeout", "300");
    
    // Render the final document in grayscale
    params.insert("grayscale", "false");
    
    // The document left margin (in mm)
    params.insert("margin-left", "0");
    
    // Set the document page size, can be one of: A0 - A9, B0 - B10, Comm10E, DLE or Letter
    params.insert("page-size", "A4");
    
    // Ignore any TLS/SSL certificate errors
    params.insert("ignore-certificate-errors", "false");
    
    // Set the PDF page height explicitly (in mm)
    params.insert("page-height", "");
    
    // The document top margin (in mm)
    params.insert("margin-top", "0");
    
    // For image rendering set the background color in hexadecimal notation (e.g. #0000ff). For PNG
    // output the special value of 'transparent' can be used to create a transparent PNG
    params.insert("bg-color", "");
    
    // The document margin (in mm)
    params.insert("margin", "0");
    
    // If rendering to an image format (PNG or JPG) use this image width (in pixels)
    params.insert("image-width", "1024");
    
    // Which format to output, available options are: PDF, PNG, JPG
    params.insert("format", "PDF");
    
    // Set the zoom factor when rendering the page (2.0 for double size, 0.5 for half size)
    params.insert("zoom", "1");
    
    // The document right margin (in mm)
    params.insert("margin-right", "0");
    
    // Number of seconds to wait before rendering the page (can be useful for pages with animations etc)
    params.insert("delay", "0");
    
    // If rendering to an image format (PNG or JPG) use this image height (in pixels). The default is
    // automatic which dynamically sets the image height based on the content
    params.insert("image-height", "");
    
    // The header HTML to insert into each page. The following dynamic tags are supported: {date},
    // {title}, {url}, {pageNumber}, {totalPages}
    params.insert("header", "<div style='width: 100%; font-size: 8pt;'>{pageNumber} of {totalPages} - {date}</div>");
    
    // The document bottom margin (in mm)
    params.insert("margin-bottom", "0");
    
    // Set the document to landscape orientation
    params.insert("landscape", "false");
    
    // Execute JavaScript on the website. This parameter accepts JavaScript as either a string
    // containing JavaScript or for sending multiple separate statements a JSON array or POST array can
    // also be used. You can also use the following specially defined user interaction functions:
    // sleep(seconds); Just wait/sleep for the specified number of seconds. click('selector'); Click on
    // the first element matching the given selector. focus('selector'); Focus on the first element
    // matching the given selector. keys('characters'); Send the specified keyboard characters. Use
    // click() or focus() first to send keys to a specific element. enter(); Send the Enter key. tab();
    // Send the Tab key.
    params.insert("exec", "");
    
    // Override the browsers default user-agent string with this one
    params.insert("user-agent", "");

    let response = client.html_render(params, output_file_path);

    if response.file.is_some() {
        let output_file = response.file.unwrap();
        // API request successful, print out the response data
        println!("API Response OK, output saved to: {}", output_file);
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
