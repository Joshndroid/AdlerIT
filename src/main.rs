#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    match adlerit::run_gui() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let message = format!("AdlerIT could not start:\n\n{error:#}");
            report_startup_error(&message);
            ExitCode::FAILURE
        }
    }
}

#[cfg(target_os = "windows")]
fn report_startup_error(message: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};

    let title: Vec<u16> = "AdlerIT startup error"
        .encode_utf16()
        .chain(Some(0))
        .collect();
    let message: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            message.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn report_startup_error(message: &str) {
    eprintln!("error: {message}");
}
