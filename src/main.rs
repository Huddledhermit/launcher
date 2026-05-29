mod application;
mod apps;
mod config;
pub const APP_ID: &str = "launchy";
fn main() {
    application::build_application()
}
