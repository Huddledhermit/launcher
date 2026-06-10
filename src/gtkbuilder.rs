use freedesktop_desktop_entry::DecodeError::AppID;
use gtk4::prelude::*;
use gtk4::{prelude::GridExt, *};

use crate::{application, apps::program};

pub fn mk_interface(app: &Application) {
    let searchbox = SearchBar::new();

    let app_grid = Grid::new();
    let mut row = 0;
    let mut coul = 0;
    for app in application::PROGRAM_DATA.get().unwrap() {
        let app_str: &str = &app.Name;
        app_grid.attach(&Button::with_label(app_str), coul, row, 1, 1);
        if coul < 5 {
            coul += 1;
        } else {
            coul = 0;
            row += 1
        }
    }

    // assemble final gui configuratiuon
    let mainwindow = ApplicationWindow::builder()
        .application(app)
        .child(&app_grid)
        .build();
    mainwindow.present();
}
