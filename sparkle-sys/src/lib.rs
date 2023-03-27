// lib.rs
// sparkle-sys

#![cfg(target_os = "macos")]

// Link to the Sparkle.framework
#[cfg_attr(target_os = "macos", link(name = "Sparkle", kind = "framework"))]
extern "C" {}

use objc::{class, msg_send, runtime::Object, sel, sel_impl};

/// Check for updates.
pub unsafe fn sparkle_check_for_updates() {
    let cls = class!(SUUpdater);
    let shared: *mut Object = msg_send![cls, sharedUpdater];

    let nil: *mut Object = std::ptr::null_mut();
    let _: () = msg_send![shared, checkForUpdates: nil];
}
