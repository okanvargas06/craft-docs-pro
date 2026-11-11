//! Windows registry read helpers for Craft Docs Pro.

pub const APP_KEY: &str = "Software\\Craft\\Craft Docs Pro";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Craft Docs Pro";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Craft Docs Pro") },
        RegValue { name: "Version",     value: String::from("v3.3.20") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Craft Docs Pro") },
        RegValue { name: "Publisher",       value: String::from("Craft") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Craft Docs Pro\\uninstall.exe\"") },
    ]
}
