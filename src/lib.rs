pub mod hash;

/// Launch the native AdlerIT desktop window.
pub fn run_gui() -> Result<(), String> {
    platform::run_gui()
}

#[cfg(target_os = "windows")]
mod platform {
    mod win32_app;

    pub fn run_gui() -> Result<(), String> {
        win32_app::run()
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    pub fn run_gui() -> Result<(), String> {
        Err("AdlerIT is a native Windows desktop app. Build it on Windows to run the GUI.".into())
    }
}
