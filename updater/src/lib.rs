// lib.rs
// sparkle-updater

#[cfg(target_os = "macos")]
use sparkle_sys::SPUStandardUpdaterController;

pub struct Updater {
    #[cfg(target_os = "macos")]
    inner: SPUStandardUpdaterController,
}

impl Updater {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        let inner = SPUStandardUpdaterController::new();

        Self { inner }
    }

    pub fn check_for_updates(&self) {
        self.inner.check_for_updates();
    }
}
