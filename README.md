# Steps

## Rust

* Create the Rust library
* export some symbols with libc (for iOS)
* use rusty-cheddar to generate the headers, or write them manually

## iOS with CocoaPods

* use the command `pod lib create InRustWeTrustKit` to create the pod with an example app
* remove the `.git` folder in the pod
* move the podspec file at the root of the project
* change `InRustWeTrustKit/Example/Podfile` to point to the pod at `../../` instead of `../`
* update the `source_files` path in the podspec from `InRustWeTrustKit/Classes/**/*` to `InRustWeTrustKit/InRustWeTrustKit/Classes/**/*`
* install `cargo-lipo`: `cargo install cargo-lipo`
* install the required toolchains:
  * rustup target add aarch64-apple-ios
  * rustup target add armv7-apple-ios
  * rustup target add armv7s-apple-ios
  * rustup target add i386-apple-ios
  * rustup target add x86_64-apple-ios
* add the `prepare_command` in the podspec file to build the library
* use `pod lib lint --verbose` to verify the podspec file
* `cd InRustWeTrustKit/Example && pod install --verbose` to test building with the example app

CocoaPods have a lot of requirements to push them to the repo, like having a valid
LICENSE file, making a different branch for every version, and storing every podspec
file in a github repository, so be prepared to spend some time fighting those issues.

# Questions

* Should the exported C symbols be in another crate depending on the first?
* how can we generate a static library with apple's bitcode format?
