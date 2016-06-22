extern crate keyru;

use keyru::{App,Ui};
use std::sync::{Arc, Mutex};



fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    let mut ui = Arc::new(Mutex::new(Ui::new()));
    let mut ui_m = ui.clone();
    let mut ui_i = ui_m.lock().unwrap();
    ui_i.prepare(app);
    ui_i.run();
}
