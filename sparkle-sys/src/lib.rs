// lib.rs
// sparkle-sys

#![cfg(target_os = "macos")]

// Link to the Sparkle.framework
#[cfg_attr(target_os = "macos", link(name = "Sparkle", kind = "framework"))]
extern "C" {}

use objc::{class, msg_send, runtime::Object, sel, sel_impl};

/// A wrapper for the `SPUStandardUpdaterController` class in `Sparkle.framework`.
///
/// Allows you to check for updates in your macOS applications.
pub struct SPUStandardUpdaterController {
    inner: *mut Object,
}

impl SPUStandardUpdaterController {
    /// Create a new `SPUStandardUpdaterController`.
    pub fn new() -> SPUStandardUpdaterController {
        let cls = class!(SPUStandardUpdaterController);

        let inner: *mut Object = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let nil: *mut Object = std::ptr::null_mut();
            let obj: *mut Object = msg_send![obj, initWithStartingUpdater:true updaterDelegate:nil userDriverDelegate:nil];
            obj
        };

        SPUStandardUpdaterController { inner }
    }

    /// Check for updates.
    pub fn check_for_updates(&self) {
        unsafe {
            let nil: *mut Object = std::ptr::null_mut();
            let _: () = msg_send![self.inner, checkForUpdates: nil];
        }
    }
}
