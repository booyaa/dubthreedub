// Copyright 2016 Mark Sta Ana.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! A what3words API library written in Rust.
//!
//! # Usage
//!
//! to be completed
//!
//! ```toml
//! [dependencies]
//! dubthreedub = "0.1.*"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate dubthreedub;
//! ```
//!
//! # Example
//!
//! ```rust
//! use dubthreedub;
//! use std::env;
//!
//! let api_key = env::var("W3W_API").expect("Error! Failed to find API key W3W_API!");
//!
//! let lat = 51.5412621;
//! let lng = -0.08813879999999999;
//!
//! let url = dubthreedub::reverse_url(&api_key, &lat, &lng);
//! let response = dubthreedub::call_w3w(&url);
//!
//! println!("raw response: {:?}", response.unwrap());
//! ```

#[macro_use]
extern crate curl;
extern crate json;

use curl::easy::Easy;

const BASE_URL: &'static str = "https://api.what3words.com/v2";

#[derive(Debug,PartialEq)]
pub enum Error {
    InvalidApiKey,
    NoInternet,
    BadUrl,
    BadResponseCode,
}

pub fn reverse_url(api: &str, lat: &f64, lng: &f64) -> String {
    let mut url = String::new();

    url.push_str(BASE_URL);
    url.push_str(&format!("/reverse?coords={}%2C{}", &lat, &lng));
    url.push_str(&format!("&key={}", &api));
    url.push_str("&lang=en&format=json&display=full");

    url.to_string()
}

pub fn call_w3w(url: &str) -> Result<String, Error> {
    let mut handle = Easy::new();
    let mut data = Vec::new();

    try!(handle.url(&url).map_err(|_| Error::BadUrl));
    let _ = handle.fail_on_error(true);
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();

        if transfer.perform().is_err() {
            return Err(Error::NoInternet);
        }
    }

    let data_string = String::from_utf8(data.clone()).unwrap();

    if try!(handle.response_code().map_err(|_| Error::BadResponseCode)) == 401 {
        return Err(Error::InvalidApiKey);
    }

    Ok(data_string.to_string())
}


// #[cfg(test)]
// mod tests {
// TODO: parsing reverse response success
// TODO: parsing reverse response failure
// TODO: integration tests


#[test]
fn integration_test_successful_call() {
    use std::env;
    let api_key = env::var("W3W_API").expect("Error! Failed to find API key W3W_API!");
    assert_eq!(8, api_key.len());

    let lat = 51.5412621;
    let lng = -0.08813879999999999;
    let url = reverse_url(&api_key, &lat, &lng);
    let response = call_w3w(&url);

    assert_eq!(false, response.is_err());
}

#[test]
fn integration_test_unsuccessful_call() {
    let api_key = "ABCDE123";
    assert_eq!(8, api_key.len());

    let lat = 51.5412621;
    let lng = -0.08813879999999999;
    let url = reverse_url(&api_key, &lat, &lng);
    let response = call_w3w(&url);

    assert_eq!(true, response.is_err());
}

// }
