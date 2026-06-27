use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=assets/Icon/AdlerIT.ico");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    if env::consts::OS != "windows" {
        println!("cargo:warning=not running on Windows; skipping Windows resource embedding");
        return;
    }

    let Some(rc) = find_resource_compiler() else {
        println!("cargo:warning=rc.exe was not found; skipping Windows resource embedding");
        return;
    };

    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let icon_path = manifest_dir.join("assets").join("Icon").join("AdlerIT.ico");
    let app_manifest_path = out_dir.join("adlerit.exe.manifest");
    let rc_path = out_dir.join("adlerit.rc");
    let res_path = out_dir.join("adlerit.res");

    let icon_path = icon_path.to_string_lossy().replace('\\', "\\\\");
    let app_manifest_rc_path = app_manifest_path.to_string_lossy().replace('\\', "\\\\");
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION");
    let version_tuple = windows_version_tuple(&version);
    fs::write(
        &app_manifest_path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity version="1.0.0.0" processorArchitecture="*" name="AdlerIT" type="win32"/>
  <description>AdlerIT Adler-32 checksum calculator</description>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/PM</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#,
    )
    .expect("failed to write application manifest");

    fs::write(
        &rc_path,
        format!(
            r#"#define CREATEPROCESS_MANIFEST_RESOURCE_ID 1
#define RT_MANIFEST 24

1 ICON "{icon_path}"

CREATEPROCESS_MANIFEST_RESOURCE_ID RT_MANIFEST "{app_manifest_rc_path}"

1 VERSIONINFO
FILEVERSION {version_tuple}
PRODUCTVERSION {version_tuple}
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x0L
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904B0"
        BEGIN
            VALUE "CompanyName", "AdlerIT\0"
            VALUE "FileDescription", "AdlerIT Adler-32 checksum calculator\0"
            VALUE "FileVersion", "{version}\0"
            VALUE "InternalName", "adlerit\0"
            VALUE "OriginalFilename", "AdlerIT-Windows-x64.exe\0"
            VALUE "ProductName", "AdlerIT\0"
            VALUE "ProductVersion", "{version}\0"
            VALUE "LegalCopyright", "Copyright (c) 2026 joshndroid\0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x0409, 1200
    END
END
"#
        ),
    )
    .expect("failed to write rc file");

    let status = Command::new(rc)
        .arg("/nologo")
        .arg(format!("/fo{}", res_path.display()))
        .arg(&rc_path)
        .status()
        .expect("failed to run rc.exe");
    if !status.success() {
        panic!("rc.exe failed with status {status}");
    }

    println!("cargo:rustc-link-arg-bins={}", res_path.display());
}

fn windows_version_tuple(version: &str) -> String {
    let mut parts = version
        .split('.')
        .map(|part| part.parse::<u16>().unwrap_or(0))
        .collect::<Vec<_>>();
    parts.resize(4, 0);
    format!("{},{},{},{}", parts[0], parts[1], parts[2], parts[3])
}

fn find_resource_compiler() -> Option<String> {
    if let Ok(rc) = env::var("RC") {
        return Some(rc);
    }
    if command_exists("rc.exe") {
        return Some("rc.exe".to_owned());
    }

    let program_files_x86 = env::var_os("ProgramFiles(x86)")?;
    let sdk_bin = PathBuf::from(program_files_x86)
        .join("Windows Kits")
        .join("10")
        .join("bin");
    find_rc_in_windows_sdk(&sdk_bin).map(|path| path.to_string_lossy().into_owned())
}

fn command_exists(command: &str) -> bool {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| path.join(command).is_file()))
        .unwrap_or(false)
}

fn find_rc_in_windows_sdk(sdk_bin: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(sdk_bin).ok()?;
    let mut candidates = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("x64").join("rc.exe"))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.pop()
}
