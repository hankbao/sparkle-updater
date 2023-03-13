# sparkle-sys

Rust bindings for the Sparkle.framework on macOS.

This crate provides a wrapper for the SPUStandardUpdaterController class in Sparkle.framework, allowing you to check for updates in your macOS applications.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
sparkle-sys = "0.1.0"
```

Then, in your Rust code, create a new `SPUStandardUpdaterController` and call `check_for_updates` to check for updates:

```rust
use sparkle_sys::SPUStandardUpdaterController;

let updater = SPUStandardUpdaterController::new();
updater.check_for_updates();
```

Note that this crate only provides bindings for Sparkle.framework, so you will need to include the framework in your macOS application separately. Additionally, to use Sparkle in your macOS app, you need to add certain settings to your Info.plist file:

```xml
<key>SUFeedURL</key>
<string>https://example.com/appcast.xml</string>
<key>SUEnableAutomaticChecks</key>
<true/>
<key>SUPublicEDKey</key>
<string>Your Pubkey</string>
```

Replace `https://example.com/appcast.xml` with the URL of your appcast file, which contains information about your app's updates. `SUEnableAutomaticChecks` enables automatic update checks for your app. `SUPublicEDKey` specifies the base64-encoded public EdDSA key.

For more information about these and other settings, see the [Sparkle documentation](https://sparkle-project.org/documentation/customization/).

## License

This crate is licensed under the MIT license. See the [LICENSE](../LICENSE) file for details.
