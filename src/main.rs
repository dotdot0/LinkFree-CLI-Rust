#![allow(unused)]
//Modules
mod question;
mod check_user;
mod linkfree_struct;
mod create_user;

use create_user::create_user;
use question::{data, get_icons};
use check_user::check_user;
use linkfree_struct::LinkFree;

//std
use std::fs::File;
use std::io::Read;
use std::path::Path;

//External crates
use serde_json;
use user_error::UFE;

#[tokio::main]
async fn main() {
    let data = data().await;
    let (user, username) = data;
    let user_json = create_user(user);
    let path = format!("{username}.json");
    let user_path = Path::new(path.as_str());
    if user_path.exists(){
        println!("User exist")
    }
    else{
        println!(":)")
    }
    // let mut file = File::open(path).expect("Unable to open file");
    // let mut buf: String = String::new();
    // file.read_to_string(&mut buf).unwrap();
    // println!("{buf}")
}

