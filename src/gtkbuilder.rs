use freedesktop_desktop_entry::DecodeError::AppID;
use gtk4::{prelude::GridExt, *};

use crate::apps::program;

pub fn mk_interface(app: &Application, applist: Vec<program>) {
    let searchbox = SearchBar::new();

    let app_grid = Grid::new();
    let mut r = 0;
    let mut c = 0;
    for app in applist {
        let app_str: &str = &app.Name;
        app_grid.attach(&Button::with_label(app_str), c, r, 1, 1);
        if c < 5 {
            c += 1;
        } else {
            c = 0;
            r += 1
        }
    }

    // assemble final gui configuratiuon
    let mainwindow = ApplicationWindow::builder()
        .application(app)
        .child(&app_grid)
        .build();
}
