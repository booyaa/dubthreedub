# dub three dub

| travis-ci | appveyor |
|-----------|----------|
| [![Build Status](https://travis-ci.org/booyaa/dubthreedub.svg?branch=master)](https://travis-ci.org/booyaa/dubthreedub) | [![Build Status](https://ci.appveyor.com/api/projects/status/github/booyaa/dubthreedub)] |


A what3words API library written in Rust.

Warning: this is a partial implemented API. I only need the /reverse endpoint. PRs welcome if you want to implement other endpoints.

# Documentation

- [doc.rs](https://docs.rs/dubthreedub) ![badge](https://docs.rs/dubthreedub/badge.svg)
- [github fallback](https://booyaa.github.io/dubthreedub/dubthreedub/index.html)

# Usage

```toml
[dependencies]
dubthreedub = "0.1"
```

and this to your crate root:

```rust
extern crate dubthreedub;
```

# Example

```rust
use dubthreedub;
use std::env;

let api_key = env::var("W3W_API").expect("Error! Failed to find API key W3W_API!");

let lat = 51.5412621;
let lng = -0.08813879999999999;

let url = dubthreedub::reverse_url(&api_key, &lat, &lng);
let response = dubthreedub::call_w3w(&url);

println!("raw response: {:?}", response.unwrap());
```

# Copyright

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
