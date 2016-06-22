extern crate keyru;

use keyru::{App,Ui};
use std::sync::{Arc, Mutex};



fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    let ui = Arc::new(Mutex::new(Ui::new(app)));

    let main_window = {
        let ui_m = ui.clone();
        let mut ui_i = ui_m.lock().unwrap();
        ui_i.prepare_mainwindow()
    };

    Ui::run(main_window);
}
