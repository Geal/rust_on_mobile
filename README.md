# Steps

## Rust

* Create the Rust library
* export some symbols with libc (for iOS)
* use rusty-cheddar to generate the headers, or write them manually

## Build for iOS

* install `cargo-lipo`: `cargo install cargo-lipo`
* install the required toolchains:
  * rustup target add aarch64-apple-ios
  * rustup target add armv7-apple-ios
  * rustup target add armv7s-apple-ios
  * rustup target add i386-apple-ios
  * rustup target add x86_64-apple-ios

## iOS with CocoaPods

* install the cocoapods: `sudo gem install cocoapods` (cf https://cocoapods.org/ )
* use the command `pod lib create InRustWeTrustKit` to create the pod with an example app
* remove the `.git` folder in the pod
* move the podspec file at the root of the project
* change `InRustWeTrustKit/Example/Podfile` to point to the pod at `../../` instead of `../`
* update the `source_files` path in the podspec from `InRustWeTrustKit/Classes/**/*` to `InRustWeTrustKit/InRustWeTrustKit/Classes/**/*`
* add the `prepare_command` in the podspec file to build the library
* use `pod lib lint --verbose` to verify the podspec file
* `cd InRustWeTrustKit/Example && pod install --verbose` to test building with the example app

CocoaPods have a lot of requirements to push them to the repo, like having a valid
LICENSE file, making a different branch for every version, and storing every podspec
file in a github repository, so be prepared to spend some time fighting those issues.

## Build for Android

* download the Android NDK
* Create the `rust-jni` project to host the JNI interface: `cargo new rust-jni`. It will generate a dylib
* add the original project as dependency by path
* create the JNI export functions
* write a `.cargo/config` file with the following content (adjust the paths, platform and SDK version accordingly):
  [target.arm-linux-androideabi]
  ar = "/usr/local/Cellar/android-ndk/r11c/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar"
  linker = "/Users/geal/dev/rust_on_mobile/rust-jni/linker-wrapper.sh"
* the `rust-jni/linker-wrapper.sh` file was a ugly hack I needed to link when a link option was unavailable with the linker version, not sure it's still needed
* `cargo build --target=arm-linux-androideabi`
* `cp target/arm-linux-androideabi/debug/libinrustwetrust.so ../android/src/main/jniLibs/armeabi/libinrustwetrust.so`
* `cp target/arm-linux-androideabi/debug/libinrustwetrust.so ../android/src/main/jniLibs/armeabi/libinrustwetrust.so

## Android plugin with Gradle

* create an Android library project with Android studio, put it in the `android/` directory


# Questions

* Should the exported C symbols be in another crate depending on the first?
* how can we generate a static library with apple's bitcode format?
* everything is done manually to write JNI functions right now, could it be automated a bit more?

