use freedesktop_desktop_entry::{DesktopEntry, desktop_entries};
use gtk4::AccessibleRole::Command;
use std::{
    ffi::{OsStr, OsString},
    path::Path,
    process,
};
#[derive(Debug)]
pub struct program {
    pub Name: String,
    pub launch: String,
    pub icon: String,
}

pub fn load_apps() -> Vec<program> {
    let mut programs: Vec<program> = Vec::new();
    let locales = freedesktop_desktop_entry::get_languages_from_env();
    let desktops = desktop_entries(&locales);
    for entry in desktops {
        if entry.no_display() == true {
            continue;
        }
        if entry.hidden() == true {
            continue;
        }
        let name = entry.name(&locales).unwrap().to_string();
        let exec = entry.exec().unwrap().to_string();
        let icon = entry.icon().unwrap().to_string();
        programs.push(program {
            Name: name,
            launch: exec,
            icon: icon,
        })
    }
    return programs;
}
pub fn on_exec(cmd: &String) {
    let exec_cmd = process::Command::new(cmd)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn();
}
pub fn delete_program() {}
pub fn add_or_remove_favorite() {}
