# winsparkle-sys

Rust bindings for the `WinSparkle.dll` on Windows.

This crate provides a wrapper for the WinSparkle dynamic-link library (DLL), allowing you to add automatic software updates to your Windows applications.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
winsparkle-sys = "0.1.0"
```

Then, in your Rust code, call the various functions provided by the WinSparkle DLL:

```rust
use winsparkle_sys::{
    win_sparkle_check_update_with_ui, win_sparkle_init, win_sparkle_set_appcast_url,
};

fn main() {
    unsafe {
        win_sparkle_set_appcast_url("https://example.com/appcast.xml\0".as_ptr() as *const i8);
        win_sparkle_init();
        win_sparkle_check_update_with_ui();
    }
}
```

Note that this crate only provides bindings for the WinSparkle DLL, so you will need to include the DLL in your Windows application separately. Additionally, to use WinSparkle in your Windows app, you need to include the DLL in your app's directory, and call the functions provided by the DLL at the appropriate times.

In addition, you will need to include the public DSA key used to sign your appcast file in your application's resource file. This is typically done by including the following line in your application's `.rc` file:

```
// Public Key
// Used by WinSparkle
// Verify Signature using DSA public key
DSAPub DSAPEM "dsa_pub.pem"
```

This assumes that your DSA public key file is named dsa_pub.pem. Make sure to include this file in your app.

For more information about using WinSparkle in your Windows app, see the [WinSparkle documentation](https://github.com/vslavik/winsparkle/blob/master/include/winsparkle.h).

## License

This crate is licensed under the MIT license. See the [LICENSE](../LICENSE) file for details.
