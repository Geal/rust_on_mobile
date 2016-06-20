# Steps

## Rust

* Create the Rust library
* export some symbols with libc (for iOS)
* use rusty-cheddar to generate the headers, or write them manually


# Questions

* Should the exported C symbols be in another crate depending on the first?
* how can we generate a static library with apple's bitcode format?
