extern crate keepass;

use self::keepass::{Database, Node, OpenDBError};
use std::fs::File;
use std::path::Path;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn open_file(&self, path: &str, pass: &str) {
        let mut pass = String::from(pass);
        println!("open file:{:?}", path);
        let db = File::open(Path::new(path))
            .map_err(|e| OpenDBError::from(e))
            .and_then(|mut db_file| Database::open(&mut db_file, &mut pass))
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

