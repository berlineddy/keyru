extern crate gtk;
extern crate keepass;

use keepass::{Database, Node, OpenDBError};
use std::fs::File;
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use gtk::{Window, WindowType, Builder, Widget, AboutDialog, FileChooserDialog, FileChooserAction,
          Menu, MenuItem};

struct App;
impl App {
    fn new() -> App {
        App
    }

    pub fn open_file(&self, path: &str) {
        println!("open file:{:?}", path);
        let db = File::open(std::path::Path::new(path))
            .map_err(|e| OpenDBError::from(e))
            .and_then(|mut db_file| Database::open(&mut db_file, "testtesttest"))
            .unwrap();

        // Iterate over all Groups and Nodes
        for node in &db.root {
            match node {
                Node::Group(g) => {
                    println!("Saw group '{0}'", g.name);
                }
                Node::Entry(e) => {
                    let title = e.get_title().unwrap();
                    let user = e.get_username().unwrap();
                    let pass = e.get_password().unwrap();
                    println!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
                }
            }
        }
    }

    pub fn save_as_file(&self, path: &str) {
        println!("save as file:{:?}", path);
    }

    pub fn save_file(&self) {
        println!("save file");
    }

    pub fn new_file(&self) {
        println!("save file");
    }
}

struct Ui {
    app: Option<Arc<Mutex<App>>>,
    builder: Option<Arc<Mutex<Builder>>>,
    main: Option<Arc<Mutex<Window>>>,
    passwd: Option<Arc<Mutex<Window>>>,
    about: Option<Arc<Mutex<AboutDialog>>>,
}

impl Ui {
    fn new() -> Ui {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
        let glade_src = include_str!("ui/main.glade");
        let builder = Builder::new_from_string(glade_src);
        Ui {
            builder: Some(Arc::new(Mutex::new(builder))),
            main: None,
            about: None,
            app: None,
            passwd: None,
        }
    }

    fn prepare(&mut self, app: Arc<Mutex<App>>) {
    
        let _b = self.builder.as_ref().unwrap();
        let builder = _b.lock().unwrap().clone();
        
        self.app = Some(app);
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
        
        open_button.connect_button_release_event(move |_b, _e| {
            let file_open = FileChooserDialog::new(Some("Open File"),
                                                   Some(&Window::new(WindowType::Popup)),
                                                   FileChooserAction::Save);
            file_open.add_button("Cancel", gtk::ResponseType::Cancel as i32);
            file_open.add_button("Select", gtk::ResponseType::Ok as i32);

            if file_open.run() == gtk::ResponseType::Ok as i32 {
                file_open.get_filename()
                    .map(|path| path.to_str().map(|text| (*app_o.lock().unwrap()).open_file(text)));
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
    }

    pub fn run(&self) {
        let _w = self.main.as_ref().unwrap().clone();
        let ref main = *_w.lock().unwrap();
        main.show_all();
        gtk::main();
    }
}

fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    let mut ui = Arc::new(Mutex::new(Ui::new()));
    let mut ui_m = ui.clone();
    let mut ui_i = ui_m.lock().unwrap();
    ui_i.prepare(app);
    ui_i.run();
}
