use freedesktop_desktop_entry::{DesktopEntry, ExecError, desktop_entries};
use gtk4::{
    AccessibleRole::Command,
    glib::{Error, error},
};

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
pub fn on_exec(cmd: &String) -> Option<&str> {
    let mut failstate: Option<&str> = None;
    println!("executed");
    let mut indexed_cmd = cmd.char_indices().peekable();

    while let Some(val) = indexed_cmd.next() {
        if val.1 == '%' {
            if let Some(next) = indexed_cmd.next() {
                match next.1 {
                    'u' => {
                        let finalcmd = cmd.replace("%u", "");
                        let exec_cmd = process::Command::new(finalcmd)
                            .stdin(process::Stdio::null())
                            .stdout(process::Stdio::null())
                            .stderr(process::Stdio::null())
                            .spawn()
                            .expect("could not launch ");
                        failstate = Some("sucess")
                    }
                    'f' => {
                        let finalcmd = cmd.replace("%f", "");
                        let exec_cmd = process::Command::new(finalcmd)
                            .stdin(process::Stdio::null())
                            .stdout(process::Stdio::null())
                            .stderr(process::Stdio::null())
                            .spawn()
                            .expect("could not launch ");
                        failstate = Some("sucess")
                    }
                    'U' => {
                        let finalcmd = cmd.replace("%U", "");
                        let exec_cmd = process::Command::new(finalcmd)
                            .stdin(process::Stdio::null())
                            .stdout(process::Stdio::null())
                            .stderr(process::Stdio::null())
                            .spawn()
                            .expect("could not launch ");
                        failstate = Some("sucess")
                    }
                    _ => panic!(),
                }
            }
        }
    }
    return failstate;
}
pub fn delete() {}
pub fn add_favorite() {}
