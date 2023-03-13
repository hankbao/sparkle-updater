// lib.rs
// winsparkle-sys

#![cfg(target_os = "windows")]

use std::ffi::c_void;

// Link to the WinSparkle.dll
#[cfg_attr(target_os = "windows", link(name = "WinSparkle", kind = "dylib"))]
extern "C" {
    /// Set the URL for the appcast file.
    ///
    /// This specifies the URL where WinSparkle will look for updates.
    pub fn win_sparkle_set_appcast_url(url: *const i8) -> c_void;

    /// Set the path in the registry where WinSparkle will store its settings.
    ///
    /// This sets the path where WinSparkle will store its settings in the registry.
    pub fn win_sparkle_set_registry_path(path: *const i8) -> c_void;

    /// Set the callback function for handling shutdown requests.
    ///
    /// This sets the callback function that WinSparkle will call when it receives a
    /// request to shut down the application during an update. The callback should
    /// return `c_void`.
    pub fn win_sparkle_set_shutdown_request_callback(
        callback: Option<extern "C" fn() -> c_void>,
    ) -> c_void;

    /// Initialize WinSparkle.
    ///
    /// This initializes WinSparkle and should be called at application startup.
    pub fn win_sparkle_init() -> c_void;

    /// Check for updates with the WinSparkle UI.
    ///
    /// This checks for updates and displays the WinSparkle UI if an update is available.
    pub fn win_sparkle_check_update_with_ui() -> c_void;
}
