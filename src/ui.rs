
extern crate gtk;


use std::sync::{Arc, Mutex};
use self::gtk::prelude::*;
use self::gtk::{Window, WindowType, Builder, Widget, AboutDialog, FileChooserDialog, FileChooserAction,
          Menu, MenuItem};
use App;

pub struct Ui {
    app: Option<Arc<Mutex<App>>>,
    builder: Option<Arc<Mutex<Builder>>>,
    main: Option<Arc<Mutex<Window>>>,
    passwd: Option<Arc<Mutex<Window>>>,
    about: Option<Arc<Mutex<AboutDialog>>>,
}

impl Ui {
    pub fn new(app: Arc<Mutex<App>>) -> Ui {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
        let glade_src = include_str!("ui/main.glade");
        let builder = Builder::new_from_string(glade_src);
        Ui {
            builder: Some(Arc::new(Mutex::new(builder))),
            main: None,
            about: None,
            app: Some(app),
            passwd: None,
        }
    }

    pub fn prepare_mainwindow(&mut self) -> Arc<Mutex<Window>>{

        let _b = self.builder.as_ref().unwrap();
        let builder = _b.lock().unwrap().clone();

        self.main = Some(Arc::new(Mutex::new(builder.get_object("mainWindow").unwrap())));
        self.about = Some(Arc::new(Mutex::new(builder.get_object("aboutDialog").unwrap())));
        self.passwd = Some(Arc::new(Mutex::new(builder.get_object("passwordDialog").unwrap())));

        let _m = self.main.as_ref().unwrap().clone();
        let ref window: Window = *_m.lock().unwrap();

        let new_button: Widget = builder.get_object("newMenuItem").unwrap();
        let save_button: Widget = builder.get_object("saveMenuItem").unwrap();
        let save_as_button: Widget = builder.get_object("saveAsMenuItem").unwrap();
        let quit_button: Widget = builder.get_object("quitMenuItem").unwrap();


        // -------------------------------------------------------
        let open_button: MenuItem = builder.get_object("openMenuItem").unwrap();
        let app_o = self.app.as_ref().unwrap().clone();
        let passwd_o = self.passwd.as_ref().unwrap().clone();

        open_button.connect_button_release_event(move |_b, _e| {
            let file_open = FileChooserDialog::new(Some("Open File"),
                                                   Some(&Window::new(WindowType::Popup)),
                                                   FileChooserAction::Save);
            file_open.add_button("Cancel", gtk::ResponseType::Cancel as i32);
            file_open.add_button("Select", gtk::ResponseType::Ok as i32);

            if file_open.run() == gtk::ResponseType::Ok as i32 {
                file_open.get_filename()
                    .map(|path| path.to_str().map(|text| (*app_o.lock().unwrap()).open_file(text, "testtesttest")));
            }

            file_open.destroy();

            Inhibit(false)
        });
        // -------------------------------------------------------
        let about_button: Widget = builder.get_object("aboutMenuItem").unwrap();
        let about_o = self.about.as_ref().unwrap().clone();

        about_button.connect_button_release_event(move |_b, _e| {
            let ref about = *about_o.lock().unwrap();
            about.run();
            about.hide();
            Inhibit(false)
        });
        // -------------------------------------------------------

        quit_button.connect_button_release_event(move |_b, _e| {
            gtk::main_quit();
            Inhibit(false)
        });
        // -------------------------------------------------------
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        self.main.as_ref().unwrap().clone()
    }

    pub fn open_file(&self) {

    }

    pub fn run(main : Arc<Mutex<Window>>) {
        let _w = main.as_ref().lock().unwrap();
        _w.show_all();
        gtk::main();
    }
}
