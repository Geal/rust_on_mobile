# Steps

## Rust

* Create the Rust library
* export some symbols with libc (for iOS)
* use rusty-cheddar to generate the headers, or write them manually

## iOS with CocoaPods

* use the command `pod lib create InRustWeTrustKit` to create the pod with an example app
* remove the `.git` folder in the pod
* move the podspec file at the root of the project

# Questions

* Should the exported C symbols be in another crate depending on the first?
* how can we generate a static library with apple's bitcode format?
