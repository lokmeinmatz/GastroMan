#![feature(duration_float)]

use colored::*;
use ws::listen;

mod database;
use database::DBManager;

mod users;
use users::User;

mod websocket;
use websocket::WSClient;

use std::thread;

fn main() {
    println!("\n\nStarting {}", " GastroMan Server ".green().reversed());
    //load db
    let mut db = DBManager::new().expect("Error while creating DBManager");
    let local_db_query_sender = db.get_query_sender();


    thread::spawn(move || {
        db.run();
    });


    //let clients : Vec<WSClient> = Vec::with_capacity(3);

    listen("192.168.178.30:443", |out| {
        println!("new client connected");
        WSClient::new(out, local_db_query_sender.clone())
    }).unwrap();
    
}
