// lib.rs
// winsparkle-sys

#![cfg(target_os = "windows")]

// Link to the WinSparkle.dll
#[cfg_attr(target_os = "windows", link(name = "WinSparkle", kind = "dylib"))]
extern "C" {
    /// Initialize WinSparkle.
    ///
    /// This initializes WinSparkle and should be called at application startup.
    pub fn win_sparkle_init();

    /// Clean up after WinSparkle.
    ///
    /// Should be called at application shutdown.
    /// Cancels any pending update checks and shuts down its helper threads.
    pub fn win_sparkle_cleanup();

    /// Set the URL for the appcast file.
    ///
    /// This specifies the URL where WinSparkle will look for updates.
    pub fn win_sparkle_set_appcast_url(url: *const i8);

    /// Set DSA public key.
    ///
    /// Only PEM format is supported.
    /// Public key will be used to verify DSA signatures of the update file.
    /// PEM data will be set only if it contains valid DSA public key.
    ///
    /// If this function isn't called by the app, public key is obtained from
    /// Windows resource named "DSAPub" of type "DSAPEM".
    ///
    /// returns 1 if valid DSA public key provided, 0 otherwise.
    pub fn win_sparkle_set_dsa_pub_pem(dsa_pub_pem: *const i8) -> i32;

    /// Set the path in the registry where WinSparkle will store its settings.
    ///
    /// This sets the path where WinSparkle will store its settings in the registry.
    pub fn win_sparkle_set_registry_path(path: *const i8);

    /// Set the callback function for handling shutdown requests.
    ///
    /// This sets the callback function that WinSparkle will call when it receives a
    /// request to shut down the application during an update.
    pub fn win_sparkle_set_shutdown_request_callback(callback: Option<extern "C" fn() -> ()>);

    /// Check for updates with the WinSparkle UI.
    ///
    /// This checks for updates and displays the WinSparkle UI if an update is available.
    pub fn win_sparkle_check_update_with_ui();

    /// Check for updates with the WinSparkle UI
    /// and immediately install the update if one is available.
    pub fn win_sparkle_check_update_with_ui_and_install();

    /// Check for updates.
    ///
    /// No progress UI is shown to the user when checking.
    /// If an update is available, the usual "update available" UI is shown.
    /// This function is *not* completely UI-less.
    pub fn win_sparkle_check_update_without_ui();
}
