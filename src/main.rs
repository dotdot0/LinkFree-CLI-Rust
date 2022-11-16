//Modules
mod question;
mod check_user;
mod linkfree_struct;

use question::data;
use check_user::check_user;

#[tokio::main]
async fn main() {
    let username = data();
    let user_status = check_user(&username).await;
    if user_status{
        println!("{username} exist!")
    }
    else{
        println!("User not found!")
    }
}
