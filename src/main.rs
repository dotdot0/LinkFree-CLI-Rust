#![allow(unused)]
//Modules
mod question;
mod check_user;
mod linkfree_struct;
mod create_user;

use question::{data, get_icons};
use check_user::check_user;
use linkfree_struct::LinkFree;
use crate::linkfree_struct::{Links, MileStones, Socials};

//External crates
use serde_json;

#[tokio::main]
async fn main() {
    let answers = data().await;
    println!("{:#?}", answers);
}

