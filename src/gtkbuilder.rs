use freedesktop_desktop_entry::DecodeError::AppID;
use gtk4::prelude::*;
use gtk4::{prelude::GridExt, *};

use crate::apps;
use crate::{application, apps::program};

pub fn mk_interface(app: &Application) {
    let searchbox = SearchBar::new();

    let app_grid = Grid::new();
    let mut row = 0;
    let mut coul = 0;
    let mut current_program: Button;
    for app in application::PROGRAM_DATA.get().unwrap() {
        let app_str: &str = &app.Name;
        current_program = Button::with_label(app_str);
        current_program.connect_clicked(|current_program| apps::on_exec(&app.launch));
        app_grid.attach(&current_program, coul, row, 1, 1);
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
