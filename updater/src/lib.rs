// lib.rs
// sparkle-updater

#[cfg(target_os = "windows")]
use winsparkle_sys::{
    win_sparkle_check_update_with_ui, win_sparkle_cleanup, win_sparkle_init,
    win_sparkle_set_appcast_url, win_sparkle_set_dsa_pub_pem, win_sparkle_set_registry_path,
    win_sparkle_set_shutdown_request_callback,
};

#[cfg(target_os = "macos")]
use sparkle_sys::SPUStandardUpdaterController;

/// A wrapper for the WinSparkle DLL on Windows and the SPUStandardUpdaterController class in Sparkle.framework on macOS.
///
/// Allows you to add automatic software updates to your Windows and macOS applications.
pub struct Updater {
    #[cfg(target_os = "macos")]
    inner: SPUStandardUpdaterController,
}

impl Updater {
    /// Create a new `Updater` instance on macOS.
    ///
    /// Creates a new `Updater` instance for macOS using the SPUStandardUpdaterController class in Sparkle.framework.
    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        let inner = SPUStandardUpdaterController::new();
        Self { inner }
    }

    /// Create a new `Updater` instance on Windows.
    ///
    /// Creates a new `Updater` instance for Windows using the WinSparkle dynamic-link library (DLL).
    /// Takes the URL for the appcast file, the path in the registry where WinSparkle will store its settings,
    /// and an optional callback function for handling shutdown requests.
    ///
    /// * `appcast_url` - A string slice containing the URL for the appcast file. This specifies the URL where WinSparkle will look for updates.
    ///
    /// * `registry_path` - A string slice containing the path in the registry where WinSparkle will store its settings. This sets the path where WinSparkle will store its settings in the registry.
    ///
    /// * `shutdown_request_callback` - An optional callback function that WinSparkle will call when it receives a request to shut down the application during an update.
    #[cfg(target_os = "windows")]
    pub fn new(
        appcast_url: &str,
        registry_path: &str,
        shutdown_request_callback: Option<extern "C" fn() -> ()>,
    ) -> Self {
        use std::ffi::CString;

        let appcast_url_cstr = CString::new(appcast_url).unwrap();
        let registry_path_cstr = CString::new(registry_path).unwrap();

        unsafe {
            win_sparkle_set_appcast_url(appcast_url_cstr.as_ptr());
            win_sparkle_set_registry_path(registry_path_cstr.as_ptr());
            win_sparkle_set_shutdown_request_callback(shutdown_request_callback);
            win_sparkle_init();
        }

        Self {}
    }

    /// Create a new `Updater` instance on Windows.
    ///
    /// Creates a new `Updater` instance for Windows using the WinSparkle dynamic-link library (DLL).
    /// Takes the URL for the appcast file, the path in the registry where WinSparkle will store its settings,
    /// and an optional callback function for handling shutdown requests.
    ///
    /// * `appcast_url` - A string slice containing the URL for the appcast file. This specifies the URL where WinSparkle will look for updates.
    ///
    /// * `registry_path` - A string slice containing the path in the registry where WinSparkle will store its settings. This sets the path where WinSparkle will store its settings in the registry.
    ///
    /// * `dsa_pub_pem` - A string slice containing the DSA public key. Only PEM format is supported. Public key will be used to verify DSA signatures of the update file.
    ///
    /// * `shutdown_request_callback` - An optional callback function that WinSparkle will call when it receives a request to shut down the application during an update.
    #[cfg(target_os = "windows")]
    pub fn new_with_pubkey(
        appcast_url: &str,
        registry_path: &str,
        dsa_pub_pem: &str,
        shutdown_request_callback: Option<extern "C" fn() -> ()>,
    ) -> Self {
        use std::ffi::CString;

        let appcast_url_cstr = CString::new(appcast_url).unwrap();
        let registry_path_cstr = CString::new(registry_path).unwrap();
        let dsa_pub_pem_cstr = CString::new(dsa_pub_pem).unwrap();

        unsafe {
            win_sparkle_set_appcast_url(appcast_url_cstr.as_ptr());
            win_sparkle_set_registry_path(registry_path_cstr.as_ptr());
            win_sparkle_set_dsa_pub_pem(dsa_pub_pem_cstr.as_ptr());
            win_sparkle_set_shutdown_request_callback(shutdown_request_callback);
            win_sparkle_init();
        }

        Self {}
    }

    /// Check for app updates
    #[cfg(target_os = "macos")]
    pub fn check_for_updates(&self) {
        self.inner.check_for_updates();
    }

    /// Check for app updates
    #[cfg(target_os = "windows")]
    pub fn check_for_updates(&self) {
        unsafe {
            win_sparkle_check_update_with_ui();
        }
    }
}

#[cfg(target_os = "windows")]
impl Drop for Updater {
    fn drop(&mut self) {
        unsafe {
            win_sparkle_cleanup();
        }
    }
}
