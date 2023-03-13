# sparkle-updater

A cross-platform library for adding automatic software updates to your macOS and Windows applications, using the `Sparkle.framework` on macOS and the `WinSparkle.dll` on Windows.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
sparkle-updater = "0.1.0"
```

### macOS

On macOS, you will need to include the `Sparkle.framework` in your application's bundle. You will also need to create an `Info.plist` file for your application, which should include the following keys:

```xml
<key>SUFeedURL</key>
<string>https://example.com/appcast.xml</string>
<key>SUEnableAutomaticChecks</key>
<true/>
<key>SUPublicEDKey</key>
<string>Your Pubkey</string>
```

Replace `https://example.com/appcast.xml` with the URL of your appcast file, which contains information about your app's updates. `SUEnableAutomaticChecks` enables automatic update checks for your app. `SUPublicEDKey` specifies the base64-encoded public EdDSA key.

To use the `sparkle-updater` crate in your Rust code, create a new `Updater` instance:

```rust
use sparkle_updater::Updater;

fn main() {
    let updater = Updater::new();

    updater.check_for_updates();
}
```

### Windows

On Windows, you will need to include the `WinSparkle.dll` in your application's directory, and include the public DSA key in your application's resource file. To do this, create a new .rc file for your application with the following contents:

```
// Public Key
// Used by WinSparkle
// Verify Signature using DSA public key
DSAPub DSAPEM "dsa_pub.pem"
```

This assumes that your DSA public key file is named `dsa_pub.pem`. Make sure to include this file in your app.

To use the `sparkle-updater` crate in your Rust code, create a new `Updater` instance:

```rust
use sparkle_updater::Updater;

fn main() {
    let updater = Updater::new(
        "https://example.com/appcast.xml",
        "Software\\MyApp",
        None,
    );

    updater.check_for_updates();
}
```

The first parameter to `new()` is the URL for the appcast file, while the second parameter is the path in the registry where WinSparkle will store its settings. The third parameter is an optional callback function for handling shutdown requests during updates.

## Thanks

Thanks to the [Sparkle](https://github.com/sparkle-project/Sparkle) project and the [WinSparkle](https://github.com/vslavik/winsparkle) project for creating the libraries that this crate depends on.

The Sparkle provides a robust and easy-to-use solution for adding automatic software updates to macOS applications, while the WinSparkle provides a similar solution for Windows applications.

Without these projects, it would be much more difficult to add automatic update functionality to our applications. We are grateful for the hard work and dedication of the developers who created and maintain these libraries, and we encourage everyone to check out their projects and contribute if possible.

Thank you, Sparkle and WinSparkle teams!

## License

This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.

