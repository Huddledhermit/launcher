mod application;
mod apps;
mod config;
mod gtkbuilder;

fn main() {
    application::PROGRAM_DATA.set(apps::load_apps()).unwrap();
    application::build_application();
}
