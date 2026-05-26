use freedesktop_desktop_entry::{DesktopEntry, desktop_entries};
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

fn load_apps() {
let programs: Vec<program>;
    let locales_str = freedesktop_desktop_entry::get_languages_from_env();
    let desktops = desktop_entries(&locales_str);
    for entry in desktops {
        if entry.no_display() == true {
            continue;
        }
        if entry.hidden() == true {
            continue;
        }
        programs += program{}
}

struct program {
    Name : str,
    launch_path: OsString,
    icon: str,
}
