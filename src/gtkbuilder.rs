use freedesktop_desktop_entry::DecodeError::AppID;
use gtk4::*;

pub fn mk_interface(app: &Application) {
    let mainwindow = ApplicationWindow::builder().application(app).build();
}
