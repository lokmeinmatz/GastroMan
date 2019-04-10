#![feature(duration_float)]

use colored::*;
use ws::listen;

mod database;
use database::DBManager;

mod users;

mod websocket;
use websocket::WSClient;

use std::thread;

fn main() {
    println!("\n\nStarting {}", " GastroMan Server ".green().reversed());
    //load db
    let mut db = DBManager::new().expect("Error while creating DBManager");
    let local_db_query_sender = db.get_query_sender();


    thread::Builder::new().name("Database".to_owned()).spawn(move || {
        db.run();
    }).expect("This system does not support threads.");


    //let clients : Vec<WSClient> = Vec::with_capacity(3);

    listen("192.168.178.30:443", |out| {
        println!("new client connected");
        WSClient::new(out, local_db_query_sender.clone())
    }).unwrap();
    
}
