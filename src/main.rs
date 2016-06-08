extern crate gtk;

use std::sync::{Arc,Mutex};
use gtk::prelude::*;
use gtk::{Window, Builder, Widget, AboutDialog, FileChooserDialog, FileChooserAction};

struct App;
impl App {
    fn new() -> App {
        App
    }
}

struct Ui {
    builder: Builder,
}

impl Ui {
    fn new() -> Ui{
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
        let glade_src = include_str!("ui/main.glade");
        let builder = Builder::new_from_string(glade_src);
        Ui {
            builder : builder,
        }
    }

    fn launch(&self, app : Arc< Mutex< App >> ){
        let window: Window = self.builder.get_object("mainWindow").unwrap();
        let about_dialog: AboutDialog = self.builder.get_object("aboutDialog").unwrap();
        let about_button: Widget = self.builder.get_object("aboutMenuItem").unwrap();
        let open_button: Widget = self.builder.get_object("openMenuItem").unwrap();
        let new_button: Widget = self.builder.get_object("newMenuItem").unwrap();
        let save_button: Widget = self.builder.get_object("saveMenuItem").unwrap();
        let save_as_button: Widget = self.builder.get_object("saveAsMenuItem").unwrap();
        let quit_button: Widget = self.builder.get_object("quitMenuItem").unwrap();

        open_button.connect_button_release_event (move |_b,_e| {
            let file_open = FileChooserDialog::new(Some("Open File"),None::<&Window>,FileChooserAction::Open);
            file_open.run();
            file_open.destroy();
            Inhibit(false)
        });

        about_button.connect_button_release_event (move |_b,_e| {
            about_dialog.run();
            about_dialog.hide();
            Inhibit(false)
        });

        quit_button.connect_button_release_event (move |_b,_e| {
            gtk::main_quit();
            Inhibit(false)
        });
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();
        gtk::main();
    }
}

fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    let ui = Ui::new();
    ui.launch(app);
}
