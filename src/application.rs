use crate::{apps, gtkbuilder};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, glib};
pub const APP_ID: &str = "launchy";
pub fn build_application() -> glib::ExitCode {
    let app_list = apps::load_apps();
    let application = gtk4::Application::builder().application_id(APP_ID).build();
    application.connect_activate(gtkbuilder::mk_interface);
    application.run()
}
